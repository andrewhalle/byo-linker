use byteorder::{BigEndian, LittleEndian, ReadBytesExt};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    filename: PathBuf,
}

#[derive(Debug)]
struct ElfFile64Header {
    data: u8,
    version: u8,
    os_abi: u8,
    abi_version: u8,
    r#type: u16,
    machine: u16,
    e_version: u32,
    entry: usize,
    phoff: usize,
    shoff: usize,
    flags: u32,
    ehsize: u16,
    phentsize: u16,
    phnum: u16,
    shentsize: u16,
    shnum: u16,
    shstrndx: u16,
}

#[derive(Debug)]
struct ElfFile64ProgramHeaderTableEntry;
#[derive(Debug)]
struct ElfFile64SectionHeaderTableEntry;

#[derive(Debug)]
struct ElfFile64 {
    header: ElfFile64Header,
    program: Vec<ElfFile64ProgramHeaderTableEntry>,
    section: Vec<ElfFile64SectionHeaderTableEntry>,
}

fn check_magic_and_class(b: &mut impl Read) -> Result<(), ParseElfFile64Error> {
    let mut buf: [u8; 5] = [0; 5];
    b.read_exact(&mut buf)?;

    if &buf == b"\x7FELF\x02" {
        Ok(())
    } else {
        Err(ParseElfFile64Error)
    }
}

trait Data {
    fn data() -> u8;
}

impl Data for LittleEndian {
    fn data() -> u8 {
        1
    }
}

impl Data for BigEndian {
    fn data() -> u8 {
        2
    }
}

fn parse_fields<B: Read, T: Data + byteorder::ByteOrder>(
    b: &mut B,
) -> Result<ElfFile64, ParseElfFile64Error> {
    let version = b.read_u8()?;
    let os_abi = b.read_u8()?;
    let abi_version = b.read_u8()?;
    let mut buf: [u8; 7] = [0; 7];
    b.read_exact(&mut buf)?;
    let r#type = b.read_u16::<T>()?;
    let machine = b.read_u16::<T>()?;
    let e_version = b.read_u32::<T>()?;
    let entry = b.read_u64::<T>()? as usize;
    let phoff = b.read_u64::<T>()? as usize;
    let shoff = b.read_u64::<T>()? as usize;
    let flags = b.read_u32::<T>()?;
    let ehsize = b.read_u16::<T>()?;
    let phentsize = b.read_u16::<T>()?;
    let phnum = b.read_u16::<T>()?;
    let shentsize = b.read_u16::<T>()?;
    let shnum = b.read_u16::<T>()?;
    let shstrndx = b.read_u16::<T>()?;

    let header = ElfFile64Header {
        data: T::data(),
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

    Ok(ElfFile64 {
        header,
        program: Vec::new(),
        section: Vec::new(),
    })
}

struct ParseElfFile64Error;
impl<T: std::error::Error> From<T> for ParseElfFile64Error {
    fn from(_: T) -> Self {
        ParseElfFile64Error
    }
}

impl ElfFile64 {
    fn from_bytes(mut b: impl Read) -> Result<ElfFile64, ParseElfFile64Error> {
        check_magic_and_class(&mut b)?;
        let data = b.read_u8()?;

        if data == 1 {
            parse_fields::<_, LittleEndian>(&mut b)
        } else {
            parse_fields::<_, BigEndian>(&mut b)
        }
    }
}

fn generic_error(action: &str) -> ! {
    eprintln!("Something went wrong {} that file.", action);
    std::process::exit(1)
}

fn main() {
    let opt = Opt::from_args();

    let mut file = File::open(opt.filename).unwrap_or_else(|_| generic_error("opening"));
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)
        .unwrap_or_else(|_| generic_error("reading"));

    let elf = match ElfFile64::from_bytes(&buf[..]) {
        Ok(elf) => {
            println!("That is an ELF file!");
            elf
        }
        Err(_) => {
            eprintln!("That is not an ELF file!");
            std::process::exit(1);
        }
    };

    println!("{:?}", elf);
}
