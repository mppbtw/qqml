use qqml_lexer::Lexer;
use crate::error::ErrorReport;
use crate::Error;
use crate::multichoice::parse_multichoice;
use qqml_lexer::Token;
use crate::Question;
use crate::Warning;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ParsedFile {
    pub questions: Vec<Question>,
    pub max_hints: usize,
    pub warnings: Vec<Warning>
}

pub fn parse(inp: String) -> Result<ParsedFile, Vec<Error>> {
    let mut max_hints = 0;
    let mut hints_directive_seen = false;
    let mut l = Lexer::new(inp);
    let mut output = ParsedFile::default();

    let mut report = ErrorReport::new();

    loop {
        let mut tok = l.next_token();
        if tok == Token::Semicolon {
            break;
        }

        // Parse 'hints' directive
        if tok == Token::Hints {
            if hints_directive_seen {
                report.errors.push(Error::HintsDirectiveRepeated);
                continue;
            } else {
                hints_directive_seen = true;
                tok = l.next_token();
                match tok {
                    Token::Number(n) => max_hints = n,
                    _ => report.errors.push(Error::HintsDirectiveRepeated),
                }
                continue
            }
        }

        // Parse questions
        if tok == Token::Ask {

            // Get the type of the next token
            tok = l.next_token();
            match tok {
                Token::Multichoice => {
                    match parse_multichoice(&mut l) {
                        Ok(data) => output.questions.push(Question::Multichoice(data)),
                        Err(r) => report.extend(r),
                    }
                },
                Token::Ident(_) => report.errors.push(Error::InvalidQuestionType(tok)),
                _ => ()
            }
            continue;
        }

        report.errors.push(Error::ExpectedQuestionOrDirective(tok));
    }

    Ok(ParsedFile {
        questions: vec![],
        max_hints,
        warnings: vec![],
    })
}
