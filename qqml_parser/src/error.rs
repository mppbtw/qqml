use qqml_lexer::Token;

#[derive(Debug, Clone)]
pub enum Error {
    NonAsciiInput,
    ExpectedToken(Token),
    UnexpectedToken(Token),
}

impl From<qqml_lexer::Error> for Error {
    fn from(value: qqml_lexer::Error) -> Self {
        match value {
            qqml_lexer::Error::NonAsciiInput => Self::NonAsciiInput,
        }
    }
}
