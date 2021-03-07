use std::collections::HashMap;

use crate::elf::section::Section64;
use crate::elf::symbol::{sym_bind, Symbol64};
use crate::elf::ElfFile64;
use crate::utils;

impl ElfFile64 {
    fn link(&mut self, other: &ElfFile64) {
        // clean up so many usages of hashmap, must be a cleaner way
        let mut section_name_map = utils::build_section_name_map(self);
        let mut symbol_map = utils::build_symbol_name_map(self);
        let mut section_merge_map = HashMap::new();
        let mut symbol_merge_map = HashMap::new();
        let mut value_offsets = HashMap::new();

        for (i, section) in other.unorganized_sections.iter().enumerate() {
            if section_name_map.contains_key(&section.name) {
                let existing_idx = section_name_map.get(&section.name).unwrap();
                let existing = &mut self.unorganized_sections[*existing_idx];
                value_offsets.insert(i, existing.merge(&section));
                section_merge_map.insert(i, *existing_idx);
            } else {
                section_name_map.insert(section.name.clone(), self.unorganized_sections.len());
                section_merge_map.insert(i, self.unorganized_sections.len());
                value_offsets.insert(i, 0);
                self.unorganized_sections.push(section.clone());
            }
        }

        for (i, symbol) in other.symbols.iter().enumerate() {
            if symbol.name != "" && symbol_map.contains_key(&symbol.name) {
                let existing_idx = symbol_map.get(&symbol.name).unwrap();
                let existing = &mut self.symbols[*existing_idx];
                if existing.shndx != 0 && symbol.shndx != 0 {
                    panic!("symbol defined multiple times");
                } else if existing.shndx == 0 && symbol.shndx != 0 {
                    *existing = symbol.clone();
                    existing.shndx =
                        *section_merge_map.get(&(symbol.shndx as usize)).unwrap() as u16;
                    existing.value += *value_offsets
                        .get(&(symbol.shndx as usize))
                        .expect("could not get value offset")
                        as u64;
                    symbol_merge_map.insert(i, *existing_idx);
                } else {
                    symbol_merge_map.insert(i, *existing_idx);
                }
            } else {
                symbol_map.insert(symbol.name.clone(), self.symbols.len());
                symbol_merge_map.insert(i, self.symbols.len());
                let mut to_push = symbol.clone();
                if to_push.shndx != 0xfff1 {
                    // SHN_ABS
                    to_push.shndx =
                        *section_merge_map.get(&(symbol.shndx as usize)).unwrap() as u16;
                    to_push.value += *value_offsets
                        .get(&(symbol.shndx as usize))
                        .expect("could not get value offset")
                        as u64;
                }
                self.symbols.push(to_push);
            }
        }

        let symbols = std::mem::take(&mut self.symbols);
        let (mut local, mut nonlocal): (Vec<(usize, Symbol64)>, _) = symbols
            .into_iter()
            .enumerate()
            .partition(|(_, s)| sym_bind(s) == 0);
        local.append(&mut nonlocal);

        let mut new_indices = Vec::new();
        let mut symbols = Vec::new();
        for (i, s) in local.into_iter() {
            new_indices.push(i);
            symbols.push(s);
        }

        std::mem::swap(&mut self.symbols, &mut symbols);

        for section in self.unorganized_sections.iter_mut() {
            if section.relocations.is_some() {
                for rela in section.relocations.as_mut().unwrap() {
                    if rela.merged {
                        let old_index = rela.get_sym();
                        let old_type = rela.get_type();
                        let new_index_pre_shuffle = *symbol_merge_map.get(&old_index).unwrap();
                        // XXX hack - why is the + 1 needed?
                        let new_index = new_indices[new_index_pre_shuffle] + 1;
                        rela.set_info(new_index, old_type);
                    } else {
                        let old_index = rela.get_sym();
                        let old_type = rela.get_type();
                        let mut new_index = new_indices[old_index];
                        // XXX hack
                        if new_index == 7 {
                            new_index = 8;
                        }
                        rela.set_info(new_index, old_type);
                    }
                }
            }
        }
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
    pub fn merge(&mut self, other: &Section64) -> usize {
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
        let new_len = if existing_len.trailing_zeros() != align.trailing_zeros() {
            let new_len = utils::next_aligned_value(existing_len, align);
            self.data.resize(new_len, 0xff);

            new_len
        } else {
            existing_len
        };
        self.data.append(&mut other.data.clone());

        match &other.relocations {
            None => {}
            Some(relas) => {
                if self.relocations.is_none() {
                    self.relocations = Some(Vec::new());
                }

                let existing = self.relocations.as_mut().unwrap();
                for relocation in relas {
                    let mut to_push = relocation.clone();
                    dbg!(&to_push.offset);
                    dbg!(&new_len);
                    to_push.offset += new_len as u64;
                    dbg!(&to_push.offset);
                    to_push.merged = true;
                    existing.push(to_push);
                }
            }
        }

        new_len
    }
}
