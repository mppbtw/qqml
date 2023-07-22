use std::env;

mod commands;
mod utils;
mod qqml;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 0 {
        let arg = args.get(0).unwrap();
        if arg.starts_with('-') {
            match arg.as_str() {
                "-v" | "--version" => commands::version(),
                "-h" | "--help" => utils::print_help(""),
                _ => println!("Unknown option. Run yarr --help for a list of commands."),
            }
            return;
        }
        match arg.as_str() {
            "play" => commands::play(),
            "help" => utils::print_help(""),
            _ => println!("Unknown command. Run yarr --help for a list of commands."),
        }
        return;
    }
    println!("No command specified. Run yarr --help for a list of commands.");
}
