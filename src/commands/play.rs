use crate::utils::*;
use std::process::exit;

pub fn play() {
    match next_arg() {
        Some(a) => {
            if a == "-h".to_owned() || a == "--help".to_owned() {
                print_help("play");
                exit(0);
            } else {
                if !validate_section_name(&a) {
                    print_error(
                        format!("Invalid section name {}.", a)
                            + "Run yarr play --help for guidelines on how to name sections.",
                    );
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
