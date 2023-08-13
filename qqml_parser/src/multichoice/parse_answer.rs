use crate::error::ErrorReport;
use crate::*;
use qqml_lexer::*;

pub fn parse_multichoice_answer(l: &mut Lexer) -> Result<MultichoiceAnswer, ErrorReport> {
    // This parser is replacement tolerant and assumes
    // that the '*' token has already been consumed
    // and therefore assumes that the next token should
    // be the question text.

    let mut starting_lexer = l.to_owned();
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

        if !matches!(tok, Token::Semicolon(_)){
            report.errors.push(Error::ExpectedAnswerSemicolon(tok.clone()));
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
            let pos = positive_tolerance(&mut starting_lexer).unwrap_err();
            if pos.errors.len() == 1 || pos.errors.len() < report.errors.len() {
                Err(pos)
            } else {
                Err(report)
            }
        }
    } else {
        Ok(dat)
    }
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
