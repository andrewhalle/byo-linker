use super::elf::{ElfFile64, ElfFile64Header, ElfFile64SectionHeader};
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::io::Read;

pub struct ParseElfFile64Error;

impl<T: std::error::Error> From<T> for ParseElfFile64Error {
    fn from(_: T) -> Self {
        ParseElfFile64Error
    }
}

pub struct ElfFile64Parser<R: Read> {
    data: R,
    big_endian: Option<bool>,
}

type ParseResult<T> = Result<T, ParseElfFile64Error>;

impl<R: Read> ElfFile64Parser<R> {
    pub fn new(data: R) -> ElfFile64Parser<R> {
        ElfFile64Parser {
            data,
            big_endian: None,
        }
    }

    fn skip(&mut self, len: usize) -> ParseResult<()> {
        let mut buf = vec![0; len];
        self.data.read_exact(&mut buf)?;

        Ok(())
    }

    fn read_u16(&mut self) -> ParseResult<u16> {
        Ok(match self.big_endian {
            Some(be) => {
                if be {
                    self.data.read_u16::<BigEndian>()?
                } else {
                    self.data.read_u16::<LittleEndian>()?
                }
            }
            None => Err(ParseElfFile64Error)?,
        })
    }

    fn read_u32(&mut self) -> ParseResult<u32> {
        Ok(match self.big_endian {
            Some(be) => {
                if be {
                    self.data.read_u32::<BigEndian>()?
                } else {
                    self.data.read_u32::<LittleEndian>()?
                }
            }
            None => Err(ParseElfFile64Error)?,
        })
    }

    fn read_u64(&mut self) -> ParseResult<u64> {
        Ok(match self.big_endian {
            Some(be) => {
                if be {
                    self.data.read_u64::<BigEndian>()?
                } else {
                    self.data.read_u64::<LittleEndian>()?
                }
            }
            None => Err(ParseElfFile64Error)?,
        })
    }

    fn check_magic(&mut self) -> ParseResult<()> {
        let mut buf: [u8; 5] = [0; 5];
        self.data.read_exact(&mut buf)?;

        if &buf == b"\x7FELF\x02" {
            Ok(())
        } else {
            Err(ParseElfFile64Error)
        }
    }

    fn parse_data(&mut self) -> ParseResult<u8> {
        let data = self.data.read_u8()?;

        if data == 1 {
            self.big_endian = Some(false);
        } else if data == 2 {
            self.big_endian = Some(true);
        } else {
            Err(ParseElfFile64Error)?
        }

        Ok(data)
    }

    fn parse_header(&mut self) -> ParseResult<ElfFile64Header> {
        let data = self.parse_data()?;

        let version = self.data.read_u8()?;
        let os_abi = self.data.read_u8()?;
        let abi_version = self.data.read_u8()?;

        // ignored
        let mut buf: [u8; 7] = [0; 7];
        self.data.read_exact(&mut buf)?;

        let r#type = self.read_u16()?;
        let machine = self.read_u16()?;
        let e_version = self.read_u32()?;
        let entry = self.read_u64()? as usize;
        let phoff = self.read_u64()? as usize;
        let shoff = self.read_u64()? as usize;
        let flags = self.read_u32()?;
        let ehsize = self.read_u16()?;
        let phentsize = self.read_u16()?;
        let phnum = self.read_u16()?;
        let shentsize = self.read_u16()?;
        let shnum = self.read_u16()?;
        let shstrndx = self.read_u16()?;

        let header = ElfFile64Header {
            data,
            version,
            os_abi,
            abi_version,
            r#type,
            machine,
            e_version,
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
        };

        Ok(header)
    }

    fn parse_section_header(&mut self) -> ParseResult<ElfFile64SectionHeader> {
        let name = self.read_u32()?;
        let r#type = self.read_u32()?;
        let flags = self.read_u64()?;
        let addr = self.read_u64()? as usize;
        let offset = self.read_u64()? as usize;
        let size = self.read_u64()? as usize;
        let link = self.read_u32()?;
        let info = self.read_u32()?;
        let addr_align = self.read_u64()?;
        let entsize = self.read_u64()? as usize;

        Ok(ElfFile64SectionHeader {
            name,
            r#type,
            flags,
            addr,
            offset,
            size,
            link,
            info,
            addr_align,
            entsize,
        })
    }

    pub fn parse(mut self) -> ParseResult<ElfFile64> {
        self.check_magic()?;
        let header = self.parse_header()?;
        let program_headers = Vec::new();
        self.skip(header.shoff - header.ehsize as usize)?;
        let mut section_headers = Vec::new();
        for _ in 0..header.shnum {
            section_headers.push(self.parse_section_header()?);
        }

        Ok(ElfFile64 {
            header,
            program_headers,
            section_headers,
        })
    }
}
