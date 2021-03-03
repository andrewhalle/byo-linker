use nom::bytes::complete::{tag, take};
use nom::combinator::all_consuming;
use nom::multi::many1;
use nom::number::complete as num_parse;
use nom::Finish;
use nom::IResult;

use super::relocation::RelocationA64;
use super::ELF_MAGIC;

#[derive(Debug)]
pub struct ElfFileIdentifier {
    pub class: u8,
    pub endianness: nom::number::Endianness,
    pub version: u8,
    pub os_abi: u8,
    pub abi_version: u8,
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

#[derive(Debug)]
pub struct ElfFile64HeaderRaw {
    pub identifier: ElfFileIdentifier,
    pub r#type: u16,
    pub machine: u16,
    pub version: u32,
    pub entry: u64,
    pub phoff: u64,
    pub shoff: u64,
    pub flags: u32,
    pub ehsize: u16,
    pub phentsize: u16,
    pub phnum: u16,
    pub shentsize: u16,
    pub shnum: u16,
    pub shstrndx: u16,
}

impl ElfFile64HeaderRaw {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, identifier) = ElfFileIdentifier::parse(input)?;

        let parse_u16 = |i| num_parse::u16(identifier.endianness)(i);
        let parse_u32 = |i| num_parse::u32(identifier.endianness)(i);
        let parse_u64 = |i| num_parse::u64(identifier.endianness)(i);

        let (input, r#type) = parse_u16(input)?;
        let (input, machine) = parse_u16(input)?;
        let (input, version) = parse_u32(input)?;
        let (input, entry) = parse_u64(input)?;
        let (input, phoff) = parse_u64(input)?;
        let (input, shoff) = parse_u64(input)?;
        let (input, flags) = parse_u32(input)?;
        let (input, ehsize) = parse_u16(input)?;
        let (input, phentsize) = parse_u16(input)?;
        let (input, phnum) = parse_u16(input)?;
        let (input, shentsize) = parse_u16(input)?;
        let (input, shnum) = parse_u16(input)?;
        let (input, shstrndx) = parse_u16(input)?;

        Ok((
            input,
            ElfFile64HeaderRaw {
                identifier,
                r#type,
                machine,
                version,
                entry,
                phoff,
                shoff,
                flags,
                ehsize,
                phentsize,
                phnum,
                shentsize,
                shnum,
                shstrndx,
            },
        ))
    }
}

#[derive(Debug)]
pub struct ElfFile64SectionHeaderRaw {
    pub name: u32,
    pub r#type: u32,
    pub flags: u64,
    pub addr: u64,
    pub offset: u64,
    pub size: u64,
    pub link: u32,
    pub info: u32,
    pub addralign: u64,
    pub entsize: u64,
}

impl ElfFile64SectionHeaderRaw {
    pub fn parse(input: &[u8], endianness: nom::number::Endianness) -> IResult<&[u8], Self> {
        let parse_u32 = |i| num_parse::u32(endianness)(i);
        let parse_u64 = |i| num_parse::u64(endianness)(i);

        let (input, name) = parse_u32(input)?;
        let (input, r#type) = parse_u32(input)?;
        let (input, flags) = parse_u64(input)?;
        let (input, addr) = parse_u64(input)?;
        let (input, offset) = parse_u64(input)?;
        let (input, size) = parse_u64(input)?;
        let (input, link) = parse_u32(input)?;
        let (input, info) = parse_u32(input)?;
        let (input, addralign) = parse_u64(input)?;
        let (input, entsize) = parse_u64(input)?;

        Ok((
            input,
            ElfFile64SectionHeaderRaw {
                name,
                r#type,
                flags,
                addr,
                offset,
                size,
                link,
                info,
                addralign,
                entsize,
            },
        ))
    }
}

#[derive(Debug)]
pub struct ElfFile64Raw {
    pub header: ElfFile64HeaderRaw,
    pub section_data: Vec<u8>,
    pub section_headers: Vec<ElfFile64SectionHeaderRaw>,
}

#[derive(Debug)]
pub struct ElfFile64RawParseError;

impl From<nom::error::Error<&[u8]>> for ElfFile64RawParseError {
    fn from(_: nom::error::Error<&[u8]>) -> Self {
        ElfFile64RawParseError
    }
}

impl ElfFile64Raw {
    pub fn parse(input: &[u8]) -> Result<Self, ElfFile64RawParseError> {
        let (_, elf_file) = Finish::finish(all_consuming(ElfFile64Raw::parse_nom)(input))?;

        Ok(elf_file)
    }

    fn parse_nom(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, header) = ElfFile64HeaderRaw::parse(input)?;
        let (input, section_data) = take(header.shoff - header.ehsize as u64)(input)?;
        let (input, section_headers) =
            many1(|i| ElfFile64SectionHeaderRaw::parse(i, header.identifier.endianness))(input)?;

        Ok((
            input,
            ElfFile64Raw {
                header,
                section_data: section_data.to_vec(),
                section_headers,
            },
        ))
    }
}

#[derive(Debug)]
pub struct Symbol64Raw {
    pub name: u32,
    pub info: u8,
    pub other: u8,
    pub shndx: u16,
    pub value: u64,
    pub size: u64,
}

impl Symbol64Raw {
    pub fn parse_many(
        input: &[u8],
        endianness: nom::number::Endianness,
    ) -> Result<Vec<Self>, ElfFile64RawParseError> {
        let single_parser = Symbol64Raw::parse_one(endianness);
        let (_, symbols) = Finish::finish(all_consuming(many1(single_parser))(input))?;

        Ok(symbols)
    }

    pub fn parse_one(
        endianness: nom::number::Endianness,
    ) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
        move |input: &[u8]| {
            let parse_u16 = |i| num_parse::u16(endianness)(i);
            let parse_u32 = |i| num_parse::u32(endianness)(i);
            let parse_u64 = |i| num_parse::u64(endianness)(i);

            let (input, name) = parse_u32(input)?;
            let (input, info) = num_parse::u8(input)?;
            let (input, other) = num_parse::u8(input)?;
            let (input, shndx) = parse_u16(input)?;
            let (input, value) = parse_u64(input)?;
            let (input, size) = parse_u64(input)?;

            let symbol = Symbol64Raw {
                name,
                info,
                other,
                shndx,
                value,
                size,
            };

            Ok((input, symbol))
        }
    }
}

impl RelocationA64 {
    pub fn parse_many(
        input: &[u8],
        endianness: nom::number::Endianness,
    ) -> Result<Vec<Self>, ElfFile64RawParseError> {
        let single_parser = RelocationA64::parse_one(endianness);
        let (_, relas) = Finish::finish(all_consuming(many1(single_parser))(input))?;

        Ok(relas)
    }

    pub fn parse_one(
        endianness: nom::number::Endianness,
    ) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
        move |input: &[u8]| {
            let parse_u64 = |i| num_parse::u64(endianness)(i);

            let (input, offset) = parse_u64(input)?;
            let (input, info) = parse_u64(input)?;
            let (input, addend) = parse_u64(input)?;

            let rela = RelocationA64 {
                offset,
                info,
                addend,
            };

            Ok((input, rela))
        }
    }
}
