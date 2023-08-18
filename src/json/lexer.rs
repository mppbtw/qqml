use crate::eval::state::State;

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
    Literal(TokenData),
    Number(TokenData),
    True(TokenData),
    False(TokenData),
    Colon(TokenData),
    Eof(TokenData),
    Illegal(TokenData),
    Comma(TokenData),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonError {
    UnexpectedEof(Token),
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Lexer {
    ch: u8,
    read_position: usize,
    position: usize,
    input: String,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            ch: 0,
            read_position: 0,
            position: 0,
            input,
        };
        l.read_char();
        l
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input.as_bytes()[self.read_position]
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn get_token_data(&self) -> TokenData {
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

    pub fn next_token(&mut self) -> Token {
        let tok = match self.ch {
            b'{' => Token::LSquirly(self.get_token_data()),
            b'}' => Token::RSquirly(self.get_token_data()),
            b':' => Token::Colon(self.get_token_data()),
            b'[' => Token::LSquare(self.get_token_data()),
            b']' => Token::RSquare(self.get_token_data()),
            b',' => Token::Comma(self.get_token_data()),
            0 => Token::Eof(self.get_token_data()),
            _ => Token::Illegal(self.get_token_data()),
        };
        self.read_char();
        tok
    }
}
