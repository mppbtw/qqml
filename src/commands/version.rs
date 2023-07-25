use crate::utils::*;
use help::HelpCommand;
use help::print_help;
use std::process::exit;

pub fn version() -> ! {
    if let Some(a) = next_arg() {
        match a.as_str() {
            "-h" | "--help" => print_help(HelpCommand::Version),
            _ => {
                print_error(format!("Unexpected argument: {}", a));
                exit(1);
            }
        }
    }
    match option_env!("CARGO_PKG_VERSION") {
        Some(v) => println!("yarr {}", v),
        None => print_error("Failed to get version metadata!"),
    };
    exit(0);
}
