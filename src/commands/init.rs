use crate::utils::*;
use colored::Colorize;
use std::fs;
use std::process::exit;

pub fn init() {
    match next_arg() {
        Some(a) => match a.as_str() {
            "help" => {
                print_help("init");
                exit(0);
            }
            _ => {
                print_error(format!("Unknown argument : {}", a));
                exit(1);
            }
        },
        None => (),
    }

    if path_exists(YARR_DIR.to_owned()) {
        if path_exists(YARR_SECTIONS_DIR.to_owned()) {
            println!("~/.yarr is already set up.");
        } else {
            match fs::create_dir(YARR_SECTIONS_DIR.to_owned()) {
                Ok(_) => {
                    println!("Successfully created ~/.yarr/sections/");
                    exit(0);
                }
                Err(e) => {
                    print_error(format!(
                        "Failed to create ~/.yarr/sections/: {}",
                        e.to_string().red()
                    ));
                    exit(1);
                }
            }
        }
    } else {
        match fs::create_dir_all(YARR_SECTIONS_DIR.to_owned()) {
            Ok(_) => {
                println!("Successfully created ~/.yarr/sections/");
                exit(0);
            }
            Err(e) => {
                print_error(format!(
                    "Failed to create ~/.yarr/sections: {}",
                    e.to_string().red()
                ));
                exit(1);
            }
        }
    }
    exit(0);
}
