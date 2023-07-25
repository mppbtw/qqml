use std::process::exit;
use std::fs;

use crate::utils::*;

use qqml_parser::*;
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
        print_error(format!("Unrecognised option '{}'", arg));
        exit(1);
    }

    let buff = match fs::read_to_string(&arg) {
        Ok(f) => f,
        Err(e) => {
            print_error(format!("Failed to read file '{}': {}", arg, e));
            exit(1);
        }
    };

    match next_arg() {
        Some(a) => {
            if a == *"-v" || a == *"--verbose" {
                verbose = true;
            } else {
                print_error(format!("Unrecognised argument '{}'", a));
                exit(1);
            }
        }
        None => ()
    }

    println!("Checking {}.", arg);
    let parsed = match parse(buff) {
        Ok(o) => o,
        Err(e) => {
            print_error(format!("{}", e));
            exit(1);
        }
    };

    // And the most useless command flag award goes to...
    if verbose {
        println!("{} Questions checked.", parsed.len());
    }
    println!("0 errors reported.");
    exit(1);
}
