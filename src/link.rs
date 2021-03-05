use crate::elf::section::Section64;
use crate::elf::symbol::{sym_bind, Symbol64};
use crate::elf::ElfFile64;
use crate::utils;

impl ElfFile64 {
    fn link(&mut self, other: &ElfFile64) {
        let mut section_name_map = utils::build_section_name_map(self);
        let mut symbol_map = utils::build_symbol_name_map(self);

        for section in other.unorganized_sections.iter() {
            if section_name_map.contains_key(&section.name) {
                let existing_idx = section_name_map.get(&section.name).unwrap();
                let existing = &mut self.unorganized_sections[*existing_idx];
                existing.merge(&section);
            } else {
                section_name_map.insert(section.name.clone(), self.unorganized_sections.len());
                self.unorganized_sections.push(section.clone());
            }
        }

        for symbol in other.symbols.iter() {
            if symbol.name == "" {
                continue;
            }

            if symbol_map.contains_key(&symbol.name) {
                let existing_idx = symbol_map.get(&symbol.name).unwrap();
                let existing = &mut self.symbols[*existing_idx];
                if existing.shndx != 0 && symbol.shndx != 0 {
                    panic!("symbol defined multiple times");
                } else if existing.shndx == 0 && symbol.shndx != 0 {
                    *existing = symbol.clone();
                }
            } else {
                symbol_map.insert(symbol.name.clone(), self.symbols.len());
                self.symbols.push(symbol.clone());
            }
        }

        let symbols = std::mem::take(&mut self.symbols);
        let (mut local, mut nonlocal): (Vec<Symbol64>, _) =
            symbols.into_iter().partition(|s| sym_bind(s) == 0);
        local.append(&mut nonlocal);
        std::mem::swap(&mut self.symbols, &mut local);
    }
}

pub fn link(mut object_files: Vec<ElfFile64>) -> ElfFile64 {
    let mut result = object_files.remove(0);

    for object_file in object_files.iter() {
        result.link(object_file);
    }

    result
}

impl Section64 {
    pub fn merge(&mut self, other: &Section64) {
        // TODO: can alignment be different?
        if self.addralign != other.addralign {
            panic!(
                "{}: cannot merge sections with different alignment",
                &other.name
            );
        }

        // TODO: can type be different?
        if self.r#type != other.r#type {
            panic!("{}: cannot merge sections with different type", &other.name);
        }

        // TODO: can flags be different?
        if self.flags != other.flags {
            panic!(
                "{}: cannot merge sections with different flags",
                &other.name
            );
        }

        // TODO: can addr be different?
        if self.addr != other.addr {
            panic!("{}: cannot merge sections with different addr", &other.name);
        }

        // append data to existing section after padding
        let align = self.addralign as usize;
        let existing_len = self.data.len();
        if existing_len.trailing_zeros() != align.trailing_zeros() {
            let new_len = utils::next_aligned_value(existing_len, align);
            self.data.resize(new_len, 0xff);
            self.data.append(&mut other.data.clone());
        }

        match &other.relocations {
            None => {}
            Some(relas) => {
                if self.relocations.is_none() {
                    self.relocations = Some(Vec::new());
                }

                let existing = self.relocations.as_mut().unwrap();
                for relocation in relas {
                    existing.push(relocation.clone());
                }
            }
        }
    }
}
