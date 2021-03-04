use super::parse::ElfFile64Raw;
use super::section::Section64;

#[derive(Debug)]
pub struct RelocationA64 {
    pub offset: u64,
    pub info: u64,
    pub addend: u64,
}

impl RelocationA64 {
    pub fn as_raw<T: byteorder::ByteOrder>(&self) -> Vec<u8> {
        use byteorder::WriteBytesExt;

        let mut retval = Vec::new();
        retval.write_u64::<T>(self.offset).expect("could not write");
        retval.write_u64::<T>(self.info).expect("could not write");
        retval.write_u64::<T>(self.addend).expect("could not write");

        retval
    }
}

pub fn get_relocations(raw: &ElfFile64Raw, rela: &Section64) -> Vec<RelocationA64> {
    RelocationA64::parse_many(&rela.data[..], raw.header.identifier.endianness)
        .expect("could not get symbols")
}
