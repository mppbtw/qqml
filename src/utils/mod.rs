use colored::Colorize;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn dotfile_is_valid() -> bool {
    exists("~/.yarr/config") && exists("~/.yarr/sections")
}

fn exists(dir: &str) -> bool {
    Path::new(dir).exists()
}

pub fn print_error<S: std::fmt::Display>(msg: S) {
    println!("{}, {}", "error: ".red(), msg);
}

pub fn print_help(command: &str) {
    println!("Help text goes here for command: {}", command);
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

fn strip_newline(str: &mut String) {
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
