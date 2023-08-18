mod lexer;
mod parser;

#[cfg(test)]
mod test;

use self::parser::JsonSyntaxError;
use crate::eval::state::State;
use lexer::*;

pub fn json_to_state(json: String) -> Result<State, JsonSyntaxError> {
    Err(JsonSyntaxError(Token::Eof(TokenData::default())))
}
