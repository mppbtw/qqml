//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

use super::token::get_keywords_map;
use super::token::Token;
use super::token::TokenData;

const WHITESPACE_CHARS: [u8; 4] = [b' ', b'\n', b'\r', b'\t'];

#[derive(Debug, Clone, PartialEq)]
pub enum LexerError {
    UnterminatedStringError(TokenData),
    IntegerTooLarge(TokenData),
}

#[derive(Debug, Clone)]
pub struct Lexer {
    input:         String,
    position:      usize,
    read_position: usize,
    ch:            u8,

    // For attaching token metadata
    line_count:   usize,
    last_newline: usize,

    /// The position of the lexer when it began to
    /// read a token (needed for getting the first
    /// char of a token).
    starting_position: usize,
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

    pub fn get_lexer_data(&self) -> LexerData {
        LexerData {
            position:          self.position,
            starting_position: self.starting_position,
            last_newline:      self.last_newline,
            line_count:        self.line_count,
            read_position:     self.read_position,
            ch:                self.ch,
        }
    }

    pub fn from_lexer_data<S: Into<String>>(input: S, dat: LexerData) -> Self {
        Self {
            input:             input.into(),
            starting_position: dat.starting_position,
            last_newline:      dat.last_newline,
            line_count:        dat.line_count,
            position:          dat.position,
            ch:                dat.ch, // It calls read_char anyway
            read_position:     dat.read_position,
        }
    }

    pub fn get_input(&self) -> &String {
        &self.input
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
        let token_len = self.position - self.starting_position;
        if self.line_count == 0 {
            TokenData {
                col:  self.position - token_len,
                line: self.line_count,
            }
        } else {
            TokenData {
                col:  ((self.position - self.last_newline) - 1) - token_len,
                line: self.line_count,
            }
        }
    }

    pub fn next_token(&mut self) -> Result<Token, LexerError> {
        self.scran_whitespace();
        while self.ch == b'#' {
            self.skip_until_newline();
            self.scran_whitespace();
        }

        self.starting_position = self.position;
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
                    let found = lookup_ident(self.read_ident());
                    found.with_different_data(self.get_token_data())
                } else if is_digit(self.ch) {
                    Token::Number(self.get_token_data(), self.read_number()?)
                } else if is_quote(self.ch) {
                    Token::Literal(self.get_token_data(), self.read_literal()?)
                } else {
                    Token::Illegal(self.get_token_data())
                }
            }
        };

        self.read_char();
        Ok(tok)
    }

    fn skip_until_newline(&mut self) {
        while self.ch != b'\n' && self.ch != 0 {
            self.read_char();
        }
    }

    fn read_number(&mut self) -> Result<usize, LexerError> {
        let pos = self.position;
        let dat = self.get_token_data();
        while is_digit(self.ch) {
            self.read_char();
        }
        let num = match self.input[pos..self.position].to_owned().parse::<usize>() {
            Ok(n) => n,
            Err(_) => return Err(LexerError::IntegerTooLarge(dat)),
        };
        self.read_position -= 1;
        self.position -= 1;
        Ok(num)
    }

    fn read_literal(&mut self) -> Result<String, LexerError> {
        let pos = self.position;
        let dat = self.get_token_data();
        let mut quotes_found = 0;
        while quotes_found < 2 {
            if self.ch == 0 {
                return Err(LexerError::UnterminatedStringError(dat));
            }
            if self.ch == b'\n' {
                self.line_count += 1;
                self.last_newline = self.position;
            }
            if is_quote(self.ch) {
                quotes_found += 1;
            }
            self.read_char();
        }
        let literal = self.input[pos + 1..self.position - 1].to_owned();
        self.position -= 1;
        self.read_position -= 1;
        Ok(literal)
    }

    fn read_ident(&mut self) -> String {
        let pos = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }
        self.read_position -= 1;
        self.input[pos..self.position].to_owned()
    }

    fn scran_whitespace(&mut self) {
        while WHITESPACE_CHARS.contains(&self.ch) {
            if self.ch == b'\n' {
                self.line_count += 1;
                self.last_newline = self.position;
            }
            if self.ch == b'\r' {
                self.last_newline = self.position;
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
            input:             "".to_owned(),
            position:          0,
            read_position:     0,
            ch:                0,
            line_count:        0,
            last_newline:      0,
            starting_position: 0,
        }
    }
}

fn lookup_ident(ident: String) -> Token {
    match get_keywords_map().get(&ident) {
        Some(i) => i.clone(),
        None => Token::Ident(TokenData::default(), ident),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct LexerData {
    position:      usize,
    read_position: usize,

    // For attaching token metadata
    line_count:   usize,
    last_newline: usize,

    /// The position of the lexer when it began to
    /// read a token (needed for getting the first
    /// char of a token).
    starting_position: usize,

    ch: u8,
}

fn is_letter(ch: u8) -> bool {
    ch.is_ascii_uppercase() || ch.is_ascii_lowercase() || ch == b'_'
}

fn is_quote(ch: u8) -> bool {
    ch == b'\'' || ch == b'"'
}

fn is_digit(ch: u8) -> bool {
    ch.is_ascii_digit()
}
