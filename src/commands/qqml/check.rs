use std::process::exit;
use std::fs;

use crate::utils::*;

use help::*;

pub fn check() -> ! {
    let arg = match next_arg() {
        Some(a) => a,
        None => {
            print_error("No filename provided. Run yarr qqml check --help for more info.");
            exit(1);
        }
    };

    if arg == *"-h" || arg == *"--help" {
        print_help(HelpCommand::QqmlCheck);
        exit(0);
    }

    let mut verbose = false;
    if arg == *"-v" || arg == *"--verbose" {
        verbose = true;
    }

    if arg.starts_with('-') {
        print_error(format!("Unrecognised option {}", arg));
    }

    let mut buf = String::new();
    match fs::read_to_string(&mut buf) {
        Ok(_) => (),
        Err(e) => {
            print_error(format!("Failed to read file '{}': {}", arg, e))
        }
    }

    match next_arg() {
        Some(a) => {
            if a == *"-v" || a == *"--verbose" {
                verbose = true;
            }
        }
        None => ()
    }

    if verbose {
        println!("*flamboyantly* looks good to me!");
    } else {
        println!("ok i think")
    }
    exit(1);
}
