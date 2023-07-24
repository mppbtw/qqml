mod error;
mod multichoice;
mod parser;

pub use crate::parser::parse;
pub use crate::multichoice::*;

#[cfg(test)]
mod test;

use multichoice::MultichoiceData;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
