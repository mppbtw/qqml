#[derive(Debug)]
pub enum Token {
    Illegal,
    Eof,

    Ident(String),
    Number(usize),

    Equal,
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
}
impl PartialEq for Token {
    fn eq(&self, _: &Self) -> bool {
        matches!(self, _)
    }
}
