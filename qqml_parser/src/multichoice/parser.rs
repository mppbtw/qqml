use qqml_lexer::*;

use super::MultichoiceAnswer;
use super::MultichoiceData;

use crate::error::expected_err;
use crate::utils::is_empty_str;
use crate::Error;
use crate::Question;

/// Parse the question assuming that the ask and multichoice keyword
/// tokens have already been comsumed.
pub fn parse_multichoice(l: &mut Lexer) -> Result<Question, Error> {
    let mut dat = MultichoiceData::default();
    let mut tok;

    // Example syntax:
    //  ask multichoice (2) "What is the best language?" {
    //     * "rust" (1) -> "explanation goes here";
    //     * "python" (1);
    //     * "C++";
    //  };

    tok = l.next_token();
    if tok != Token::LParen {
        return Err(Error::UnexpectedToken(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Number(n) => dat.set_max_marks(n),
        _ => return Err(Error::UnexpectedToken(tok)),
    };

    tok = l.next_token();
    if tok != Token::RParen {
        return Err(Error::UnexpectedToken(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Literal(l) => {
            if is_empty_str(&l) {
                return Err(Error::EmptyQuestionText);
            } else {
                dat.set_text(l);
            }
        }
        _ => return Err(Error::UnexpectedToken(tok)),
    }

    tok = l.next_token();
    if tok != Token::LSquirly {
        return Err(Error::UnexpectedToken(tok));
    }

    // For each possible answer in the question body
    loop {
        tok = l.next_token();
        if tok == Token::RSquirly {
            break;
        }
        if tok != Token::Asterisk {
            return Err(Error::UnexpectedToken(tok));
        }

        parse_multichoice_answer(l)?;
    }

    Ok(Question::Multichoice(dat))
}

/// Parse a single multichoice answer, assumes that the asterisk
/// has already been consumed.
pub fn parse_multichoice_answer(l: &mut Lexer) -> Result<MultichoiceAnswer, Error> {
    let mut tok;
    let mut answer = MultichoiceAnswer::default();

    tok = l.next_token();
    match tok {
        Token::Literal(l) => {
            if is_empty_str(&l) {
                return Err(Error::EmptyAnswerText);
            } else {
                answer.set_text(l);
            }
        },
        _ => {
            return expected_err(
                Token::Literal("".to_owned()),
                tok,
                "Multichoice answers must contain the text to be displayed.",
            );
        }
    };

    tok = l.next_token();
    if tok != Token::LParen {
        if tok == Token::Semicolon {
            answer.set_marks(0);
            return Ok(answer);
        }
        return expected_err(
            Token::LParen,
            tok,
            "The mark for this question should be enclosed in brackets, e.g. (2)",
        );
    };

    tok = l.next_token();
    match tok {
        Token::Number(n) => answer.set_marks(n),
        _ => {
            return expected_err(
                Token::Number(0),
                tok,
                "The mark for this answer should be enclosed in brackets, e.g. (2)",
            )
        }
    };

    tok = l.next_token();
    if tok != Token::RParen {
        return expected_err(
            Token::RParen,
            tok,
            "The mark for this answer should be enclosed in brackets, e.g. (2)",
        );
    }

    tok = l.next_token();
    if tok == Token::Semicolon {
        return Ok(answer);
    }

    if tok != Token::RArrow {
        return expected_err(
            vec![Token::RArrow, Token::Semicolon],
            tok,
            "The answer should either terminate here or provide and explanation",
        );
    }

    tok = l.next_token();
    match tok {
        Token::Literal(l) => answer.set_explanation(l),
        _ => {
            return expected_err(
                Token::Literal("".to_owned()),
                tok,
                "An answer explanation should be given as a string literal after the arrow.",
            )
        }
    }

    Ok(answer)
}
