//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'MrPiggyPegasus'
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

use super::error::Error;
use super::multichoice::parser::parse_multichoice;
use super::Question;
use crate::lexer::core::Lexer;
use crate::lexer::token::Token;
use crate::State;

pub fn parse<S: Into<String>>(inp: S, path_to_source: Option<String>) -> Result<State, Error> {
    let inp: String = inp.into();
    let mut hints_directive_seen = false;
    let mut l = Lexer::new(inp);
    let mut output = State::default();
    output.path_to_source = path_to_source;

    let mut tok;

    loop {
        tok = l.next_token()?;
        if matches!(tok, Token::Semicolon(_)) {
            continue;
        }

        if matches!(tok, Token::Eof(_)) {
            break;
        }

        // Parse 'hints' directive
        if matches!(tok, Token::Hints(_)) {
            if hints_directive_seen {
                return Err(Error::HintsDirectiveRepeated(tok));
            } else {
                hints_directive_seen = true;
                tok = l.next_token()?;
                if let Token::Number(_, n) = tok {
                    output.max_hints = n;
                    tok = l.next_token()?;
                } else {
                    return Err(Error::HintsDirectiveRequiresNumber(tok));
                }
                if !matches!(tok, Token::Semicolon(_)) {
                    return Err(Error::ExpectedSemicolonAfterHintsDirective(tok));
                }
                continue;
            }
        }

        // Parse questions
        if matches!(tok, Token::Ask(_)) {
            let keyword = tok;
            // Get the type of the next token
            tok = l.next_token()?;
            match tok {
                Token::Multichoice(_) => output
                    .questions
                    .push(Question::Multichoice(parse_multichoice(&mut l, keyword)?)),
                Token::Ident(..) => return Err(Error::InvalidQuestionType(tok)),
                _ => return Err(Error::ExpectedQuestionType(tok)),
            }
            continue;
        }

        return Err(Error::ExpectedQuestionType(tok));
    }

    Ok(output)
}
