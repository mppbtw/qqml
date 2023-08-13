use qqml_lexer::*;
use crate::*;
use crate::error::ErrorReport;

pub fn parse_multichoice_answer(l: &mut Lexer) -> Result<MultichoiceAnswer, ErrorReport> {
    // This parser is replacement tolerant and assumes
    // that the '*' token has already been consumed
    // and therefore assumes that the next token should
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
