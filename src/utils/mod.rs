use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::Path;

mod next_arg;

pub use next_arg::*;

pub fn path_exists<S: Into<String>>(dir: S) -> bool {
    Path::new(&dir.into()).exists()
}

pub fn print_error<S: std::fmt::Display>(msg: S) {
    println!("{} {}", "error: ".red(), msg);
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
