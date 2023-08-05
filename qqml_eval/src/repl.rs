use std::process::exit;
use crate::*;
use qqml_parser::*;

pub fn run(input: String) -> ! {
    let parsed = parse(input).unwrap();
    loop { }
    exit(0)
}
