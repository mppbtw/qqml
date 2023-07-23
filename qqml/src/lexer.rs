use super::error::{Error, Result};
use super::token::{Token, KEYWORDS};

const WHITESPACE_CHARS: [u8; 4] = [b' ', b'\n', b'\r', b'\t'];

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
            return Err(Error::NonAsciiInput);
        }

        let mut lexer = Self {
            input,
            ..Default::default()
        };
        lexer.read_char();
        Ok(lexer)
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0;
        } else {
            self.ch = self.input.as_bytes()[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.scran_whitespace();
        let tok: Token = match self.ch {
            b'=' => Token::Equal,
            b'*' => Token::Asterisk,
            b';' => Token::Semicolon,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'[' => Token::LSquare,
            b']' => Token::RSquare,
            b',' => Token::Comma,
            b':' => Token::Colon,
            b'/' => Token::Divide,
            b'+' => Token::Plus,
            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GThanOrEqual
                } else {
                    Token::GThan
                }
            },
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LThanOrEqual
                } else {
                    Token::LThan
                }
            },
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NEqual
                } else {
                    Token::Illegal
                }
            }
            b'-' => {
                if self.peek_char() == b'>' {
                    self.read_char();
                    Token::RArrow
                } else {
                    Token::Subtract
                }
            }
            0 => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_ident();
                    lookup_ident(ident)
                } else if is_digit(self.ch) {
                    Token::Number(self.read_number())
                } else if is_quote(self.ch) {
                    Token::Literal(self.read_literal())
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
    }

    fn read_number(&mut self) -> usize {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }
        let num = self.input[pos..self.position].to_owned().parse().unwrap();
        num
    }

    fn read_literal(&mut self) -> String {
        let pos = self.position;
        let mut quotes_found = 0;
        while quotes_found < 2 {
            if is_quote(self.ch) {
                quotes_found += 1;
            }
            self.read_char();
        }
        self.input[pos+1..self.position].to_owned()
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.input[pos..self.position].to_owned()
    }

    fn scran_whitespace(&mut self) {
        while WHITESPACE_CHARS.contains(&self.ch) {
            self.read_char();
        }
    }

    fn peek_char(&self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        }
        self.input.bytes().collect::<Vec<u8>>()[self.read_position]
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            input: "".to_owned(),
            position: 0,
            read_position: 0,
            ch: 0,
        }
    }
}

fn lookup_ident(ident: String) -> Token {
    match KEYWORDS.get(&ident) {
        Some(i) => i.clone(),
        None => Token::Ident(ident),
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == b'_'
}

fn is_quote(ch: u8) -> bool {
    ch == b'\'' || ch == b'"'
}

pub fn is_digit(ch: u8) -> bool {
    (b'0'..=b'9').contains(&ch)
}
