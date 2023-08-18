pub mod lexer;

#[cfg(test)]
mod test;

use crate::eval::state::State;
use lexer::*;

pub fn json_to_state(json: String) -> Result<State, JsonError> {
    Err(JsonError::UnexpectedEof(Token::LSquirly(
        TokenData::default(),
    )))
}
