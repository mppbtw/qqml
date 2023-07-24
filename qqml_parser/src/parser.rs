use crate::error::Error;
use crate::multichoice::parse_multichoice;
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
