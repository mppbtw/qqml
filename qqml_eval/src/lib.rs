mod error;
mod tui;
mod quiz;

use quiz::Quiz;
use qqml_parser::MultichoiceData;
use std::process::exit;

pub use error::Error;

// Example TUI:

// < (23/25) > // h and l to move between questions
//
// Where is France? (2)
//
// Europe   <    // j and k to select the answer
// Asia
// Africa
