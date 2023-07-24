use crate::commands::play;
use crate::utils::*;
use help::*;
use std::process::exit;

pub fn section() {
    if let Some(a) = next_arg() {
        match a.as_str() {
            "-h" | "--help" => print_help(HelpCommand::Section),
            "play" => play(),
            _ => {
                print_error(format!("Unexpected argument: {}", a));
                exit(1);
            }
        }
    }
    print_error("No command specified. Run yarr section --help for more info.");
    exit(0);
}
