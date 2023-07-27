use crate::error::expected_err;
use crate::error::Error;
use crate::multichoice::parse_multichoice;

use crate::Question;

use qqml_lexer::Lexer;
use qqml_lexer::Token;

pub struct ParsedSection {
    pub questions: Vec<Question>,
    pub max_hints: usize,
}

pub fn parse(inp: String) -> Result<ParsedSection, Error> {
    let mut l = Lexer::new(inp)?;
    let mut max_hints: usize = 0;
    let mut questions: Vec<Question> = Vec::new();

    // Loops once per question
    loop {
        let tok = l.next_token();

        match tok {
            Token::Eof => break,
            Token::Semicolon => continue,
            Token::Ask => (),
            Token::Hints => {
                let mut tok = l.next_token();
                match tok {
                    Token::Number(n) => max_hints = n,
                    _ => {
                        return expected_err(
                            Token::Number(0),
                            tok,
                            "Expected the number of hints to be specified",
                        )
                    }
                };
                tok = l.next_token();
                if tok != Token::Semicolon {
                    return expected_err(
                        Token::Semicolon,
                        tok,
                        "Use a semicolon to terminate the hints statement",
                    );
                };
                continue;
            }
            _ => return Err(Error::UnexpectedToken(tok)),
        }

        let question_type = l.next_token();
        match question_type {
            Token::Multichoice => {
                let p = parse_multichoice(&mut l)?;

                p.validate()?;

                questions.push(Question::Multichoice(p));
            }
            _ => return Err(Error::UnexpectedToken(question_type)),
        };
    }

    Ok(ParsedSection {
        questions,
        max_hints,
    })
}
