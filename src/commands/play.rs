use crate::utils::*;
use help::*;
use std::fs;
use std::process::exit;

pub fn play() {
    match next_arg() {
        Some(a) => {
            if a == *"-h" || a == *"--help" {
                print_help(HelpCommand::SectionPlay);
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
                play_section(a);
                exit(0);
            }
        }
        None => {
            print_error("No section specified. Run yarr section play --help for more info.");
            exit(1);
        }
    }
}

fn play_section(sec_name: String) {
    let sec_path = format!("{}/{}", *YARR_SECTIONS_DIR, sec_name);
    let sec = match fs::read_to_string(&sec_path) {
        Ok(s) => s,
        Err(e) => {
            print_error(format!("Failed to read file {}: {}", sec_path, e));
            exit(1);
        }
    };

    println!("Section contents: {}", sec);
    exit(0);
}
