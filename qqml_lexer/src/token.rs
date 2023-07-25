use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, Token> = {
        let mut m = HashMap::new();
        m.insert("ask".into(), Token::Ask);
        m.insert("multichoice".into(), Token::Multichoice);
        m.insert("string".into(), Token::String);
        m.insert("calculation".into(), Token::Calculation);
        m.insert("inputs".into(), Token::Inputs);
        m.insert("hints".into(), Token::Hints);
        m
    };
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
    Hints,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Eof => "EOF",
            Self::RArrow => "RightArrow",
            Self::Semicolon => "Semicolon",
            Self::Literal(_) => "Literal",
            Self::Divide => "Divide",
            Self::Ask => "Ask",
            Self::NEqual => "NotEqual",
            Self::Equal => "Equal",
            Self::String => "String",
            Self::Plus => "Plus",
            Self::Asterisk => "Asterisk",
            Self::LThan => "LessThan",
            Self::GThan => "GreaterThan",
            Self::Number(_) => "Number",
            Self::RParen => "RightParenthese",
            Self::LParen => "LeftParenthese",
            Self::RSquare => "RightSquareBracket",
            Self::LSquare => "LeftSquareBracket",
            Self::RSquirly => "RightSquirlyBracket",
            Self::LSquirly => "LeftSquirlyBracket",
            Self::Subtract => "Subtract",
            Self::Illegal => "Illegal",
            Self::Inputs => "Inputs",
            Self::GThanOrEqual => "GreatherThanOrEqual",
            Self::LThanOrEqual => "LessThanOrEqual",
            Self::Comma => "Comma",
            Self::Ident(_) => "Identifier",
            Self::Colon => "Colon",
            Self::Multichoice => "Multichoice",
            Self::Hints => "Hints",
            Self::Calculation => "Calculation",
        };
        write!(f, "{}", name)
    }
}

impl Into<Vec<Token>> for Token {
    fn into(self) -> Vec<Token> {
        vec![self]
    }
}
