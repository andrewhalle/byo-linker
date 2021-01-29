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

struct ElfFile64 {
    header: [u8; 4],
    class: u8,
    data: u8,
    version: u8,
    abi: u8,
    abi_version: u8,
    pad: [u8; 7],
    r#type: [u8; 2],
    machine: [u8; 2],
    e_version: [u8; 4],
    entry: [u8; 8],
    phoff: [u8; 8],
    shoff: [u8; 8],
    flags: [u8; 4],
    ehsize: [u8; 2],
    phentsize: [u8; 2],
    phnum: [u8; 2],
    shentsize: [u8; 2],
    shnum: [u8; 2],
    shstrndx: [u8; 2],
    _rest: [u8],
}

struct ParseElfFile64Error;
impl ElfFile64 {
    fn from_vec(v: Vec<u8>) -> Result<Box<ElfFile64>, ParseElfFile64Error> {
        if &v[0..4] != b"\x7FELF" || &v[4] != &2_u8 {
            Err(ParseElfFile64Error)
        } else {
            Ok(unsafe { std::mem::transmute(v.into_boxed_slice()) })
        }
    }

    fn print_info(&self) {
        println!("Header: {:#?}", self.header);
        println!("Class: {}", self.class);
        println!("Data: {}", self.data);
        println!("Version: {}", self.version);
        println!("ABI: {}", self.abi);
        println!("ABI Version: {}", self.abi_version);
        println!("Pad: {:#?}", self.pad);
        println!("Type: {:#?}", self.r#type);
        println!("Machine: {:#?}", self.machine);
        println!("E_Version: {:#?}", self.e_version);
        println!("Entry: {:#?}", self.entry);
        println!("Phoff: {:#?}", self.phoff);
        println!("Shoff: {:#?}", self.shoff);
        println!("Flags: {:#?}", self.flags);
        println!("Ehsize: {:#?}", self.ehsize);
        println!("Phentsize: {:#?}", self.phentsize);
        println!("Phnum: {:#?}", self.phnum);
        println!("Shentsize: {:#?}", self.shentsize);
        println!("Shnum: {:#?}", self.shnum);
        println!("Shstrndx: {:#?}", self.shstrndx);
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

    let elf = match ElfFile64::from_vec(buf) {
        Ok(elf) => {
            println!("That is an ELF file!");
            elf
        }
        Err(_) => {
            eprintln!("That is not an ELF file!");
            std::process::exit(1);
        }
    };

    elf.print_info();
}
