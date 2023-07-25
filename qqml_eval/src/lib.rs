mod tui;


use std::process::exit;
use qqml_parser::MultichoiceData;


// Example TUI:

// < (23/25) > // h and l to move between questions
//
// Where is France? (2)
//
// Europe   <    // j and k to select the answer
// Asia
// Africa

pub fn run(inp: Vec<MultichoiceData>) -> ! {
    exit(0);
}
