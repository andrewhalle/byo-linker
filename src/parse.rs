use nom::bytes::complete::{tag, take};
use nom::number::complete as num_parse;
use nom::IResult;

use crate::elf::ELF_MAGIC;

#[derive(Default, Debug)]
struct ElfFile64HeaderRaw {
    pub data: u8,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
    pub r#type: u16,
    pub machine: u16,
    pub e_version: u32,
    pub entry: usize,
    pub phoff: usize,
    pub shoff: usize,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

#[derive(Debug)]
struct ElfFileIdentifier {
    class: u8,
    endianness: nom::number::Endianness,
    version: u8,
    os_abi: u8,
    abi_version: u8,
}

impl ElfFileIdentifier {
    fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, _) = tag(ELF_MAGIC)(input)?;
        let (input, class) = num_parse::u8(input)?;
        let (input, endianness) = elf_endianness(input)?;
        let (input, version) = num_parse::u8(input)?;
        let (input, os_abi) = num_parse::u8(input)?;
        let (input, abi_version) = num_parse::u8(input)?;
        let (input, _) = take(7_usize)(input)?;

        Ok((
            input,
            ElfFileIdentifier {
                class,
                endianness,
                version,
                os_abi,
                abi_version,
            },
        ))
    }
}

fn elf_endianness(input: &[u8]) -> IResult<&[u8], nom::number::Endianness> {
    let (input, b) = num_parse::u8(input)?;

    let endianness = match b {
        0x01 => nom::number::Endianness::Little,
        0x02 => nom::number::Endianness::Big,
        _ => unreachable!(),
    };

    Ok((input, endianness))
}
