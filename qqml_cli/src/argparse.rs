use std::env::args;

const OPTIONS: [&str; 6] = ["-h", "--help", "-v", "--version", "-c", "--check"];

pub fn has_help() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned())
}

pub fn has_check() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-c".to_owned()) || args.contains(&"--check".to_owned())
}

pub fn has_parse() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-p".to_owned()) || args.contains(&"--parse".to_owned())
}

pub fn has_json() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-j".to_owned()) || args.contains(&"--json".to_owned())
}

pub fn get_file_arg() -> Option<String> {
    for a in &args().collect::<Vec<String>>()[1..] {
        if !OPTIONS.contains(&a.as_str()) {
            return Some(a.clone());
        }
    }
    None
}
