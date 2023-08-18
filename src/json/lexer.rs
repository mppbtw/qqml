#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TokenData {
    col: usize,
    line: usize, // dunno why you would need this but its an option i guess
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LSquirly(TokenData),
    RSquirly(TokenData),
    LSquare(TokenData),
    RSquare(TokenData),
    String(TokenData, String),
    Number(TokenData, i128),
    True(TokenData),
    False(TokenData),
    Colon(TokenData),
    Eof(TokenData),
    Illegal(TokenData),
    Comma(TokenData),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lexer {
    ch: u8,
    read_position: usize,
    position: usize,
    input: String,
    pub tok_count: usize,
}
impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Lexer {
        let mut l = Lexer {
            ch: 0,
            read_position: 0,
            position: 0,
            tok_count: 0,
            input: input.into(),
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.munch_and_crunch_whitespace();
        let tok = match self.ch {
            b'{' => Token::LSquirly(self.get_token_data()),
            b'}' => Token::RSquirly(self.get_token_data()),
            b':' => Token::Colon(self.get_token_data()),
            b'[' => Token::LSquare(self.get_token_data()),
            b']' => Token::RSquare(self.get_token_data()),
            b',' => Token::Comma(self.get_token_data()),
            0 => Token::Eof(self.get_token_data()),
            b'"' => self.read_string(),
            _ => {
                if self.ch.is_ascii_alphabetic() {
                    self.read_keyword()
                } else if self.ch.is_ascii_digit() {
                    self.read_number()
                } else {
                    Token::Illegal(self.get_token_data())
                }
            }
        };
        self.read_char();
        tok
    }

    fn read_number(&mut self) -> Token {
        let pos = self.position;
        let dat = self.get_token_data();
        while self.ch.is_ascii_digit() {
            self.read_char()
        }
        Token::Number(dat, self.input[pos..self.position].parse::<i128>().unwrap())
    }

    fn read_keyword(&mut self) -> Token {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        let str = self.input[pos..self.position].to_string();
        if str == "false" {
            Token::False(self.get_token_data())
        } else if str == "true" {
            Token::True(self.get_token_data())
        } else {
            Token::Illegal(self.get_token_data())
        }
    }

    fn read_string(&mut self) -> Token {
        let dat = self.get_token_data();
        self.read_char();
        let pos = self.position;
        while self.ch != b'"' {
            self.read_char();
        }
        Token::String(
            self.get_token_data(),
            self.input[pos..self.position].to_string(),
        )
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    fn munch_and_crunch_whitespace(&mut self) {
        while vec![b' ', b'\t', b'\n', b'\r'].contains(&self.ch) {
            self.read_char();
        }
    }

    fn get_token_data(&self) -> TokenData {
        TokenData {
            col: self.position,
            line: {
                self.input
                    .as_bytes()
                    .iter()
                    .map(|b| if *b == b'\n' { 1 } else { 0 })
                    .sum()
            },
        };
        TokenData::default()
    }
}
