use std::env::args;

const OPTIONS: [&'static str; 6] = [
    "-h", "--help",
    "-v", "--version",
    "-c", "--check",
];

pub fn has_help() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned())
}

pub fn get_file_arg() -> Option<String> {
    for a in &args().collect::<Vec<String>>()[1..] {
        if !OPTIONS.contains(&a.as_str()) {
            return Some(a.clone())
        }
    }
    None
}
