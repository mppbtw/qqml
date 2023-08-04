mod lexer;
mod token;

pub use lexer::Lexer;
pub use lexer::UnterminatedStringError;
pub use token::Token;
pub use token::TokenData;

#[cfg(test)]
mod test;
