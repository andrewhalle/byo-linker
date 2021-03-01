pub const ELF_MAGIC: &[u8] = b"\x7FELF";

#[derive(Debug)]
pub struct ElfFile64;

#[derive(Debug)]
pub enum ElfFileError {
    ParseError,
}
