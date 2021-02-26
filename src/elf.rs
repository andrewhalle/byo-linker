use std::collections::HashMap;

pub const ELF_MAGIC: &[u8] = b"\x7FELF";

/*
#[derive(Clone, Debug)]
pub struct ElfFile64SectionHeader {
    pub name: u32,
    pub r#type: u32,
    pub flags: u64,
    pub addr: usize,
    pub offset: usize,
    pub size: usize,
    pub link: u32,
    pub info: u32,
    pub addr_align: u64,
    pub entsize: usize,
}

#[derive(Clone, Debug)]
pub struct ElfFile64Section {
    pub header: ElfFile64SectionHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ElfFile64ProgramHeader;

#[derive(Default, Debug)]
pub struct ElfFile64 {
    pub filename: Option<String>,
    pub header: ElfFile64Header,
    pub program_headers: Vec<ElfFile64ProgramHeader>,
    pub sections: Vec<ElfFile64Section>,
}

/*
impl ElfFile64 {
    pub fn get_section_name(&self, idx: usize) -> String {
        let section_names = &self.sections[self.header.shstrndx as usize];
        let section = &self.sections[idx];
        unsafe {
            std::ffi::CStr::from_ptr(
                &section_names.data[section.header.name as usize] as *const u8
                    as *const std::os::raw::c_char,
            )
            .to_str()
            .expect("could not get string name from string table")
            .to_string()
        }
    }

    pub fn get_string(&self, string_table_index: usize, string_idx: usize) -> String {
        unsafe {
            std::ffi::CStr::from_ptr(
                &self.sections[string_table_index].data[string_idx] as *const u8
                    as *const std::os::raw::c_char,
            )
            .to_str()
            .expect("could not get string from string table")
            .to_string()
        }
    }

    pub fn get_symbol_name(&self, idx: usize) -> String {
        let symbol_names = self.string_table();
        unsafe {
            std::ffi::CStr::from_ptr(
                &symbol_names.data[idx] as *const u8 as *const std::os::raw::c_char,
            )
            .to_str()
            .expect("could not get string name from string table")
            .to_string()
        }
    }

    pub fn symbol_table(&self) -> &ElfFile64Section {
        let mut sections = self.sections.iter().filter(|s| s.header.r#type == 0x2);

        // ensure there's only one symbol table
        let section = sections.next().expect("no symbol table sections");
        assert!(sections.next().is_none());

        section
    }

    pub fn string_table(&self) -> &ElfFile64Section {
        let sections: Vec<(usize, &ElfFile64Section)> = self
            .sections
            .iter()
            .enumerate()
            .filter(|(_, s)| s.header.r#type == 0x3)
            .collect();

        if sections.len() == 1 {
            sections[0].1
        } else {
            sections
                .iter()
                .find(|(idx, _)| *idx != self.header.shstrndx as usize)
                .expect("no string table that is not the section name table")
                .1
        }
    }

    pub fn section_name_map(&self) -> HashMap<String, usize> {
        let mut result = HashMap::new();

        for (idx, section) in self.sections.iter().enumerate() {
            result.insert(
                self.get_string(self.header.shstrndx as usize, section.header.name as usize),
                idx,
            );
        }

        result
    }
}

impl std::fmt::Display for ElfFile64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "ElfFile:")?;
        writeln!(f, "Size of section headers: {}", self.header.shnum)?;
        writeln!(f, "Section header name index: {}\n", self.header.shstrndx)?;

        for section in self.sections.iter() {
            writeln!(
                f,
                "ElfFile Section: {}",
                self.get_string(self.header.shstrndx as usize, section.header.name as usize),
            )?
        }

        Ok(())
    }
}
*/
*/
