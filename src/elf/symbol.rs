use std::collections::HashMap;

use super::parse::{ElfFile64Raw, Symbol64Raw};
use super::section::Section64;

#[derive(Debug)]
pub struct Symbol64 {
    pub name: String,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}

pub fn get_symbols(
    raw: &ElfFile64Raw,
    symtab_section: &Section64,
    index_map: &HashMap<usize, usize>,
) -> Vec<Symbol64> {
    let raw_symbols =
        Symbol64Raw::parse_many(&symtab_section.data[..], raw.header.identifier.endianness)
            .expect("could not get symbols");

    // XXX duplicated from super::section
    // move into method on ElfFile64Raw
    let get_name = |idx: usize| {
        let symbol_name_string_table_header = &raw.section_headers[symtab_section.link as usize];
        let offset = (symbol_name_string_table_header.offset - raw.header.ehsize as u64) as usize;
        let start_of_name = &raw.section_data[offset + idx] as *const u8 as *const i8;
        let name = unsafe { std::ffi::CStr::from_ptr(start_of_name) };

        name.to_str().expect("could not create a &str").to_string()
    };

    let mut symbols = Vec::new();
    for raw_symbol in raw_symbols.iter() {
        symbols.push(Symbol64 {
            name: get_name(raw_symbol.name as usize),
            info: raw_symbol.info,
            other: raw_symbol.other,
            shndx: get_new_shndx(raw_symbol.shndx, index_map),
            value: raw_symbol.value,
            size: raw_symbol.size,
        });
    }
    symbols
}

fn get_new_shndx(old: u16, index_map: &HashMap<usize, usize>) -> u16 {
    if old == 0xfff1 {
        // SHN_ABS - not affected by relocation
        old
    } else {
        let old_shndx = old as usize;
        let new_shndx = index_map
            .get(&old_shndx)
            .expect("tried to reference a filtered out section");

        *new_shndx as u16
    }
}
