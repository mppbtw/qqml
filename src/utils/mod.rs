use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::Path;
use std::process::exit;

mod constants;
mod macros;
mod next_arg;

pub use constants::*;
pub use next_arg::*;

pub fn path_exists<S: Into<String>>(dir: S) -> bool {
    Path::new(&dir.into()).exists()
}

pub fn dotfile_is_valid() -> bool {
    path_exists(YARR_DIR.to_owned()) && path_exists(YARR_SECTIONS_DIR.to_owned())
}

pub fn print_error<S: std::fmt::Display>(msg: S) {
    println!("{} {}", "error: ".red(), msg);
}

pub fn print_help(command: &str) {
    println!("Help text goes here for command: {}", command);
    exit(0);
}

pub fn yes_or_no<S: std::fmt::Display>(msg: S, default: bool) -> bool {
    if default {
        loop {
            print!("{}, [{}/{}]", msg, "Y".green(), "n".red());
            io::stdout().flush().unwrap();
            let mut buff = String::new();
            io::stdin().read_line(&mut buff).unwrap();
            strip_newline(&mut buff);

            if buff.to_lowercase() == "y" || buff.is_empty() {
                return true;
            }
            if buff.to_lowercase() == "n" {
                return false;
            }

            print_error("Invalid input, please try again.");
        }
    } else {
        loop {
            print!("{}, [{}/{}]", msg, "y".green(), "N".red());
            io::stdout().flush().unwrap();
            let mut buff = String::new();
            io::stdin().read_line(&mut buff).unwrap();
            strip_newline(&mut buff);

            if buff.to_lowercase() == "n" || buff.is_empty() {
                return false;
            }
            if buff.to_lowercase() == "y" {
                return true;
            }

            print_error("Invalid input, please try again.");
        }
    }
}

pub fn strip_newline(str: &mut String) {
    loop {
        if str.ends_with('\n') {
            *str = str.strip_suffix('\n').unwrap().to_owned();
            if str.ends_with('\r') {
                *str = str.strip_suffix('\r').unwrap().to_owned();
            }
        } else {
            break;
        }
    }
}

pub fn validate_section_name(name: &str) -> bool {
    let bytes = name.bytes();
    for ch in bytes {
        if !(ch.is_ascii_lowercase()
            || ch.is_ascii_uppercase()
            || ch == b'_'
            || ch.is_ascii_digit())
        {
            return false;
        }
    }
    true
}
