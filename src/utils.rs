use std::collections::HashMap;

use crate::elf::ElfFile64;

pub fn next_aligned_value(value: usize, align: usize) -> usize {
    if align == 0 || align == 1 {
        value
    } else {
        (value & (!align + 1)) + align
    }
}

pub fn build_section_name_map(file: &ElfFile64) -> HashMap<String, usize> {
    let mut retval = HashMap::new();
    for (i, section) in file.unorganized_sections.iter().enumerate() {
        retval.insert(section.name.clone(), i);
    }

    retval
}

pub fn build_symbol_name_map(file: &ElfFile64) -> HashMap<String, usize> {
    let mut retval = HashMap::new();
    for (i, symbol) in file.symbols.iter().enumerate() {
        retval.insert(symbol.name.clone(), i);
    }

    retval
}
