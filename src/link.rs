use crate::elf::ElfFile64;

pub fn link(_object_files: Vec<ElfFile64>) -> ElfFile64 {
    ElfFile64::default()
}
