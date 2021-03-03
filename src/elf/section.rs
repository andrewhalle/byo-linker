use super::parse::ElfFile64Raw;

#[derive(Debug, PartialEq)]
pub enum SectionType64 {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    UnwindX64,
    Loos,
}

impl From<u32> for SectionType64 {
    fn from(x: u32) -> Self {
        use SectionType64::*;

        match x {
            0 => Null,
            1 => Progbits,
            2 => Symtab,
            3 => Strtab,
            4 => Rela,
            1879048193 => UnwindX64,
            1879002115 => Loos,
            _ => unimplemented!(),
        }
    }
}

impl From<SectionType64> for u32 {
    fn from(x: SectionType64) -> Self {
        use SectionType64::*;

        match x {
            Null => 0,
            Progbits => 1,
            Symtab => 2,
            Strtab => 3,
            Rela => 4,
            UnwindX64 => 1879048193,
            Loos => 1879002115,
        }
    }
}

#[derive(Debug)]
pub struct Section64 {
    pub name: String,
    pub r#type: SectionType64,
    pub flags: u64,
    pub addr: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub data: Vec<u8>,
}

pub fn get_sections(raw: &ElfFile64Raw) -> Vec<Section64> {
    let get_name = |idx: usize| {
        let section_name_string_table_header = &raw.section_headers[raw.header.shstrndx as usize];
        let offset = (section_name_string_table_header.offset - raw.header.ehsize as u64) as usize;
        let start_of_name = &raw.section_data[offset + idx] as *const u8 as *const i8;
        let name = unsafe { std::ffi::CStr::from_ptr(start_of_name) };

        name.to_str().expect("could not create a &str").to_string()
    };

    let get_data = |offset: usize, size: usize| {
        if offset == 0 && size == 0 {
            return Vec::new();
        }

        let begin = offset - raw.header.ehsize as usize;
        let end = begin + size as usize;
        let data = &raw.section_data[begin..end];

        data.to_owned()
    };

    let mut sections = Vec::new();
    for section_header in raw.section_headers.iter() {
        sections.push(Section64 {
            name: get_name(section_header.name as usize),
            r#type: section_header.r#type.into(),
            flags: section_header.flags,
            addr: section_header.addr,
            link: section_header.link,
            info: section_header.info,
            addralign: section_header.addralign,
            data: get_data(section_header.offset as usize, section_header.size as usize),
        });
    }

    sections
}
