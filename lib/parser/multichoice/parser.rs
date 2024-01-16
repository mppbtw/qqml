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

use super::parse_answer::parse_multichoice_answer;
use crate::lexer::core::Lexer;
use crate::lexer::token::Token;
use crate::parser::error::Error;
use crate::parser::MultichoiceData;

/// 'keyword' is the token for the ask keyword used to attach some more metadata
/// to the data
pub fn parse_multichoice<T: Into<Token>>(
    l: &mut Lexer,
    keyword: T,
) -> Result<MultichoiceData, Error> {
    // This parser expects the 'ask' and 'multichoice' tokens to have already been
    // consumed by the top level parser.

    let keyword = keyword.into();

    let mut tok = l.next_token()?;
    let mut dat = MultichoiceData::default();

    if !matches!(tok, Token::LParen(_)) {
        return Err(Error::ExpectedLParenForQuestionMaxMark(tok));
    }
    tok = l.next_token()?;

    match tok {
        Token::Number(_, n) => {
            if n == 0 {
                return Err(Error::MaxMarkIsZero(tok));
            } else {
                dat.max_marks = n;
            }
        }
        _ => return Err(Error::ExpectedNumberForQuestionMaxMark(tok)),
    };
    tok = l.next_token()?;

    if !matches!(tok, Token::RParen(_)) {
        return Err(Error::ExpectedRParenForQuestionMaxMark(tok));
    }

    tok = l.next_token()?;
    match tok {
        Token::Literal(_, s) => dat.text = s,
        _ => return Err(Error::ExpectedQuestionText(tok)),
    };

    tok = l.next_token()?;
    if !matches!(tok, Token::LSquirly(_)) {
        return Err(Error::ExpectedLSquirlyForQuestion(tok));
    }

    loop {
        tok = l.next_token()?;

        if matches!(tok, Token::Eof(_)) {
            return Err(Error::ExpectedRSquirlyForQuestion(tok));
        }

        if matches!(tok, Token::Semicolon(_)) {
            continue;
        }

        if matches!(tok, Token::RSquirly(_)) {
            break;
        }

        if matches!(tok, Token::Asterisk(_)) {
            match parse_multichoice_answer(l) {
                Ok(a) => dat.answers.push(a),
                Err(r) => return Err(r),
            }
        } else {
            return Err(Error::UnexpectedBodyToken(tok));
        }
    }

    tok = l.next_token()?;
    if matches!(tok, Token::Hints(_)) {
        loop {
            tok = l.next_token()?;
            match tok {
                Token::Literal(_, h) => {
                    dat.hints.push(h);
                    tok = l.next_token()?;
                    match tok {
                        Token::Comma(_) => continue,
                        Token::Semicolon(_) => break,
                        Token::Eof(_) => {
                            return Err(Error::ExpectedCommaInHintsList(tok));
                        }
                        _ => return Err(Error::ExpectedCommaInHintsList(tok)),
                    };
                }
                Token::Eof(_) => {
                    return Err(Error::ExpectedCommaInHintsList(tok));
                }
                Token::Semicolon(_) => break,
                _ => return Err(Error::ExpectedHintText(tok)),
            };
        }
    }

    // Some semantic analasys can be done here as the
    // errors produced need not be bound to a token we
    // have already consumed.
    let mut total_marks = 0;
    for a in dat.answers.iter() {
        total_marks += a.marks;
    }
    if total_marks < dat.max_marks {
        return Err(Error::ImpossibleMaxMark(keyword));
    }

    if dat.answers.is_empty() {
        return Err(Error::NoMultichoiceAnswers(keyword));
    } else if dat.answers.len() == 1 {
        return Err(Error::OnlyOneMultichoiceAnswer(keyword));
    }
    dat.line = keyword.get_data().line;
    Ok(dat)
}
(dat)
}
