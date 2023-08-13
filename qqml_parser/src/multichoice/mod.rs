mod data;
mod parse_answer;
mod parser;

#[cfg(test)]
mod tests;

pub use self::data::*;
pub use self::parser::parse_multichoice;
pub use self::parse_answer::parse_multichoice_answer;
