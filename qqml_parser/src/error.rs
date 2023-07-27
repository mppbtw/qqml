use qqml_lexer::Token;

#[derive(Debug, Clone)]
pub enum Error {
    NonAsciiInput,
    /// Stores the expected token/s, and then the token we recieved plus any extra info for the end user.
    ExpectedToken(Vec<Token>, Token, String),
    UnexpectedToken(Token),
}
