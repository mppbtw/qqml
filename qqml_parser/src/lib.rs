mod error;
mod multichoice;
mod parser;

pub use multichoice::MultichoiceData;
pub use multichoice::MultichoiceAnswer;
pub use crate::parser::parse;
pub use error::Error;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
