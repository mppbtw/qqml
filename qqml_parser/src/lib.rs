mod error;
mod multichoice;

pub use self::error::Error;
pub use self::multichoice::MultichoiceAnswer;
pub use self::multichoice::MultichoiceData;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
