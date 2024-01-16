//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TokenData {
    pub col:  usize,
    pub line: usize, // dunno why you would need this but its an option i guess
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    LSquirly(TokenData),
    RSquirly(TokenData),
    LSquare(TokenData),
    RSquare(TokenData),
    String(TokenData, String),
    Number(TokenData, usize),
    True(TokenData),
    False(TokenData),
    Colon(TokenData),
    Eof(TokenData),
    Illegal(TokenData),
    Comma(TokenData),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lexer {
    ch:            u8,
    read_position: usize,
    position:      usize,
    input:         String,
    pub tok_count: usize,
}
impl Lexer {
    pub fn new<S: Into<String>>(input: S) -> Lexer {
        let mut l = Lexer {
            ch:            0,
            read_position: 0,
            position:      0,
            tok_count:     0,
            input:         input.into(),
        };
        l.read_char();
        l
    }

    pub fn next_token(&mut self) -> Token {
        self.munch_and_crunch_whitespace();
        self.tok_count += 1;
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
            self.read_char();
        }
        let num = self.input[pos..self.position]
            .to_owned()
            .parse::<usize>()
            .unwrap();
        self.read_position -= 1;
        self.position -= 1;
        Token::Number(dat, num)
    }

    fn read_keyword(&mut self) -> Token {
        let pos = self.position;
        while self.ch.is_ascii_alphabetic() {
            self.read_char();
        }
        let str = self.input[pos..self.position].to_string();
        self.position -= 1;
        self.read_position -= 1;
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
        Token::String(dat, self.input[pos..self.position].to_string())
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
        while [b' ', b'\t', b'\n', b'\r'].contains(&self.ch) {
            self.read_char();
        }
    }

    fn get_token_data(&self) -> TokenData {
        TokenData { line: 0, col: 0 }
    }
}
    }
}
