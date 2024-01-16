//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
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

use crate::lexer::core::Lexer;
use crate::lexer::token::Token;
use crate::parser::error::Error;
use crate::parser::multichoice::data::MultichoiceAnswer;

pub fn parse_multichoice_answer(l: &mut Lexer) -> Result<MultichoiceAnswer, Error> {
    // This parser assumes that the '*' token has already been consumed and
    // therefore assumes that the next token should be the question text.

    let mut dat = MultichoiceAnswer::default();
    let mut tok = l.next_token()?;

    match tok {
        Token::Literal(_, m) => dat.text = Some(m),
        _ => return Err(Error::ExpectedAnswerText(tok)),
    };

    tok = l.next_token()?;
    if matches!(tok, Token::LParen(_)) {
        tok = l.next_token()?;
        match tok {
            Token::Number(_, n) => dat.marks = n,
            _ => return Err(Error::ExpectedNumberForAnswerMark(tok)),
        }

        tok = l.next_token()?;
        if !matches!(tok, Token::RParen(_)) {
            return Err(Error::ExpectedRParenForAnswerMark(tok));
        }

        tok = l.next_token()?;
    }

    if matches!(tok, Token::RArrow(_)) {
        tok = l.next_token()?;
        match tok {
            Token::Literal(_, l) => dat.explanation = Some(l),
            _ => return Err(Error::ExpectedAnswerExplanationText(tok)),
        };
        tok = l.next_token()?;
    }

    if !matches!(tok, Token::Semicolon(_)) {
        return Err(Error::ExpectedAnswerSemicolon(tok));
    }

    Ok(dat)
}
(dat)
}
