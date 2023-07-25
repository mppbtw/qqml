use std::process::exit;
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

    exit(1);
}
