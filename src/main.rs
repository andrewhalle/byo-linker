use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

use ld_rs::elf::ElfFile64;
use ld_rs::parse::ElfFile64Parser;

#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    #[structopt(name = "FILE", parse(from_os_str))]
    filenames: Vec<PathBuf>,
}

fn generic_error(action: &str) -> ! {
    eprintln!("Something went wrong {} that file.", action);
    std::process::exit(1)
}

fn main() {
    let opt = Opt::from_args();

    let elfs: Vec<ElfFile64> = opt
        .filenames
        .iter()
        .map(|f| {
            let mut buf = Vec::new();
            let mut file = File::open(f).unwrap_or_else(|_| generic_error("opening"));
            file.read_to_end(&mut buf)
                .unwrap_or_else(|_| generic_error("reading"));

            let elf = match ElfFile64Parser::new(&buf[..]).parse(f.to_str().unwrap().to_string()) {
                Ok(elf) => elf,
                Err(_) => {
                    eprintln!("That is not an ELF file!");
                    std::process::exit(1);
                }
            };

            elf
        })
        .collect();

    let result = ld_rs::link(elfs);

    println!("{}", result);

    println!("{}", &result.sections[2].header.size);
    println!("{}", hex::encode(&result.sections[2].data));
}
