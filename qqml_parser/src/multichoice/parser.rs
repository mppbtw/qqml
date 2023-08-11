use crate::error::ErrorReport;
use crate::Error;
use crate::MultichoiceAnswer;
use crate::MultichoiceData;
use qqml_lexer::Lexer;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

/// 't' is the token for the ask keyword used to attach some more metadata to the data
pub fn parse_multichoice<T: Into<Token>>(
    l: &mut Lexer,
    keyword: T,
) -> Result<MultichoiceData, ErrorReport> {
    let keyword = keyword.into();
    // This function expects the 'ask' and 'multichoice' tokens
    // to have already been consumed by it's caller.

    let mut report = ErrorReport::new();
    let mut tok = l.next_token()?;
    let mut dat = MultichoiceData::default();

    if !matches!(tok, Token::LParen(_)) {
        report
            .errors
            .push(Error::ExpectedLParenForQuestionMaxMark(tok.clone()))
    } else {
        tok = l.next_token()?;
    }

    match tok {
        Token::Number(_, n) => {
            if n == 0 {
                report.errors.push(Error::MaxMarkIsZero(tok));
            } else {
                dat.max_marks = n;
            }
            tok = l.next_token()?;
        }
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
        Token::Literal(_, s) => {
            dat.text = s;
            tok = l.next_token()?;
        }
        _ => report.errors.push(Error::ExpectedQuestionText(tok.clone())),
    };

    let mut skip_next_tok = false;

    if !matches!(tok, Token::LSquirly(_)) {
        report
            .errors
            .push(Error::ExpectedLSquirlyForQuestion(tok.clone()));
        skip_next_tok = true;
    }

    loop {
        if skip_next_tok {
            skip_next_tok = false;
        } else {
            tok = l.next_token()?;
        }

        if matches!(tok.clone(), Token::Semicolon(_)) {
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
    let mut total_marks = 0;
    for a in dat.answers.to_vec() {
        total_marks += a.marks;
    }
    if total_marks < dat.max_marks {
        report.errors.push(Error::ImpossibleMaxMark(keyword.clone()));
    }

    if dat.answers.len() == 0 {
        report.errors.push(Error::NoMultichoiceAnswers(keyword.clone()));
    } else if dat.answers.len() == 1 {
        report.errors.push(Error::OnlyOneMultichoiceAnswer(keyword.clone()));
    }

    dat.line = keyword.get_data().line;

    if !report.errors.is_empty() {
        Err(report)
    } else {
        Ok(dat)
    }
}

pub fn parse_multichoice_answer(l: &mut Lexer) -> Result<MultichoiceAnswer, ErrorReport> {
    // This function assumes that the '*' token has already been
    // consumed and therefore assumes that the next token should
    // be the question text.
    let mut report = ErrorReport::default();
    let mut dat = MultichoiceAnswer::default();
    let mut tok = l.next_token()?;

    match tok {
        Token::Literal(_, m) => dat.text = Some(m),
        _ => report.errors.push(Error::ExpectedAnswerText(tok)),
    };

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
        report.errors.push(Error::ExpectedAnswerSemicolon(tok));
    }

    if !report.errors.is_empty() {
        Err(report)
    } else {
        Ok(dat)
    }
}
