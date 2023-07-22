use crate::qqml::error::{Result, Error};
#[allow(unused)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

#[allow(unused)]
impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Result<Self> {
        let input: String = input.into();

        if !input.is_ascii() {
            return Err(Error::NonAsciiInput)
        }

        Ok(Self {
            input,
            ..Default::default()
        })
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            input: "".to_owned(),
            position: 0,
            read_position: 1,
            ch: 0,
        }
    }
}
