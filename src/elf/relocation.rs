use super::parse::ElfFile64Raw;
use super::section::Section64;

#[derive(Debug)]
pub struct RelocationA64 {
    pub offset: u64,
    pub info: u64,
    pub addend: u64,
}

pub fn get_relocations(raw: &ElfFile64Raw, rela: &Section64) -> Vec<RelocationA64> {
    RelocationA64::parse_many(&rela.data[..], raw.header.identifier.endianness)
        .expect("could not get symbols")
}
