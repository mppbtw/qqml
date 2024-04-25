//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

mod c_utils;
mod eval;
mod json;
mod lexer;
mod parser;

pub use eval::diagnostics::render_error;
pub use eval::repl::run;
pub use eval::repl::run_from_state;
pub use eval::state::State;
pub use lexer::core::Lexer;
pub use lexer::core::LexerError;
pub use lexer::token::Token;
pub use lexer::token::TokenData;
pub use parser::core::parse;
pub use parser::error::Error;
pub use parser::multichoice::parse_answer::parse_multichoice_answer;
pub use parser::multichoice::parser::parse_multichoice;
pub use parser::Question;
