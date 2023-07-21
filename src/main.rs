use std::env;

fn help() {
    println!("help logic here");
}

fn version() {
    println!("i dunno how to get the version lol");
}

fn play() {
    println!("playing logic goes here");
}

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>()[1..].to_vec();

    if args.len() > 0 {
        let arg = args.get(0).unwrap();
        if arg.starts_with('-') {
            match arg.as_str() {
                "-v" => version(),
                "--version" => version(),
                _ => help(),
            }
            return;
        }
        match arg.as_str() {
            "play" => play(),
            _ => help(),
        }
        return;
    }
    help();
}
