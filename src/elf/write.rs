use std::collections::HashMap;
use std::io;

use super::section::Section64;
use super::{ElfFile64, ELF_MAGIC};

impl ElfFile64 {
    pub fn write_out<W: io::Write>(&self, mut output: W) -> io::Result<()> {
        // collect data needed to build string tables
        let symbol_names: Vec<String> = self.symbols.iter().map(|s| s.name.clone()).collect();
        let mut section_names: Vec<String> = self.symbols.iter().map(|s| s.name.clone()).collect();
        section_names.push(".strtab".to_string());
        section_names.push(".shstrtab".to_string());
        section_names.push(".symtab".to_string());
        for section in self.unorganized_sections.iter() {
            if section.relocations.is_some() {
                let rela_name = format!(".rela{}", &section.name);
                section_names.push(rela_name);
            }
        }

        let strtab = build_string_table(symbol_names);
        let shstrtab = build_string_table(section_names);

        // XXX
        // re-construct sections that were abstracted
        let symtab = ();
        let relas = ();

        // put all sections together
        let sections = vec![
            Section64::from_hashmap(strtab, ".strtab".to_string()),
            Section64::from_hashmap(shstrtab, ".shstrtab".to_string()),
        ];

        // write file
        self.write_header(&mut output)?;
        self.write_section_data(&mut output, &sections)?;
        self.write_section_headers(&mut output, &sections)?;

        Ok(())
    }

    fn write_header<W: io::Write>(&self, output: &mut W) -> io::Result<()> {
        output.write(ELF_MAGIC)?;

        Ok(())
    }

    fn write_section_data<W: io::Write>(
        &self,
        _output: &mut W,
        _sections: &Vec<Section64>,
    ) -> io::Result<()> {
        Ok(())
    }

    fn write_section_headers<W: io::Write>(
        &self,
        _output: &mut W,
        _sections: &Vec<Section64>,
    ) -> io::Result<()> {
        Ok(())
    }
}

fn build_string_table(strings: Vec<String>) -> HashMap<String, usize> {
    let mut retval = HashMap::new();
    let mut offset = 1;
    for string in strings {
        if string == "" || retval.contains_key(&string) {
            continue;
        } else {
            offset += string.len() + 1;
            retval.insert(string, offset);
        }
    }

    retval
}
