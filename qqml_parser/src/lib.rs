mod error;
mod multichoice;
mod parser;

pub use crate::multichoice::*;
pub use crate::parser::parse;

#[cfg(test)]
mod test;

use multichoice::MultichoiceData;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
