use std::process::exit;

mod commands;
mod help;
mod utils;

use help::*;
use utils::*;

fn main() {
    let arg = match next_arg() {
        Some(a) => a,
        None => {
            println!("No command specified. Run yarr --help for a list of commands.");
            exit(1);
        }
    };

    match arg.as_str() {
        "version" | "-v" | "--version" => commands::version(),
        "help" | "-h" | "--help" => print_help(HelpCommand::Yarr),

        "play" => commands::play(),
        "init" => commands::init(),

        _ => {
            println!("Unknown command. Run yarr --help for a list of commands.");
            exit(1);
        }
    }
}
