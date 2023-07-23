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
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NEqual
                } else {
                    Token::Illegal
                }
            }
            b'*' => Token::Asterisk,
            b';' => Token::Semicolon,
            b'>' => Token::GThan,
            b'<' => Token::LThan,
            b'(' => Token::LParen,
            b')' => Token::RParen,
            b'{' => Token::LSquirly,
            b'}' => Token::RSquirly,
            b'[' => Token::LSquare,
            b']' => Token::RSquare,
            b',' => Token::Comma,
            b':' => Token::Colon,
            b'-' => {
                if self.peek_char() == b'>' {
                    self.read_char();
                    Token::RArrow
                } else {
                    Token::Illegal
                }
            }
            0 => Token::Eof,
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_ident();
                    dbg!(&ident);
                    lookup_ident(ident)
                } else {
                    Token::Illegal
                }
            }
        };

        self.read_char();
        tok
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
            return 0
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

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == b'_'
}

fn lookup_ident(ident: String) -> Token {
    match KEYWORDS.get(&ident) {
        Some(i) => i.clone(),
        None => Token::Ident(ident),
    }
}
