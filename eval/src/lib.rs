#[cfg(test)]
mod test;

mod error;
#[allow(unused)]
mod render;
mod section;
mod utils;

pub use error::Error;

// Example TUI:

// ~/.yarr/sections/geography.qqml
//
// <-(23/25)-> // h and l to move between questions
//
// Where is France? (2)
//
// Europe   <    // j and k to select the answer
// Asia
// Africa
//
// Hints: // Will appear as 'Hint:' when only one hint is present
//  France is west of Germany which is to the north of Italy
//  France is not in Africa
