use super::parse::{ElfFile64Raw, Symbol64Raw};
use super::section::{Section64, SectionType64};

#[derive(Debug)]
pub struct Symbol64 {
    name: String,
    info: u8,
    other: u8,
    shndx: u16,
    value: u64,
    size: u64,
}

pub fn get_symbols(raw: &ElfFile64Raw, sections: &Vec<Section64>) -> Vec<Symbol64> {
    let symtab_section = sections
        .iter()
        .find(|s| s.r#type == SectionType64::Symtab)
        .expect("could not find .symtab");
    let raw_symbols =
        Symbol64Raw::parse_many(&symtab_section.data[..], raw.header.identifier.endianness)
            .expect("could not get symbols");

    // XXX duplicated from super::section
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
            shndx: raw_symbol.shndx,
            value: raw_symbol.value,
            size: raw_symbol.size,
        });
    }
    symbols
}
