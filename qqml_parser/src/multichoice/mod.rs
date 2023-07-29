mod data;
mod parser;

#[cfg(test)]
mod tests;

pub use self::data::*;
pub use self::parser::parse_multichoice;
pub use self::parser::parse_multichoice_answer;
