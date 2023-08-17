use std::env::args;

const OPTIONS: [&str; 12] = [
    "-h",
    "--help",
    "-v",
    "--version",
    "-c",
    "--check",
    "-p",
    "--parse",
    "-j",
    "--json",
    "-l",
    "--log",
];

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

pub fn get_logfile() -> Option<String> {
    let args = args().collect::<Vec<String>>();
    for (i, a) in args.iter().enumerate() {
        if a == "-l" || a == "--log" {
            return args.get(i + 1).cloned();
        }
    }
    None
}

pub fn get_file_arg() -> Option<String> {
    let mut skip_next_arg = false;

    for a in &args().collect::<Vec<String>>()[1..] {
        if skip_next_arg {
            skip_next_arg = false;
            continue;
        }
        if a == "-l" || a == "--log" {
            skip_next_arg = true;
            continue;
        }
        if !OPTIONS.contains(&a.as_str()) {
            return Some(a.clone());
        }
    }
    None
}