use super::parse::ElfFile64Raw;
use super::section::Section64;

#[derive(Clone, Debug)]
pub struct RelocationA64 {
    pub offset: u64,
    pub info: u64,
    pub addend: u64,
    pub merged: bool, // XXX remove me
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

    pub fn get_sym(&self) -> usize {
        (self.info >> 32) as usize
    }

    pub fn get_type(&self) -> usize {
        (self.info & 0xffffffff) as usize
    }

    pub fn set_info(&mut self, sym: usize, r#type: usize) {
        self.info = ((sym << 32) + (r#type)) as u64;
    }
}

pub fn get_relocations(raw: &ElfFile64Raw, rela: &Section64) -> Vec<RelocationA64> {
    RelocationA64::parse_many(&rela.data[..], raw.header.identifier.endianness)
        .expect("could not get symbols")
}
