use std::collections::HashMap;
use std::fmt;

pub fn get_keywords_map() -> HashMap<String, Token> {
    let mut m = HashMap::new();
    m.insert("ask".into(), Token::Ask(TokenData::default()));
    m.insert(
        "multichoice".into(),
        Token::Multichoice(TokenData::default()),
    );
    m.insert("string".into(), Token::String(TokenData::default()));
    m.insert(
        "calculation".into(),
        Token::Calculation(TokenData::default()),
    );
    m.insert("inputs".into(), Token::Inputs(TokenData::default()));
    m.insert("hints".into(), Token::Hints(TokenData::default()));
    m
}

#[derive(Clone, Debug, Default)]
pub struct TokenData {
    /// STARTS AT ZERO!
    pub col: usize,

    /// STARTS AT ZERO!
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
    UnterminatedLiteral(TokenData),

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
            Self::UnterminatedLiteral(_) => "UnterminatedLiteral",
        };
        write!(f, "{}", name)
    }
}

impl Token {
    pub fn with_different_data(&self, new_data: TokenData) -> Token {
        match self {
            Self::Literal(_, v) => Self::Literal(new_data, v.to_owned()),
            Self::Number(_, v) => Self::Number(new_data, *v),
            Self::Ident(_, v) => Self::Ident(new_data, v.to_owned()),
            Self::Eof(_) => Self::Eof(new_data),
            Self::RArrow(_) => Self::RArrow(new_data),
            Self::Semicolon(_) => Self::Semicolon(new_data),
            Self::Divide(_) => Self::Divide(new_data),
            Self::Ask(_) => Self::Ask(new_data),
            Self::NEqual(_) => Self::NEqual(new_data),
            Self::Equal(_) => Self::Equal(new_data),
            Self::String(_) => Self::String(new_data),
            Self::Plus(_) => Self::Plus(new_data),
            Self::Asterisk(_) => Self::Asterisk(new_data),
            Self::LThan(_) => Self::LThan(new_data),
            Self::GThan(_) => Self::GThan(new_data),
            Self::RParen(_) => Self::RParen(new_data),
            Self::LParen(_) => Self::LParen(new_data),
            Self::RSquare(_) => Self::RSquare(new_data),
            Self::LSquare(_) => Self::LSquare(new_data),
            Self::RSquirly(_) => Self::RSquirly(new_data),
            Self::LSquirly(_) => Self::LSquirly(new_data),
            Self::Subtract(_) => Self::Subtract(new_data),
            Self::Illegal(_) => Self::Illegal(new_data),
            Self::Inputs(_) => Self::Inputs(new_data),
            Self::GThanOrEqual(_) => Self::GThanOrEqual(new_data),
            Self::LThanOrEqual(_) => Self::LThanOrEqual(new_data),
            Self::Comma(_) => Self::Comma(new_data),
            Self::Colon(_) => Self::Colon(new_data),
            Self::Multichoice(_) => Self::Multichoice(new_data),
            Self::Hints(_) => Self::Hints(new_data),
            Self::Calculation(_) => Self::Calculation(new_data),
            Self::UnterminatedLiteral(_) => Self::UnterminatedLiteral(new_data),
        }
    }
    pub fn get_data(&self) -> &TokenData {
        match self {
            Self::Literal(d, _) => d,
            Self::Number(d, _) => d,
            Self::Ident(d, _) => d,
            Self::Eof(d) => d,
            Self::RArrow(d) => d,
            Self::Semicolon(d) => d,
            Self::Divide(d) => d,
            Self::Ask(d) => d,
            Self::NEqual(d) => d,
            Self::Equal(d) => d,
            Self::String(d) => d,
            Self::Plus(d) => d,
            Self::Asterisk(d) => d,
            Self::LThan(d) => d,
            Self::GThan(d) => d,
            Self::RParen(d) => d,
            Self::LParen(d) => d,
            Self::RSquare(d) => d,
            Self::LSquare(d) => d,
            Self::RSquirly(d) => d,
            Self::LSquirly(d) => d,
            Self::Subtract(d) => d,
            Self::Illegal(d) => d,
            Self::Inputs(d) => d,
            Self::GThanOrEqual(d) => d,
            Self::LThanOrEqual(d) => d,
            Self::Comma(d) => d,
            Self::Colon(d) => d,
            Self::Multichoice(d) => d,
            Self::Hints(d) => d,
            Self::Calculation(d) => d,
            Self::UnterminatedLiteral(d) => d,
        }
    }
}
