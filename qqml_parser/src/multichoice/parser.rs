use crate::error::ErrorReport;
use crate::Error;
use crate::MultichoiceAnswer;
use crate::MultichoiceData;
use qqml_lexer::Lexer;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

pub fn parse_multichoice(l: &mut Lexer) -> Result<MultichoiceData, ErrorReport> {
    // This function expects the 'ask' and 'multichoice' tokens
    // to have already been consumed by it's caller.

    let mut report = ErrorReport::new();
    let mut tok = l.next_token();
    let mut dat = MultichoiceData::default();

    if !matches!(tok, Token::LParen(_)) {
        report
            .errors
            .push(Error::ExpectedLParenForQuestionMaxMark(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Number(_, n) => dat.max_marks = Some(n),
        _ => report
            .errors
            .push(Error::ExpectedNumberForQuestionMaxMark(tok)),
    };

    tok = l.next_token();
    if !matches!(tok, Token::RParen(_)) {
        report
            .errors
            .push(Error::ExpectedRParenForQuestionMaxMark(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Literal(_, l) => dat.text = Some(l),
        _ => report.errors.push(Error::ExpectedQuestionText(tok)),
    };

    tok = l.next_token();
    if !matches!(tok, Token::LSquirly(_)) {
        report.errors.push(Error::ExpectedLSquirlyForQuestion(tok));
    }

    loop {
        tok = l.next_token();
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
        }
    }

    if report.errors.len() != 0 {
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
    let mut tok = l.next_token();

    match tok {
        Token::Literal(_, m) => dat.text = Some(m),
        _ => report.errors.push(Error::ExpectedAnswerText(tok)),
    };

    tok = l.next_token();
    if matches!(tok, Token::LParen(_)) {
        tok = l.next_token();
        match tok {
            Token::Number(_, n) => dat.marks = Some(n),
            _ => report.errors.push(Error::UnexpectedAnswerToken(
                tok,
                vec![
                    Token::RArrow(TokenData::default()),
                    Token::Semicolon(TokenData::default()),
                    Token::Number(TokenData::default(), 0),
                ],
            )),
        };

        tok = l.next_token();
        if !matches!(tok, Token::RParen(_)) {
            report.errors.push(Error::ExpectedRParenForAnswerMark(tok));
        }

        tok = l.next_token();
    }

    if matches!(tok, Token::RArrow(_)) {
        tok = l.next_token();
        match tok {
            Token::Literal(_, l) => dat.explanation = Some(l),
            _ => report
                .errors
                .push(Error::ExpectedAnswerExplanationText(tok)),
        };
        tok = l.next_token();
    }

    if !matches!(tok, Token::Semicolon(_)) {
        report.errors.push(Error::ExpectedAnswerSemicolon(tok));
    }

    if report.errors.len() != 0 {
        Err(report)
    } else {
        Ok(dat)
    }
}
