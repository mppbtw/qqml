use crate::error::ErrorReport;
use crate::*;
use qqml_lexer::*;

pub fn parse_multichoice_answer(l: &mut Lexer) -> Result<MultichoiceAnswer, ErrorReport> {
    // This parser is replacement tolerant and assumes
    // that the '*' token has already been consumed
    // and therefore assumes that the next token should
    // be the question text.

    let starting_l_data = l.get_lexer_data();
    let mut report = ErrorReport::default();
    let mut dat = MultichoiceAnswer::default();
    let mut tok = l.next_token()?;

    match tok {
        Token::Literal(_, m) => dat.text = Some(m),
        _ => report.errors.push(Error::ExpectedAnswerText(tok)),
    };

    loop {
        tok = l.next_token()?;
        if matches!(tok, Token::LParen(_)) {
            tok = l.next_token()?;
            match tok {
                Token::Number(_, n) => dat.marks = n,
                _ => report.errors.push(Error::UnexpectedAnswerToken(
                    tok,
                    vec![
                        Token::RArrow(TokenData::default()),
                        Token::Semicolon(TokenData::default()),
                        Token::Number(TokenData::default(), 0),
                    ],
                )),
            };

            tok = l.next_token()?;
            if !matches!(tok, Token::RParen(_)) {
                report.errors.push(Error::ExpectedRParenForAnswerMark(tok));
            }

            tok = l.next_token()?;
        }

        if matches!(tok, Token::RArrow(_)) {
            tok = l.next_token()?;
            match tok {
                Token::Literal(_, l) => dat.explanation = Some(l),
                _ => report
                    .errors
                    .push(Error::ExpectedAnswerExplanationText(tok)),
            };
            tok = l.next_token()?;
        }

        if !matches!(tok, Token::Semicolon(_)) {
            report
                .errors
                .push(Error::ExpectedAnswerSemicolon(tok.clone()));
        } else {
            break;
        }
        if matches!(tok, Token::Eof(_)) {
            break;
        }
    }

    if !report.errors.is_empty() {
        if report.errors.len() == 1 {
            Err(report)
        } else {
            let pos = positive_tolerance(&mut Lexer::from_lexer_data(
                l.get_input(),
                starting_l_data.clone(),
            ))
            .unwrap_err();
            if pos.errors.len() == 1 {
                Err(pos)
            } else if pos.errors.len() < report.errors.len() {
                let neg = negative_tolerance(&mut Lexer::from_lexer_data(
                    l.get_input(),
                    starting_l_data.clone(),
                ))
                .unwrap_err();
                if neg.errors.len() < pos.errors.len() {
                    Err(neg)
                } else {
                    Err(pos)
                }
            } else {
                let neg = negative_tolerance(&mut Lexer::from_lexer_data(
                    l.get_input(),
                    starting_l_data.clone(),
                ))
                .unwrap_err();
                if neg.errors.len() < report.errors.len() {
                    Err(neg)
                } else {
                    Err(report)
                }
            }
        }
    } else {
        Ok(dat)
    }
}

pub fn negative_tolerance(l: &mut Lexer) -> Result<(), ErrorReport> {
    let mut report = ErrorReport::default();
    let mut tok = l.next_token()?;
    let mut skip_next_tok = false;

    match tok {
        Token::Literal(..) => (),
        _ => {
            report.errors.push(Error::ExpectedAnswerText(tok.clone()));
            skip_next_tok = true;
        }
    };

    loop {
        if skip_next_tok {
            skip_next_tok = false;
        } else {
            tok = l.next_token()?;
        }
        if matches!(tok, Token::LParen(_)) {
            tok = l.next_token()?;
            match tok {
                Token::Number(..) => tok = l.next_token()?,
                _ => report.errors.push(Error::UnexpectedAnswerToken(
                    tok.clone(),
                    vec![
                        Token::RArrow(TokenData::default()),
                        Token::Semicolon(TokenData::default()),
                        Token::Number(TokenData::default(), 0),
                    ],
                )),
            };

            if !matches!(tok, Token::RParen(_)) {
                report
                    .errors
                    .push(Error::ExpectedRParenForAnswerMark(tok.clone()));
            } else {
                tok = l.next_token()?;
            }
        }

        if matches!(tok, Token::RArrow(_)) {
            tok = l.next_token()?;
            match tok {
                Token::Literal(..) => tok = l.next_token()?,
                _ => report
                    .errors
                    .push(Error::ExpectedAnswerExplanationText(tok.clone())),
            };
        }

        if !matches!(tok, Token::Semicolon(_)) {
            report
                .errors
                .push(Error::ExpectedAnswerSemicolon(tok.clone()));
        } else {
            break;
        }
        if matches!(tok, Token::Eof(_)) {
            break;
        }
    }

    Err(report)
}
fn positive_tolerance(l: &mut Lexer) -> Result<(), ErrorReport> {
    let mut report = ErrorReport::default();
    let mut tok;

    loop {
        tok = l.next_token()?;
        match tok {
            Token::Literal(..) => break,
            Token::Eof(_) => break,

            _ => report.errors.push(Error::ExpectedAnswerText(tok)),
        };
    }

    let mut tok = l.next_token()?;
    if matches!(tok, Token::LParen(_)) {
        tok = l.next_token()?;
        match tok {
            Token::Number(..) => (),
            _ => {
                report.errors.push(Error::UnexpectedAnswerToken(
                    tok,
                    vec![
                        Token::RArrow(TokenData::default()),
                        Token::Semicolon(TokenData::default()),
                        Token::Number(TokenData::default(), 0),
                    ],
                ));
                l.next_token()?;
            }
        };

        tok = l.next_token()?;
        if !matches!(tok, Token::RParen(_)) {
            report.errors.push(Error::ExpectedRParenForAnswerMark(tok));
            l.next_token()?;
        }

        tok = l.next_token()?;
    }

    if matches!(tok, Token::RArrow(_)) {
        tok = l.next_token()?;
        match tok {
            Token::Literal(..) => (),
            _ => {
                report
                    .errors
                    .push(Error::ExpectedAnswerExplanationText(tok));
                l.next_token()?;
            }
        };
        tok = l.next_token()?;
    }

    if !matches!(tok, Token::Semicolon(_)) {
        report.errors.push(Error::ExpectedAnswerSemicolon(tok));
        l.next_token()?;
    }

    Err(report)
}
