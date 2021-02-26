/*
use std::collections::HashMap;

use crate::elf::{ElfFile64, ElfFile64Section};
use crate::utils;

impl ElfFile64 {
    fn link_symbols(&mut self) {}

    fn link_relocations(&mut self) {}

    fn merge_section(
        &mut self,
        section: &ElfFile64Section,
        other_name: String,
        existing: &HashMap<String, usize>,
        original_strtab_size: u32,
    ) {
        if !existing.contains_key(&other_name) {
            let mut new_section = section.clone();
            new_section.header.name += original_strtab_size as u32;
            self.sections.push(new_section);
            self.header.shnum += 1;
        } else {
            let existing_section = &mut self.sections[*existing.get(&other_name).unwrap()];

            // TODO: can alignment be different?
            if existing_section.header.addr_align != section.header.addr_align {
                panic!(
                    "{}: cannot merge sections with different alignment",
                    &other_name
                );
            }

            // TODO: can type be different?
            if existing_section.header.r#type != section.header.r#type {
                panic!("{}: cannot merge sections with different type", &other_name);
            }

            // TODO: can flags be different?
            if existing_section.header.flags != section.header.flags {
                panic!(
                    "{}: cannot merge sections with different flags",
                    &other_name
                );
            }

            // TODO: can addr be different?
            if existing_section.header.addr != section.header.addr {
                panic!("{}: cannot merge sections with different addr", &other_name);
            }

            // TODO: can entsize be different?
            if existing_section.header.entsize != section.header.entsize {
                panic!(
                    "{}: cannot merge sections with different entsize",
                    &other_name
                );
            }

            // append data to existing section after padding
            let align = existing_section.header.addr_align as usize;
            let existing_len = existing_section.data.len() as usize;
            if existing_len.trailing_zeros() != align.trailing_zeros() {
                let new_len = utils::next_aligned_value(existing_len, align);
                existing_section.data.resize(new_len, 0xff);
                existing_section.data.append(&mut section.data.clone());
            }

            existing_section.header.size = existing_section.data.len();
        }
    }

    fn link(&mut self, other: &ElfFile64) {
        let existing = self.section_name_map();

        let original_strtab_size = {
            let strtab = &self.sections[*existing.get(".strtab").unwrap()];

            utils::next_aligned_value(strtab.header.addr_align as usize, strtab.data.len())
        };

        for section in other.sections.iter() {
            let other_name =
                other.get_string(other.header.shstrndx as usize, section.header.name as usize);

            self.merge_section(section, other_name, &existing, original_strtab_size as u32);
        }

        self.link_symbols();
        self.link_relocations();
    }
}

pub fn link(mut object_files: Vec<ElfFile64>) -> ElfFile64 {
    let mut result = object_files.pop().expect("need object file but not found");

    for object_file in object_files.iter() {
        result.link(object_file);
    }

    result
}
*/
