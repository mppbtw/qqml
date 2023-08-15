use qqml_eval::render_error;
use qqml_eval::run;
use qqml_parser::parse;
use qqml_parser::ErrorReport;
use qqml_parser::ParsedFile;
use std::time::Instant;

use std::{env::args, fs, process::exit};

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_RED: &str = "\x1b[31m";
const ANSI_BOLD: &str = "\x1b[1m";

mod argparse;
use argparse::*;

fn main() {
    if has_help() || args().collect::<Vec<String>>().len() == 1 {
        help_msg();
    }
    match get_file_arg() {
        Some(i) => match fs::read_to_string(i.clone()) {
            Ok(f) => {
                if has_check() {
                    check_file(f, i);
                } else if has_parse() {
                    match parse(&f) {
                        Ok(p) => {
                            if has_json() {
                                println!("{}", p.to_json());
                            } else {
                                println!("{}", render_parsed_file(p));
                            }
                            exit(0);
                        }
                        Err(r) => {
                            println!("{}", render_error_report(r, f, i));
                            exit(1);
                        }
                    };
                } else {
                    run(&f, Some(&i));
                }
            }
            Err(e) => {
                eprintln!("Couldn't read file: {}", e);
                exit(1);
            }
        },
        None => {
            eprintln!("Requires file argument.");
            exit(1);
        }
    }
}

fn render_parsed_file(p: ParsedFile) -> String {
    let mut output = String::new();

    // Put all of the newlines in the right place
    for ch in format!("{p:?}").chars().map(|ch| String::from(ch)) {
        output += &ch;
        match ch.as_str() {
            "," => output += "\n",
            "{" | "[" => output += "\n",
            "]" | "}" => output.insert_str(output.len() - 2, "\n"),
            _ => (),
        };
    }

    let mut current_indent = 0;
    let mut indented = String::new();
    for mut l in output.lines() {
        loop {
            if l.starts_with(" ") {
                l = l.strip_prefix(" ").unwrap();
            } else {
                break;
            }
        }

        loop {
            if l.ends_with(" ") {
                l = l.strip_suffix(" ").unwrap();
            } else {
                break;
            }
        }

        if l.contains("{") {
            current_indent += 4;
        }
        if l.contains("[") {
            current_indent += 4;
        }

        if l.contains("}") {
            current_indent -= 4;
        }
        if l.contains("]") {
            current_indent -= 4;
        }

        indented += &l
            .replace("{", "")
            .replace("[", "")
            .replace("}", "")
            .replace("]", "")
            .replace(",", "")
            .replace(":", "");

        if l.len() > 2 {
            indented += "\n";
            (0..current_indent).for_each(|_| indented += " ");
        }
    }

    indented
}

fn render_error_report(r: ErrorReport, path: String, inp: String) -> String {
    let mut output = String::new();
    for e in r.errors.iter().rev() {
        output += &render_error(&inp, e, Some(&path));
    }

    output += &format!(
        "{}{}    Error:{} Failed to parse {} due to {} error{}",
        ANSI_RED,
        ANSI_BOLD,
        ANSI_RESET,
        path,
        r.errors.len(),
        if r.errors.len() == 1 { "" } else { "s" }
    );
    output
}

fn check_file(inp: String, path: String) -> ! {
    let start_t = Instant::now();
    match parse(&inp) {
        Ok(_) => {
            println!(
                "{}{}    Finished{} target in {}ms.",
                ANSI_GREEN,
                ANSI_BOLD,
                ANSI_RESET,
                start_t.elapsed().as_millis(),
            );
            exit(0);
        }
        Err(r) => {
            for e in r.errors.iter().rev() {
                println!("{}", render_error(&inp, e, Some(&path)));
            }
            println!(
                "{}{}    Finished{} check in {}ms",
                ANSI_RED,
                ANSI_BOLD,
                ANSI_RESET,
                start_t.elapsed().as_millis()
            );

            println!(
                "{}{}    Error:{} Failed to parse {} due to {} error{}",
                ANSI_RED,
                ANSI_BOLD,
                ANSI_RESET,
                path,
                r.errors.len(),
                if r.errors.len() == 1 { "" } else { "s" }
            );
            exit(1);
        }
    }
}

fn help_msg() -> ! {
    println!(
        " QQML v1.0 (c) 2023 'MrPiggyPegasus'

usage: qqml [OPTIONS] <FILE>

OPTIONS:
    -c --check      Validate the QQML source file.

    -h --help       Print this help message and
                    exit.

    -V --version    Print version information and
                    exit.

    -p --parse      Attempt to parse the file, if
                    succesful will then print the
                    parsed data.

    -j --json       Output any parsing data in a
                    JSON format

More information about the QQML language
and its related tooling is available at
https://github.com/MrPiggyPegasus/qqml "
    );
    exit(1);
}
