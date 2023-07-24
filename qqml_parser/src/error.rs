use qqml_lexer::Token;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Error {
    NonAsciiInput,
    /// Stores the expected token/s, and then the token we recieved plus any extra info
    /// for the end user.
    ExpectedToken(Vec<Token>, Token, String),
    UnexpectedToken(Token),

    EmptyQuestionText,
    EmptyAnswerText,
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
            Self::ExpectedToken(t, g, i) => {
                if t.len() == 1 {
                    if i.len() == 0 {
                        format!("Expected token {}, got {}", t[0], g)
                    } else {
                        format!("{}. Expected token {}, got {}", i, t[0], g)
                    }
                } else {
                    let token_names = t.iter().map(|t| format!("{}", t)).collect::<Vec<String>>();
                    if i.len() == 0 {
                        format!("Expected one of {}, got {}", token_names.join(", "), g)
                    } else {
                        format!(
                            "{}. Expected one of {}, got {}",
                            i,
                            token_names.join(", "),
                            g
                        )
                    }
                }
            }
            Self::NonAsciiInput => "QQML doesn't support Unicode just yet.".to_owned(),
            Self::EmptyQuestionText => "Questions cannot contain only whitespace characters".to_owned(),
            Self::EmptyAnswerText => "Answers cannot contain only whitespace characters".to_owned(),
        };
        write!(f, "{}", msg)
    }
}

/// Convenience function to create expected token errors.
pub fn expected_err<X, S, T>(exp: X, gotten: Token, explanation: S) -> Result<T, Error>
where
    X: Into<Vec<Token>>,
    S: Into<String>,
{
    Err(Error::ExpectedToken(exp.into(), gotten, explanation.into()))
}
