use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Number(usize),
    Literal(String),

    RArrow,
    Equal,    // =
    NEqual,   // !=
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
    fn eq(&self, _: &Self) -> bool {
        matches!(self, _)
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
