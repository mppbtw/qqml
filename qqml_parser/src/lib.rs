mod error;
mod multichoice;
mod parser;

pub use parser::parse;
pub use multichoice::*;

#[cfg(test)]
mod test;

use multichoice::MultichoiceData;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
