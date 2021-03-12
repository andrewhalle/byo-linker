use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use structopt::StructOpt;

use ld_rs::elf::ElfFile64;
use ld_rs::link;

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

    let mut elfs = Vec::new();

    for f in opt.filenames.iter() {
        let mut buf = Vec::new();
        let mut file = File::open(f).unwrap_or_else(|_| generic_error("opening"));
        file.read_to_end(&mut buf)
            .unwrap_or_else(|_| generic_error("reading"));

        match ElfFile64::parse(&buf[..]) {
            Ok(elf) => {
                elfs.push(elf);
            }
            Err(_) => {
                eprintln!("That is not an ELF file!");
                std::process::exit(1);
            }
        };
    }

    let result = link(elfs);

    let mut file = File::create("main").expect("could not create file");
    ElfFile64::write_out(result, &mut file).expect("could not write file");
    /*
    let mut file = File::create("output.o").expect("could not create file");
    ElfFile64::write_out(link(elfs), &mut file).expect("could not write file");
    */
}
