mod lexer;
mod token;

pub use lexer::Lexer;
pub use token::Token;
pub use token::TokenData;
pub use lexer::UnterminatedStringError;

#[cfg(test)]
mod test;
