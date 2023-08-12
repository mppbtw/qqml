use std::{process::exit, env::args, fs};
use qqml_eval::run;

mod argparse;
use argparse::*;

fn main() {
    if has_help() || args().collect::<Vec<String>>().len() == 1 {
        help_msg();
    }
    match get_file_arg() {
        Some(i) => {
            match fs::read_to_string(i.clone()) {
                Ok(f) => run(f, Some(i)),
                Err(e) => eprintln!("Couldn't read file: {}", e),
            }
        },
        None => help_msg(),
    }
}

fn help_msg() -> ! {
    println!("
QQML v1.0 (c) 2023 'MrPiggyPegasus'

usage: qqml [OPTIONS] <FILE>

OPTIONS:
    -c --check      Validate the QQML source file.

    -h --help       Print this help message and
                    exit.

    -V --version    Print version information and
                    exit.

More information about the QQML language
and its related tooling is available at
https://github.com/MrPiggyPegasus/qqml ");
    exit(1);
}
