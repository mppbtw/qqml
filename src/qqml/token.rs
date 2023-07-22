pub enum Token {
    Illegal,
    Eof,
    Ident,
    Number,
    Equal,
    Comma,
    Semicolon,
    Colon,

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
