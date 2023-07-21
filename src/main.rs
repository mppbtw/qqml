use std::env;

mod utils;
mod commands;

fn help(subcommand: &str) {
    println!("help logic here");
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 0 {
        let arg = args.get(0).unwrap();
        if arg.starts_with('-') {
            match arg.as_str() {
                "-v" => commands::version(),
                "--version" => commands::version(),
                _ => help(""),
            }
            return;
        }
        match arg.as_str() {
            "play" => commands::play(),
            _ => help("play"),
        }
        return;
    }
    help("");
}
