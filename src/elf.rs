use crate::parse::{ElfFile64Raw, ElfFile64RawParseError};

pub const ELF_MAGIC: &[u8] = b"\x7FELF";

#[derive(Debug)]
pub struct ElfFile64;

#[derive(Debug)]
pub enum ElfFileError {
    ParseError,
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
        println!("{:#?}", raw);
        ElfFile64
    }
}
