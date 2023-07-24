mod error;
mod multichoice;
mod parser;

pub use self::multichoice::MultichoiceData;
pub use self::multichoice::MultichoiceAnswer;
pub use self::error::Error;

pub use crate::parser::parse;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
