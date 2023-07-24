use crate::error::Error;
use crate::multichoice::MultichoiceAnswer;
use crate::MultichoiceData;
use crate::Question;
use qqml_lexer::Lexer;
use qqml_lexer::Token;

pub fn parse(inp: String) -> Result<Vec<Question>, Error> {
    let mut l = Lexer::new(inp)?;

    let mut questions: Vec<Question> = Vec::new();

    // Loops once per question
    loop {
        let tok = l.next_token();

        if tok == Token::Eof {
            break;
        }
        if tok == Token::Semicolon {
            continue;
        }
        if tok != Token::Ask {
            return Err(Error::UnexpectedToken(tok));
        }

        let question_type = l.next_token();
        match question_type {
            Token::Multichoice => {
                questions.push(parse_multichoice(&mut l)?);
            }
            _ => return Err(Error::UnexpectedToken(question_type)),
        };
    }

    Ok(vec![Question::Multichoice(MultichoiceData::default())])
}

/// Parse the question assuming that the ask and multichoice keyword
/// tokens have already been comsumed.
pub fn parse_multichoice(l: &mut Lexer) -> Result<Question, Error> {
    let mut dat = MultichoiceData::default();
    let mut tok; // dummy value

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
        Token::Literal(l) => dat.set_text(l),
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
    let mut tok; // dummy value
    let mut answer = MultichoiceAnswer::default();

    tok = l.next_token();
    match tok {
        Token::Literal(l) => answer.set_text(l),
        _ => return Err(Error::UnexpectedToken(tok)),
    };

    tok = l.next_token();
    if tok != Token::LParen {
        return Err(Error::UnexpectedToken(tok));
    };

    tok = l.next_token();
    match tok {
        Token::Number(n) => answer.set_marks(n),
        _ => return Err(Error::UnexpectedToken(tok)),
    };

    tok = l.next_token();
    if tok != Token::RParen {
        return Err(Error::UnexpectedToken(tok));
    }

    tok = l.next_token();
    if tok == Token::Semicolon {
        return Ok(answer);
    }
    if tok != Token::RArrow {
        return Err(Error::UnexpectedToken(tok));
    }

    tok = l.next_token();
    match tok {
        Token::Literal(l) => answer.set_explanation(l),
        _ => return Err(Error::UnexpectedToken(tok)),
    }

    Ok(answer)
}
