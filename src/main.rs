use std::process::exit;

mod utils;
mod commands;

use help::{print_help, HelpCommand};
use utils::*;

fn main() {
    let arg = match next_arg() {
        Some(a) => a,
        None => {
            print_error("No command specified. Run yarr --help for more info.");
            exit(1);
       },
    };

    match arg.as_str() {
        "-h" | "--help" => print_help(HelpCommand::Yarr),
        "-V" | "--version" => commands::version(),
        "qqml" => commands::qqml(),

        _ => print_error("Unknown command. Run yarr --help for more info."),
    }
}
