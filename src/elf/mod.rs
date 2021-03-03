mod parse;
mod relocation;
mod section;
mod symbol;

use parse::{ElfFile64Raw, ElfFile64RawParseError};
use relocation::Relocation64;
use section::{get_sections, Section64};
use symbol::{get_symbols, Symbol64};

pub const ELF_MAGIC: &[u8] = b"\x7FELF";

#[derive(Debug)]
pub struct ElfFile64 {
    pub unorganized_sections: Vec<Section64>,
    pub symbols: Vec<Symbol64>,
    pub relocations: Vec<Relocation64>,
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
        let symbols = get_symbols(&raw, &sections);

        ElfFile64 {
            unorganized_sections: sections,
            symbols,
            relocations: Vec::new(),
        }
    }
}
