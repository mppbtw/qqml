#[allow(unused)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
}

impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let input: String = input.into();

        if !input.is_ascii() {
            panic!("QQML only supports ASCII encoding.")
        }

        return Self {
            input,
            ..Default::default()
        }
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
