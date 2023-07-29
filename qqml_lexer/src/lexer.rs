use crate::token::Token;
use crate::token::TokenData;
use crate::token::KEYWORDS;

const WHITESPACE_CHARS: [u8; 4] = [b' ', b'\n', b'\r', b'\t'];

#[allow(unused)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
    line_count: usize,
}

#[allow(unused)]
impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Self {
        let input: String = input.into();

        let mut lexer = Self {
            input,
            ..Default::default()
        };
        lexer.read_char();
        lexer
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

    fn get_token_data(&self) -> TokenData {
        TokenData {
            col: self.position,
            line: self.line_count + 1,
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.scran_whitespace();
        let tok: Token = match self.ch {
            b'=' => Token::Equal(self.get_token_data()),
            b'*' => Token::Asterisk(self.get_token_data()),
            b';' => Token::Semicolon(self.get_token_data()),
            b'(' => Token::LParen(self.get_token_data()),
            b')' => Token::RParen(self.get_token_data()),
            b'{' => Token::LSquirly(self.get_token_data()),
            b'}' => Token::RSquirly(self.get_token_data()),
            b'[' => Token::LSquare(self.get_token_data()),
            b']' => Token::RSquare(self.get_token_data()),
            b',' => Token::Comma(self.get_token_data()),
            b':' => Token::Colon(self.get_token_data()),
            b'/' => Token::Divide(self.get_token_data()),
            b'+' => Token::Plus(self.get_token_data()),

            b'>' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::GThanOrEqual(self.get_token_data())
                } else {
                    Token::GThan(self.get_token_data())
                }
            }
            b'<' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::LThanOrEqual(self.get_token_data())
                } else {
                    Token::LThan(self.get_token_data())
                }
            }
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NEqual(self.get_token_data())
                } else {
                    Token::Illegal(self.get_token_data())
                }
            }
            b'-' => {
                if self.peek_char() == b'>' {
                    self.read_char();
                    Token::RArrow(self.get_token_data())
                } else {
                    Token::Subtract(self.get_token_data())
                }
            }
            0 => Token::Eof(self.get_token_data()),
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_ident();
                    let found = lookup_ident(ident);
                    match found {
                        Token::Ident(_, i) => Token::Ident(self.get_token_data(), i),
                        _ => found,
                    }
                } else if is_digit(self.ch) {
                    Token::Number(self.get_token_data(), self.read_number())
                } else if is_quote(self.ch) {
                    Token::Literal(self.get_token_data(), self.read_literal())
                } else {
                    Token::Illegal(self.get_token_data())
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
        self.read_position -= 1;
        self.position -= 1;
        num
    }

    fn read_literal(&mut self) -> String {
        let pos = self.position;
        let mut quotes_found = 0;
        while quotes_found < 2 {
            if is_quote(self.ch) {
                quotes_found += 1;
            }
            if self.ch == 0 {
                break;
            }
            self.read_char();
        }
        let literal = self.input[pos + 1..self.position - 1].to_owned();
        self.position -= 1;
        self.read_position -= 1;
        literal
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
            if self.ch == b'\n' {
                self.line_count += 1;
            }
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
            line_count: 0,
        }
    }
}

fn lookup_ident(ident: String) -> Token {
    match KEYWORDS.get(&ident) {
        Some(i) => i.clone(),
        None => Token::Ident(TokenData::default(), ident),
    }
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == b'_'
}

fn is_quote(ch: u8) -> bool {
    ch == b'\'' || ch == b'"'
}

fn is_digit(ch: u8) -> bool {
    (b'0'..=b'9').contains(&ch)
}
