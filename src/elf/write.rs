use std::collections::HashMap;
use std::io;

use super::section::Section64;
use super::{ElfFile64, EHSIZE_64, ELF_MAGIC};

impl ElfFile64 {
    pub fn write_out<W: io::Write>(file: ElfFile64, output: W) -> io::Result<()> {
        use nom::number::Endianness::*;

        match file.header.identifier.endianness {
            Big => ElfFile64::write_out_endian::<_, byteorder::BigEndian>(file, output),
            Little => ElfFile64::write_out_endian::<_, byteorder::LittleEndian>(file, output),
            _ => unreachable!(),
        }
    }

    fn write_out_endian<W: io::Write, T: byteorder::ByteOrder>(
        mut file: ElfFile64,
        mut output: W,
    ) -> io::Result<()> {
        let mut num_relas = 0;
        // collect data needed to build string tables
        let symbol_names: Vec<String> = file.symbols.iter().map(|s| s.name.clone()).collect();
        let mut section_names: Vec<String> = file
            .unorganized_sections
            .iter()
            .map(|s| s.name.clone())
            .collect();
        section_names.push(".strtab".to_string());
        section_names.push(".shstrtab".to_string());
        section_names.push(".symtab".to_string());
        for section in file.unorganized_sections.iter() {
            if section.relocations.is_some() {
                let rela_name = format!(".rela{}", &section.name);
                section_names.push(rela_name);
                num_relas += 1;
            }
        }

        let strtab = build_string_table(symbol_names);
        let shstrtab = build_string_table(section_names);

        // re-construct sections that were abstracted
        let mut relas = Vec::new();
        for (idx, section) in file.unorganized_sections.iter().enumerate() {
            if let Some(ref rs) = section.relocations {
                let symtab_index = file.unorganized_sections.len() + num_relas;
                let name = format!(".rela{}", &section.name);
                relas.push(Section64::from_rela::<T>(rs, symtab_index, idx, name));
            }
        }

        let symtab = Section64::from_symtab::<T>(
            &file.symbols,
            &strtab,
            // will be index of .strtab where symbol names will be held
            file.unorganized_sections.len() + num_relas + 1,
        );

        // put all sections together
        let mut sections = Vec::new();
        sections.append(&mut file.unorganized_sections);
        sections.append(&mut relas);
        sections.push(symtab);
        sections.push(Section64::from_strtab(&strtab, ".strtab".to_string()));
        sections.push(Section64::from_strtab(&shstrtab, ".shstrtab".to_string()));

        // write file
        file.write_header::<_, T>(&mut output, &sections)?;
        file.write_program_headers::<_, T>(&mut output);
        file.write_section_data(&mut output, &sections)?;
        file.write_section_headers::<_, T>(&mut output, &sections, &shstrtab)?;

        Ok(())
    }

    fn write_header<W: io::Write, T: byteorder::ByteOrder>(
        &self,
        output: &mut W,
        sections: &Vec<Section64>,
    ) -> io::Result<()> {
        use byteorder::WriteBytesExt;

        // write ELF identifier
        output.write(ELF_MAGIC)?;
        output.write(&[
            2_u8, // 64-bit object
            serialize_endianness(self.header.identifier.endianness),
            self.header.identifier.version,
            self.header.identifier.os_abi,
            self.header.identifier.abi_version,
        ])?;
        output.write(&[0; 7])?;

        // write rest of ELF header
        output.write_u16::<T>(self.header.r#type)?;
        output.write_u16::<T>(self.header.machine)?;
        output.write_u32::<T>(self.header.version)?;
        // XXX default entry address is top of .text?
        // 0x400000 base address because that's size of kernel reserved space?
        output.write_u64::<T>(0x401000)?;
        output.write_u64::<T>(EHSIZE_64 as u64)?;
        let shoff = get_section_data_size(sections) as u64;
        output.write_u64::<T>(EHSIZE_64 as u64 + shoff)?;
        output.write_u32::<T>(self.header.flags)?;
        output.write_u16::<T>(EHSIZE_64 as u16)?;
        output.write_u16::<T>(self.header.phentsize)?;
        output.write_u16::<T>(self.program_headers.len() as u16)?;
        output.write_u16::<T>(self.header.shentsize)?;
        output.write_u16::<T>(sections.len() as u16)?;
        output.write_u16::<T>((sections.len() - 1) as u16)?;

        Ok(())
    }

    fn write_program_headers<W: io::Write, T: byteorder::ByteOrder>(
        &self,
        output: &mut W,
    ) -> io::Result<()> {
        Ok(())
    }

    fn write_section_data<W: io::Write>(
        &self,
        output: &mut W,
        sections: &Vec<Section64>,
    ) -> io::Result<()> {
        for section in sections {
            output.write(&section.data[..])?;
        }

        Ok(())
    }

    fn write_section_headers<W: io::Write, T: byteorder::ByteOrder>(
        &self,
        output: &mut W,
        sections: &Vec<Section64>,
        shstrtab: &HashMap<String, usize>,
    ) -> io::Result<()> {
        use super::section::SectionType64::*;
        use byteorder::WriteBytesExt;

        let mut offset = EHSIZE_64;
        for section in sections {
            let name = *shstrtab
                .get(&section.name)
                .expect("could not find section name") as u32;

            output.write_u32::<T>(name)?;
            output.write_u32::<T>(section.r#type.into())?;
            output.write_u64::<T>(section.flags)?;
            output.write_u64::<T>(section.addr)?;
            output.write_u64::<T>(offset as u64)?;
            output.write_u64::<T>(section.data.len() as u64)?;
            output.write_u32::<T>(section.link)?;
            output.write_u32::<T>(section.info)?;
            output.write_u64::<T>(section.addralign)?;
            let entsize = match section.r#type {
                Symtab => 24,
                Rela => 24,
                _ => 0,
            };
            output.write_u64::<T>(entsize)?;

            offset += section.data.len();
        }

        Ok(())
    }
}

fn build_string_table(strings: Vec<String>) -> HashMap<String, usize> {
    let mut retval = HashMap::new();
    retval.insert("".to_string(), 0);

    let mut offset = 1;
    for string in strings {
        if retval.contains_key(&string) {
            continue;
        } else {
            let len = string.len();
            retval.insert(string, offset);
            offset += len + 1;
        }
    }

    retval
}

fn serialize_endianness(e: nom::number::Endianness) -> u8 {
    use nom::number::Endianness::*;

    match e {
        Little => 0x1,
        Big => 0x2,
        _ => unreachable!(),
    }
}

fn get_section_data_size(sections: &Vec<Section64>) -> usize {
    let mut size = 0;
    for section in sections {
        size += section.data.len();
    }

    size
}
