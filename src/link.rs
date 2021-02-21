use crate::elf::ElfFile64;

impl ElfFile64 {
    fn link(&mut self, other: &ElfFile64) {
        let existing = self.section_name_map();

        for section in other.sections.iter() {
            let other_name =
                other.get_string(other.header.shstrndx as usize, section.header.name as usize);

            if !existing.contains_key(&other_name) {
                self.sections.push(section.clone());
                self.header.shnum += 1;
            } else {
                let existing_section = &mut self.sections[*existing.get(&other_name).unwrap()];

                // TODO: can alignment be different?
                if existing_section.header.addr_align != section.header.addr_align {
                    panic!("cannot merge sections with different alignment");
                }

                // append data to existing section after padding
                let align = existing_section.header.addr_align as usize;
                if existing_section.data.len().trailing_zeros() != align.trailing_zeros() {
                    let new_len = (existing_section.data.len() & (!align + 1)) + align;
                    existing_section.data.resize(new_len, 0xff);
                    existing_section.data.append(&mut section.data.clone());
                }

                // TODO: rewrite values of header
                existing_section.header.size = existing_section.data.len();
            }
        }
    }
}

pub fn link(mut object_files: Vec<ElfFile64>) -> ElfFile64 {
    let mut result = object_files.pop().expect("need object file but not found");

    for object_file in object_files.iter() {
        result.link(object_file);
    }

    result
}
