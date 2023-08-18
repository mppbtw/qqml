pub mod diagnostics;
pub mod end_screen;
pub mod exit;
pub mod render;
pub mod repl;
pub mod state;

// Example TUI:

// ~/.qpm/local/gcse/geography/countries.qqml
// QQML Version 1.0.0, press ? for help
//
//                      <-(23/25)-> // h and l to move between questions
//
// Where is France? (2)
//
// Europe   <    // j and k to select the answer and space
// Asia          // to confirm (will become highlighted)
// Africa
//
// Hints (3 remaining): // This is the number of hints the question has left
//  France is west of Germany which is to the north of Italy
//
//  France is not in Africa

// After answering the question:

// ~/.qpm/local/gcse/geography/countries.qqml
// QQML Version 1.0.0, press ? for help
//
//                      <-(23/25)-> // h and l to move between questions
// hints: 5 // How many hints the player has left in total
//
// Where is France? (2)
//
// Europe   France is located in western Europe        // Word wrap
//          bordering other countries such as Germany
//          and Spain.
//
// Asia     France is not located in Asia.
// Africa
//
// Hints (0 remaining): // Hints are no longer available
//  France is west of Germany which is to the north of Italy
//
//  France is not in Africa