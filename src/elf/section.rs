use std::collections::HashMap;

use super::parse::ElfFile64Raw;
use super::relocation::RelocationA64;
use super::symbol::{sym_bind, Symbol64};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SectionType64 {
    Null,
    Progbits,
    Symtab,
    Strtab,
    Rela,
    UnwindX64,
    Loos,
}

impl From<u32> for SectionType64 {
    fn from(x: u32) -> Self {
        use SectionType64::*;

        match x {
            0 => Null,
            1 => Progbits,
            2 => Symtab,
            3 => Strtab,
            4 => Rela,
            1879048193 => UnwindX64,
            1879002115 => Loos,
            _ => unimplemented!(),
        }
    }
}

impl From<SectionType64> for u32 {
    fn from(x: SectionType64) -> Self {
        use SectionType64::*;

        match x {
            Null => 0,
            Progbits => 1,
            Symtab => 2,
            Strtab => 3,
            Rela => 4,
            UnwindX64 => 1879048193,
            Loos => 1879002115,
        }
    }
}

#[derive(Debug)]
pub struct Section64 {
    pub name: String,
    pub r#type: SectionType64,
    pub flags: u64,
    pub addr: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub data: Vec<u8>,
    pub relocations: Option<Vec<RelocationA64>>,
}

pub fn get_sections(raw: &ElfFile64Raw) -> Vec<Section64> {
    let get_name = |idx: usize| {
        let section_name_string_table_header = &raw.section_headers[raw.header.shstrndx as usize];
        let offset = (section_name_string_table_header.offset - raw.header.ehsize as u64) as usize;
        let start_of_name = &raw.section_data[offset + idx] as *const u8 as *const i8;
        let name = unsafe { std::ffi::CStr::from_ptr(start_of_name) };

        name.to_str().expect("could not create a &str").to_string()
    };

    let get_data = |offset: usize, size: usize| {
        if offset == 0 && size == 0 {
            return Vec::new();
        }

        let begin = offset - raw.header.ehsize as usize;
        let end = begin + size as usize;
        let data = &raw.section_data[begin..end];

        data.to_owned()
    };

    let mut sections = Vec::new();
    for section_header in raw.section_headers.iter() {
        sections.push(Section64 {
            name: get_name(section_header.name as usize),
            r#type: section_header.r#type.into(),
            flags: section_header.flags,
            addr: section_header.addr,
            link: section_header.link,
            info: section_header.info,
            addralign: section_header.addralign,
            data: get_data(section_header.offset as usize, section_header.size as usize),
            relocations: None,
        });
    }

    sections
}

// filter out sections that have their data represented elsewhere in the in-memory
// structure. e.g.
//   * string tables
//   * symbol tables
//   * relocation tables
// return:
//   - unorganized_sections
//   - symtab section
//   - rela sections
//   - map from old indexes to new for unorganized sections
pub fn organize_sections(
    sections: Vec<Section64>,
) -> (
    Vec<Section64>,
    Section64,
    Vec<Section64>,
    HashMap<usize, usize>,
) {
    use SectionType64::*;

    let mut unorganized_sections = Vec::new();
    let mut symtab = None;
    let mut relas = Vec::new();
    let mut index_map = HashMap::new();

    let mut offset = 0;
    for (i, section) in sections.into_iter().enumerate() {
        match section.r#type {
            Symtab => symtab = Some(section),
            Rela => relas.push(section),
            Strtab => {}
            _ => {
                unorganized_sections.push(section);
                index_map.insert(i, i - offset);
                continue;
            }
        }

        offset += 1;
    }

    // re-write rela references
    for rela in relas.iter_mut() {
        let old_info = rela.info as usize;
        let new_info = index_map
            .get(&old_info)
            .expect("referenced a section that was not explicitly included");
        rela.info = *new_info as u32;
    }

    (
        unorganized_sections,
        symtab.expect("no .symtab found"),
        relas,
        index_map,
    )
}

impl Section64 {
    pub fn from_strtab(strtab: &HashMap<String, usize>, name: String) -> Self {
        use std::cmp::Ordering::*;
        use std::io::Write;
        use SectionType64::*;

        let mut all: Vec<(&String, &usize)> = strtab.into_iter().collect();
        all.sort_by(|(_, idx1), (_, idx2)| {
            if idx1 < idx2 {
                Less
            } else if idx1 > idx2 {
                Greater
            } else {
                Equal
            }
        });

        let mut data = vec![0_u8];
        for (s, _) in all.iter() {
            if &s[..] == "" {
                continue;
            }

            data.write_all(s.as_bytes()).expect("could not write");
            data.write_all(&[0_u8]).expect("could not write");
        }

        Section64 {
            name,
            r#type: Strtab,
            flags: 0,
            addr: 0,
            link: 0,
            info: 0,
            addralign: 1,
            data,
            relocations: None,
        }
    }

    pub fn from_symtab<T: byteorder::ByteOrder>(
        symtab: &Vec<Symbol64>,
        strtab: &HashMap<String, usize>,
        strtab_idx: usize,
    ) -> Self {
        use std::io::Write;
        use SectionType64::*;

        let mut data = Vec::new();
        for symbol in symtab {
            data.write_all(&symbol.as_raw::<T>(strtab)[..])
                .expect("could not write");
        }

        let info = symtab
            .iter()
            .enumerate()
            .find(|(_, sym)| sym_bind(*sym) != 0)
            .expect("no nonlocal symbols")
            .0 as u32;

        Section64 {
            name: ".symtab".to_string(),
            r#type: Symtab,
            flags: 0,
            addr: 0,
            link: strtab_idx as u32,
            info,
            addralign: 8,
            data,
            relocations: None,
        }
    }

    pub fn from_rela<T: byteorder::ByteOrder>(
        relas: &Vec<RelocationA64>,
        symtab_idx: usize,
        section_idx: usize,
        name: String,
    ) -> Self {
        use std::io::Write;
        use SectionType64::*;

        let mut data = Vec::new();
        for rela in relas {
            data.write_all(&rela.as_raw::<T>()[..])
                .expect("could not write");
        }

        Section64 {
            name,
            r#type: Rela,
            flags: 0, // set I flag?
            addr: 0,
            link: symtab_idx as u32,
            info: section_idx as u32,
            addralign: 8,
            data,
            relocations: None,
        }
    }
}
