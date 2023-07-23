use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Number(usize),
    Literal(String),

    Plus,
    Subtract,
    GThanOrEqual,
    LThanOrEqual,
    Divide,

    RArrow,
    Equal,  // =
    NEqual, // !=
    Comma,
    Semicolon,
    Colon,
    Asterisk,

    LParen,
    RParen,
    LSquirly,
    RSquirly,
    LSquare,
    RSquare,
    LThan,
    GThan,

    Ask,
    Multichoice,
    String,
    Calculation,
    Inputs,
}
impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        if !matches!(self, other) {
            return false;
        }
        match self {
            Self::Number(sn) => {
                match other {
                    Self::Number(n) => n == sn,
                    _ => false,
                }
            }
            Self::Literal(sl) => {
                match other {
                    Self::Literal(l) => l == sl,
                    _ => false,
                }
            }
            _ => true
        }
    }
}

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, Token> = {
        let mut m = HashMap::new();
        m.insert("ask".into(), Token::Ask);
        m.insert("multichoice".into(), Token::Multichoice);
        m.insert("string".into(), Token::String);
        m.insert("calculation".into(), Token::Calculation);
        m.insert("inputs".into(), Token::Inputs);
        m
    };
}
