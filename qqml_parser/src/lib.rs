mod error;
mod multichoice;
mod parser;
mod warning;

#[cfg(test)]
mod test_correct;
#[cfg(test)]
mod test_error;

pub use self::error::Error;
pub use self::multichoice::MultichoiceAnswer;
pub use self::multichoice::MultichoiceData;
pub use self::parser::parse;
pub use self::parser::ParsedFile;
pub use self::warning::Warning;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
