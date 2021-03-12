use crate::elf::program_header::ProgramHeader64;
use crate::elf::ElfFile64;

impl ElfFile64 {
    pub fn relocate(&mut self) {
        self.apply_relocations();
        self.add_program_headers();
        self.resize_sections();
    }

    fn apply_relocations(&mut self) {
        // get text section
        // replace address
    }

    fn resize_sections(&mut self) {}

    fn add_program_headers(&mut self) {
        self.program_headers.push(ProgramHeader64 {
            r#type: 1,
            flags: 0b100,
            offset: 0,
            vaddr: 0x400000,
            paddr: 0x400000,
            filesz: 0xe8,
            memsz: 0xe8,
            align: 0x1000,
        });
        self.program_headers.push(ProgramHeader64 {
            r#type: 1,
            flags: 0b101,
            offset: 0x1000,
            vaddr: 0x401000,
            paddr: 0x401000,
            filesz: 0x23,
            memsz: 0x23,
            align: 0x1000,
        });
        self.program_headers.push(ProgramHeader64 {
            r#type: 1,
            flags: 0b110,
            offset: 0x2000,
            vaddr: 0x402000,
            paddr: 0x402000,
            filesz: 0xc,
            memsz: 0xc,
            align: 0x1000,
        });
    }
}
