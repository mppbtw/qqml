use qqml_lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    NonAsciiInput,
    /// Stores the expected token/s, and then the token we recieved.
    ExpectedToken(Vec<Token>, Token),
    UnexpectedToken(Token),
}

impl From<qqml_lexer::Error> for Error {
    fn from(value: qqml_lexer::Error) -> Self {
        match value {
            qqml_lexer::Error::NonAsciiInput => Self::NonAsciiInput,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::UnexpectedToken(t) => format!("Unexpected token: {}", t),
            Self::ExpectedToken(t, g) => {
                if t.len() == 1 {
                    format!("Expected token {}, got {}", t[0], g)
                } else {
                    let token_names = t.iter().map(|t| format!("{}", t)).collect::<Vec<String>>();
                    format!("Expected one of {}, got {}", token_names.join(", "), g)
                }
            }
            Self::NonAsciiInput => "QQML doesn't support Unicode just yet!".to_owned(),
        };
        write!(f, "{}", msg)
    }
}
