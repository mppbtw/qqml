use crate::error::ErrorReport;
use crate::Error;
use crate::{MultichoiceAnswer, MultichoiceData};
use qqml_lexer::{Lexer, Token};

pub fn parse_multichoice(l: &mut Lexer) -> Result<MultichoiceData, ErrorReport> {
    // This function expects the 'ask' and 'multichoice' tokens
    // to have already been consumed by it's caller.

    let mut report = ErrorReport::new();
    let mut tok = l.next_token();
    let mut dat = MultichoiceData::default();

    if tok != Token::LParen {
        report
            .errors
            .push(Error::ExpectedLParenForQuestionMaxMark(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Number(n) => dat.max_marks = Some(n),
        _ => report
            .errors
            .push(Error::ExpectedNumberForQuestionMaxMark(tok)),
    };

    tok = l.next_token();
    if tok != Token::RParen {
        report
            .errors
            .push(Error::ExpectedRParenForQuestionMaxMark(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Literal(l) => dat.text = Some(l),
        _ => report.errors.push(Error::ExpectedQuestionText(tok)),
    };

    tok = l.next_token();
    if tok != Token::LSquirly {
        report.errors.push(Error::ExpectedLSquirlyForQuestion(tok));
    }

    loop {
        tok = l.next_token();
        if tok == Token::RParen {
            break;
        }
        if tok == Token::Asterisk {
            match parse_multichoice_question(l) {
                Ok(a) => dat.answers.push(a),
                Err(r) => report.extend(r),
            }
        }
    }

    if report.errors.len() != 0 {
        Err(report)
    } else {
        Ok(dat)
    }
}

pub fn parse_multichoice_question(l: &mut Lexer) -> Result<MultichoiceAnswer, ErrorReport> {
    // This function assumes that the '*' token has already been
    // consumed and therefore assumes that the next token should
    // be the question text.
    let mut report = ErrorReport::default();
    let mut dat = MultichoiceAnswer::default();
    let mut tok = l.next_token();

    match tok {
        Token::Literal(m) => dat.text = Some(m),
        _ => report.errors.push(Error::ExpectedAnswerText(tok)),
    };

    tok = l.next_token();
    if tok == Token::LParen {
        tok = l.next_token();
        match tok {
            Token::Number(n) => dat.marks = Some(n),
            _ => report.errors.push(Error::ExpectedNumberForAnswerMark(tok)),
        };

        tok = l.next_token();
        if tok != Token::RParen {
            report.errors.push(Error::ExpectedRParenForAnswerMark(tok));
        }

        tok = l.next_token();
    }

    if tok == Token::RArrow {
        tok = l.next_token();
        match tok {
            Token::Literal(ref l) => dat.explanation = Some(l.to_owned()),
            _ => report.errors.push(Error::ExpectedAnswerExplanationText(tok.clone())),
        };
        tok = l.next_token();
    }

    if tok != Token::Semicolon {
        report.errors.push(Error::ExpectedAnswerSemicolon(tok));
    }

    if report.errors.len() != 0 {
        Err(report)
    } else {
        Ok(dat)
    }
}
