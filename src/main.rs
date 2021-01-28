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

fn generic_error(action: &str) -> ! {
    eprintln!("Something went wrong {} that file.", action);
    std::process::exit(1)
}

fn main() {
    let opt = Opt::from_args();

    let mut file = match File::open(opt.filename) {
        Err(_) => generic_error("opening"),
        Ok(f) => f,
    };
    let mut buf = [0; 4];
    match file.read(&mut buf) {
        Err(_) => generic_error("reading"),
        Ok(_) => {}
    }

    if &buf == b"\x7FELF" {
        println!("That is an ELF file!");
    } else {
        println!("That is not an ELF file!");
    }
}
