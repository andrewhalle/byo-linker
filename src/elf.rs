#[derive(Debug)]
pub struct ElfFile64Header {
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub r#type: u16,
    pub machine: u16,
    pub e_version: u32,
    pub entry: usize,
    pub phoff: usize,
    pub shoff: usize,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[derive(Debug)]
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

#[derive(Debug)]
pub struct ElfFile64Section {
    pub header: ElfFile64SectionHeader,
    pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct ElfFile64ProgramHeader;

#[derive(Debug)]
pub struct ElfFile64 {
    pub header: ElfFile64Header,
    pub program_headers: Vec<ElfFile64ProgramHeader>,
    pub sections: Vec<ElfFile64Section>,
}

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
}
