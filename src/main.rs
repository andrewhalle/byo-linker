use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

use ld_rs::parse::ElfFile64Parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(short, long, parse(from_os_str))]
    filename: PathBuf,
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

    let elf = match ElfFile64Parser::new(&buf[..]).parse() {
        Ok(elf) => {
            println!("That is an ELF file!");
            elf
        }
        Err(_) => {
            eprintln!("That is not an ELF file!");
            std::process::exit(1);
        }
    };

    println!("{:#?}", elf);
}
