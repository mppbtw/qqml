use super::parse_answer::parse_multichoice_answer;
use crate::error::ErrorReport;
use crate::Error;
use crate::MultichoiceData;
use qqml_lexer::Lexer;
use qqml_lexer::Token;

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

    dat.line = keyword.get_data().line;

    if !report.errors.is_empty() {
        Err(report)
    } else {
        Ok(dat)
    }
}
