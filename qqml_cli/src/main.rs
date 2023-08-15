use qqml_eval::render_error;
use qqml_eval::run;
use qqml_parser::parse;
use std::time::Instant;

use std::{env::args, fs, process::exit};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_BOLD: &str = "\x1b[1m";

mod argparse;
use argparse::*;

fn main() {
    if has_help() || args().collect::<Vec<String>>().len() == 1 {
        help_msg();
    }
    match get_file_arg() {
        Some(i) => match fs::read_to_string(i.clone()) {
            Ok(f) => {
                if has_check() {
                    check_file(f, i);
                } else {
                    run(&f, Some(&i));
                }
            }
            Err(e) => eprintln!("Couldn't read file: {}", e),
        },
        None => help_msg(),
    }
}

fn check_file(inp: String, path: String) -> ! {
    let start_t = Instant::now();
    match parse(&inp) {
        Ok(_) => {
            println!(
                "{}{}    Finished{} target in {}ms.",
                ANSI_GREEN,
                ANSI_BOLD,
                ANSI_RESET,
                start_t.elapsed().as_millis(),
            );
            exit(0);
        }
        Err(r) => {
            for e in r.errors.iter().rev() {
                println!("{}", render_error(&inp, e, Some(&path)));
            }
            println!(
                "{}{}    Finished{} check in {}ms",
                ANSI_RED,
                ANSI_BOLD,
                ANSI_RESET,
                start_t.elapsed().as_millis()
            );

            println!(
                "{}{}    Error:{} Failed to parse {} due to {} error{}",
                ANSI_RED,
                ANSI_BOLD,
                ANSI_RESET,
                path,
                r.errors.len(),
                if r.errors.len() == 1 { "" } else { "s" }
            );
            exit(1);
        }
    }
}

fn help_msg() -> ! {
    println!(
        " QQML v1.0 (c) 2023 'MrPiggyPegasus'

usage: qqml [OPTIONS] <FILE>

OPTIONS:
    -c --check      Validate the QQML source file.

    -h --help       Print this help message and
                    exit.

    -V --version    Print version information and
                    exit.

More information about the QQML language
and its related tooling is available at
https://github.com/MrPiggyPegasus/qqml "
    );
    exit(1);
}
