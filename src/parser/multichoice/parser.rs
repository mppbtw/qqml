use super::parse_answer::parse_multichoice_answer;
use crate::lexer::core::Lexer;
use crate::lexer::token::Token;
use crate::parser::error::Error;
use crate::parser::error::ErrorReport;
use crate::parser::MultichoiceData;

/// 't' is the token for the ask keyword used to attach some more metadata to the data
pub fn parse_multichoice<T: Into<Token>>(
    l: &mut Lexer,
    keyword: T,
) -> Result<MultichoiceData, ErrorReport> {
    // This parser is replacement tolerant and
    // expects the 'ask' and 'multichoice' tokens
    // to have already been consumed by the top
    // level parser.

    let keyword = keyword.into();

    let starting_l_data = l.get_lexer_data();
    let mut report = ErrorReport::new();
    let mut tok = l.next_token()?;
    let mut dat = MultichoiceData::default();

    if !matches!(tok, Token::LParen(_)) {
        report
            .errors
            .push(Error::ExpectedLParenForQuestionMaxMark(tok.clone()))
    }
    tok = l.next_token()?;

    match tok {
        Token::Number(_, n) => {
            if n == 0 {
                report.errors.push(Error::MaxMarkIsZero(tok));
            } else {
                dat.max_marks = n;
            }
        }
        _ => report
            .errors
            .push(Error::ExpectedNumberForQuestionMaxMark(tok.clone())),
    };
    tok = l.next_token()?;

    if !matches!(tok, Token::RParen(_)) {
        report
            .errors
            .push(Error::ExpectedRParenForQuestionMaxMark(tok.clone()));
    }

    tok = l.next_token()?;
    match tok {
        Token::Literal(_, s) => dat.text = s,
        _ => report.errors.push(Error::ExpectedQuestionText(tok.clone())),
    };

    tok = l.next_token()?;
    if !matches!(tok, Token::LSquirly(_)) {
        report
            .errors
            .push(Error::ExpectedLSquirlyForQuestion(tok.clone()));
    }

    loop {
        tok = l.next_token()?;

        if matches!(tok, Token::Eof(_)) {
            report
                .errors
                .push(Error::ExpectedRSquirlyForQuestion(tok.clone()));
            break;
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
                Err(r) => report.extend(r),
            }
        } else {
            report.errors.push(Error::UnexpectedBodyToken(tok.clone()));
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
                            report.errors.push(Error::ExpectedCommaInHintsList(tok));
                            break;
                        }
                        _ => report.errors.push(Error::ExpectedCommaInHintsList(tok)),
                    };
                }
                Token::Eof(_) => {
                    report.errors.push(Error::ExpectedCommaInHintsList(tok));
                    break;
                }
                Token::Semicolon(_) => break,
                _ => report.errors.push(Error::ExpectedHintText(tok)),
            };
        }
    }

    // Some semantic analasys can be done here as the
    // errors produced need not be bound to a token we
    // have already consumed.
    if report.errors.is_empty() {
        let mut total_marks = 0;
        for a in dat.answers.iter().cloned() {
            total_marks += a.marks;
        }
        if total_marks < dat.max_marks {
            report
                .errors
                .push(Error::ImpossibleMaxMark(keyword.clone()));
        }

        if dat.answers.is_empty() {
            report
                .errors
                .push(Error::NoMultichoiceAnswers(keyword.clone()));
        } else if dat.answers.len() == 1 {
            report
                .errors
                .push(Error::OnlyOneMultichoiceAnswer(keyword.clone()));
        }
    }

    dat.line = keyword.get_data().line;

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

fn negative_tolerance(l: &mut Lexer) -> Result<(), ErrorReport> {
    let mut report = ErrorReport::new();
    let mut tok = l.next_token()?;
    if !matches!(tok, Token::LParen(_)) {
        report
            .errors
            .push(Error::ExpectedLParenForQuestionMaxMark(tok.clone()))
    } else {
        tok = l.next_token()?;
    }

    match tok {
        Token::Number(..) => tok = l.next_token()?,
        _ => report
            .errors
            .push(Error::ExpectedNumberForQuestionMaxMark(tok.clone())),
    };

    if !matches!(tok, Token::RParen(_)) {
        report
            .errors
            .push(Error::ExpectedRParenForQuestionMaxMark(tok.clone()));
    } else {
        tok = l.next_token()?;
    }

    match tok {
        Token::Literal(..) => tok = l.next_token()?,
        _ => report.errors.push(Error::ExpectedQuestionText(tok.clone())),
    };

    let mut skip_token = false;
    if !matches!(tok, Token::LSquirly(_)) {
        report
            .errors
            .push(Error::ExpectedLSquirlyForQuestion(tok.clone()));
        skip_token = true;
    }

    loop {
        if skip_token {
            skip_token = false;
        } else {
            tok = l.next_token()?;
        }

        if matches!(tok, Token::Eof(_)) {
            report
                .errors
                .push(Error::ExpectedRSquirlyForQuestion(tok.clone()));
            break;
        }

        if matches!(tok, Token::Semicolon(_)) {
            continue;
        }

        if matches!(tok, Token::RSquirly(_)) {
            break;
        }

        if matches!(tok, Token::Asterisk(_)) {
            match parse_multichoice_answer(l) {
                Ok(_) => (),
                Err(r) => report.extend(r),
            }
        } else {
            report.errors.push(Error::UnexpectedBodyToken(tok.clone()));
        }
    }

    tok = l.next_token()?;
    if matches!(tok, Token::Hints(_)) {
        loop {
            tok = l.next_token()?;
            match tok {
                Token::Literal(..) => {
                    tok = l.next_token()?;
                    match tok {
                        Token::Comma(_) => continue,
                        Token::Semicolon(_) => break,
                        Token::Eof(_) => {
                            report.errors.push(Error::ExpectedCommaInHintsList(tok));
                            break;
                        }
                        _ => report.errors.push(Error::ExpectedCommaInHintsList(tok)),
                    };
                }
                Token::Eof(_) => {
                    report.errors.push(Error::ExpectedCommaInHintsList(tok));
                    break;
                }
                Token::Semicolon(_) => break,
                _ => report.errors.push(Error::ExpectedHintText(tok)),
            };
        }
    }

    Err(report)
}

fn positive_tolerance(l: &mut Lexer) -> Result<(), ErrorReport> {
    let mut report = ErrorReport::new();
    let mut tok = l.next_token()?;

    loop {
        if !matches!(tok, Token::LParen(_)) {
            report
                .errors
                .push(Error::ExpectedLParenForQuestionMaxMark(tok.clone()));
            tok = l.next_token()?;
        } else {
            tok = l.next_token()?;
            break;
        }
    }

    loop {
        match tok {
            Token::Number(..) => {
                tok = l.next_token()?;
                break;
            }
            _ => {
                report
                    .errors
                    .push(Error::ExpectedNumberForQuestionMaxMark(tok.clone()));
                tok = l.next_token()?;
            }
        };
    }

    loop {
        if !matches!(tok, Token::RParen(_)) {
            report
                .errors
                .push(Error::ExpectedRParenForQuestionMaxMark(tok.clone()));
            tok = l.next_token()?;
        } else {
            tok = l.next_token()?;
            break;
        }
    }

    loop {
        match tok {
            Token::Literal(..) => {
                tok = l.next_token()?;
                break;
            }
            _ => {
                report.errors.push(Error::ExpectedQuestionText(tok.clone()));
                tok = l.next_token()?;
            }
        };
    }

    loop {
        if matches!(tok, Token::Eof(_)) {
            report
                .errors
                .push(Error::ExpectedLSquirlyForQuestion(tok.clone()));
            break;
        }

        if !matches!(tok, Token::LSquirly(_)) {
            report
                .errors
                .push(Error::ExpectedLSquirlyForQuestion(tok.clone()));
            tok = l.next_token()?;
        } else {
            break;
        }
    }

    loop {
        tok = l.next_token()?;

        if matches!(tok, Token::Eof(_)) {
            report
                .errors
                .push(Error::ExpectedRSquirlyForQuestion(tok.clone()));
            break;
        }

        if matches!(tok, Token::Semicolon(_)) {
            continue;
        }

        if matches!(tok, Token::RSquirly(_)) {
            break;
        }

        if matches!(tok, Token::Asterisk(_)) {
            match parse_multichoice_answer(l) {
                Ok(_) => (),
                Err(r) => report.extend(r),
            }
        } else {
            report.errors.push(Error::UnexpectedBodyToken(tok.clone()));
        }
    }

    tok = l.next_token()?;
    if matches!(tok, Token::Hints(_)) {
        loop {
            tok = l.next_token()?;
            match tok {
                Token::Literal(..) => {
                    tok = l.next_token()?;
                    match tok {
                        Token::Comma(_) => continue,
                        Token::Semicolon(_) => break,
                        Token::Eof(_) => {
                            report.errors.push(Error::ExpectedCommaInHintsList(tok));
                            break;
                        }
                        _ => report.errors.push(Error::ExpectedCommaInHintsList(tok)),
                    };
                }
                Token::Eof(_) => {
                    report.errors.push(Error::ExpectedCommaInHintsList(tok));
                    break;
                }
                Token::Semicolon(_) => break,
                _ => report.errors.push(Error::ExpectedHintText(tok)),
            };
        }
    }

    Err(report)
}
