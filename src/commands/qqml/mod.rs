use std::process::exit;
use crate::utils::*;
use help::*;

mod check;

pub fn qqml() -> ! {
    let arg = match next_arg() {
        Some(a) => a,
        None => {
            print_error("No command specified. Run yarr qqml --help for more info.");
            exit(1);
        }
    };

    match arg.as_str() {
        "-h" | "--help" => print_help(HelpCommand::Qqml),
        "check" => check::check(),
        _ => {
            print_error(format!("Unrecognised argument '{}'", arg));
            exit(1);
        }
    }
    exit(0);
}
