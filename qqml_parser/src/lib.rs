mod error;
mod multichoice;
mod parser;

pub use self::error::Error;
pub use self::multichoice::MultichoiceAnswer;
pub use self::multichoice::MultichoiceData;

pub use crate::parser::parse;

pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}

mod utils {

    pub const WHITESPACE_CHARS: [u8; 4] = [b' ', b'\n', b'\t', b'\r'];

    pub fn is_empty_str<S: Into<String>>(str: S) -> bool {
        let str: String = str.into();

        for ch in str.bytes() {
            if !WHITESPACE_CHARS.contains(&ch) {
                return false
            }
        }
        true
    }
}
