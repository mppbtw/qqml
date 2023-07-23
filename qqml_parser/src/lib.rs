mod error;
mod multichoice;
mod parser;

#[cfg(test)]
mod test;

use multichoice::MultichoiceData;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}
