use lazy_static::lazy_static;
use std::{collections::HashMap, fmt};

lazy_static! {
    pub static ref KEYWORDS: HashMap<String, Token> = {
        let mut m = HashMap::new();
        m.insert("ask".into(), Token::Ask(TokenData::default()));
        m.insert("multichoice".into(), Token::Multichoice(TokenData::default()));
        m.insert("string".into(), Token::String(TokenData::default()));
        m.insert("calculation".into(), Token::Calculation(TokenData::default()));
        m.insert("inputs".into(), Token::Inputs(TokenData::default()));
        m.insert("hints".into(), Token::Hints(TokenData::default()));
        m
    };
}

#[derive(Clone, Debug, Default)]
pub struct TokenData {
    pub col: usize,
    pub line: usize,
}
impl PartialEq for TokenData {
    fn eq(&self, _: &Self) -> bool {
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Illegal(TokenData),
    Eof(TokenData),

    Ident(TokenData, String),
    Number(TokenData, usize),
    Literal(TokenData, String),

    Plus(TokenData),
    Subtract(TokenData),
    GThanOrEqual(TokenData),
    LThanOrEqual(TokenData),
    Divide(TokenData),

    RArrow(TokenData),
    Equal(TokenData),
    NEqual(TokenData),
    Comma(TokenData),
    Semicolon(TokenData),
    Colon(TokenData),
    Asterisk(TokenData),

    LParen(TokenData),
    RParen(TokenData),
    LSquirly(TokenData),
    RSquirly(TokenData),
    LSquare(TokenData),
    RSquare(TokenData),
    LThan(TokenData),
    GThan(TokenData),

    Ask(TokenData),
    Multichoice(TokenData),
    String(TokenData),
    Calculation(TokenData),
    Inputs(TokenData),
    Hints(TokenData),
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            Self::Literal(_, _) => "Literal",
            Self::Number(_, _) => "Number",
            Self::Ident(_, _) => "Identifier",
            Self::Eof(_) => "EOF",
            Self::RArrow(_) => "RightArrow",
            Self::Semicolon(_) => "Semicolon",
            Self::Divide(_) => "Divide",
            Self::Ask(_) => "Ask",
            Self::NEqual(_) => "NotEqual",
            Self::Equal(_) => "Equal",
            Self::String(_) => "String",
            Self::Plus(_) => "Plus",
            Self::Asterisk(_) => "Asterisk",
            Self::LThan(_) => "LessThan",
            Self::GThan(_) => "GreaterThan",
            Self::RParen(_) => "RightParenthese",
            Self::LParen(_) => "LeftParenthese",
            Self::RSquare(_) => "RightSquareBracket",
            Self::LSquare(_) => "LeftSquareBracket",
            Self::RSquirly(_) => "RightSquirlyBracket",
            Self::LSquirly(_) => "LeftSquirlyBracket",
            Self::Subtract(_) => "Subtract",
            Self::Illegal(_) => "Illegal",
            Self::Inputs(_) => "Inputs",
            Self::GThanOrEqual(_) => "GreatherThanOrEqual",
            Self::LThanOrEqual(_) => "LessThanOrEqual",
            Self::Comma(_) => "Comma",
            Self::Colon(_) => "Colon",
            Self::Multichoice(_) => "Multichoice",
            Self::Hints(_) => "Hints",
            Self::Calculation(_) => "Calculation",
        };
        write!(f, "{}", name)
    }
}

impl Into<Vec<Token>> for Token {
    fn into(self) -> Vec<Token> {
        vec![self]
    }
}
