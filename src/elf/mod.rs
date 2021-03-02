mod parse;
mod relocation;
mod section;
mod symbol;

use parse::{ElfFile64Raw, ElfFile64RawParseError};
use relocation::RelocationTable;
use section::Section64;
use symbol::SymbolTable;

pub const ELF_MAGIC: &[u8] = b"\x7FELF";

#[derive(Debug)]
pub struct ElfFile64 {
    unorganized_sections: Vec<Section64>,
    symbols: SymbolTable,
    relocations: RelocationTable,
}

#[derive(Debug)]
pub enum ElfFileError {
    ParseError,
    InvalidFileError,
}

impl From<ElfFile64RawParseError> for ElfFileError {
    fn from(_: ElfFile64RawParseError) -> Self {
        ElfFileError::ParseError
    }
}

impl ElfFile64 {
    pub fn parse(input: &[u8]) -> Result<ElfFile64, ElfFileError> {
        Ok(ElfFile64Raw::parse(input)?.into())
    }
}

impl From<ElfFile64Raw> for ElfFile64 {
    fn from(raw: ElfFile64Raw) -> Self {
        let get_name = |idx: usize| {
            let section_name_string_table_header =
                &raw.section_headers[raw.header.shstrndx as usize];
            let offset =
                (section_name_string_table_header.offset - raw.header.ehsize as u64) as usize;
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
                r#type: section_header.r#type,
                flags: section_header.flags,
                addr: section_header.addr,
                link: section_header.link,
                info: section_header.info,
                addralign: section_header.addralign,
                data: get_data(section_header.offset as usize, section_header.size as usize),
            });
        }

        ElfFile64 {
            unorganized_sections: sections,
            symbols: SymbolTable,
            relocations: RelocationTable,
        }
    }
}
