use crate::utils::*;
use std::process::exit;

pub fn play() {
    match next_arg() {
        Some(a) => {
            if a == "-h".to_owned() || a == "--help".to_owned() {
                print_help("play");
                exit(0);
            } else {
                if !dotfile_is_valid() {
                    print_error("The ~/.yarr directory is not yet setup. Please run yarr init");
                    exit(1);
                }
                if !section_exists(&a) {
                    print_error(format!("No such section: {}", a));
                    exit(1);
                }
                println!("Playing section {}", a);
                exit(0);
            }
        }
        None => {
            print_error("No section specified. Run yarr play --help for more info.");
            exit(1);
        }
    }
}

fn play_section(sec: String) {}
