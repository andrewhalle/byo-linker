mod parse;
mod relocation;
pub mod section;
pub mod symbol;
mod write;

use parse::{ElfFile64HeaderRaw, ElfFile64Raw, ElfFile64RawParseError};
use relocation::get_relocations;
use section::{get_sections, organize_sections, Section64};
use symbol::{get_symbols, Symbol64};

pub const ELF_MAGIC: &[u8] = b"\x7FELF";
pub const EHSIZE_64: usize = 64;

#[derive(Debug)]
pub struct ElfFile64 {
    pub header: ElfFile64HeaderRaw,
    pub unorganized_sections: Vec<Section64>,
    pub symbols: Vec<Symbol64>,
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
        let sections = get_sections(&raw);

        let (mut unorganized_sections, symtab, relas, index_map) = organize_sections(sections);

        let symbols = get_symbols(&raw, &symtab, &index_map);
        for rela in relas {
            let referenced_section = &mut unorganized_sections[rela.info as usize];
            referenced_section.relocations = Some(get_relocations(&raw, &rela));
        }

        ElfFile64 {
            header: raw.header,
            unorganized_sections,
            symbols,
        }
    }
}
