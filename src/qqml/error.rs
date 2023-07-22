pub type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
pub enum Error {
    NonAsciiInput,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NonAsciiInput => {
                "Creation of the lexer failed because the input contained non-ASCII characters"
            }
        };

        write!(f, "{}", msg)
    }
}
