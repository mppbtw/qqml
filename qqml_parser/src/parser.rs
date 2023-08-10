use crate::error::ErrorReport;
use crate::multichoice::parse_multichoice;
use crate::Error;
use crate::Question;
use qqml_lexer::Lexer;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ParsedFile {
    pub questions: Vec<Question>,
    pub max_hints: usize,
}

pub fn parse<S: Into<String>>(inp: S) -> Result<ParsedFile, ErrorReport> {
    let inp: String = inp.into();
    let mut hints_directive_seen = false;
    let mut l = Lexer::new(inp);
    let mut output = ParsedFile::default();

    let mut report = ErrorReport::new();

    let mut tok;

    let mut iters = 0;
    loop {
        tok = l.next_token()?;
        if matches!(tok, Token::Semicolon(_)) {
            continue;
        }

        if matches!(tok, Token::Eof(_)) {
            break;
        }

        // Parse 'hints' directive
        if matches!(tok, Token::Hints(_)) {
            if hints_directive_seen {
                report.errors.push(Error::HintsDirectiveRepeated);
                continue;
            } else {
                hints_directive_seen = true;
                tok = l.next_token()?;
                match tok {
                    Token::Number(_, n) => output.max_hints = n,
                    _ => report.errors.push(Error::HintsDirectiveRepeated),
                }
                tok = l.next_token()?;
                if !matches!(tok, Token::Semicolon(_)) {
                    report
                        .errors
                        .push(Error::ExpectedSemicolonAfterHintsDirective(tok));
                }
                continue;
            }
        }

        // Parse questions
        if matches!(tok, Token::Ask(_)) {
            let keyword = tok;
            // Get the type of the next token
            tok = l.next_token()?;
            match tok {
                Token::Multichoice(_) => match parse_multichoice(&mut l, keyword) {
                    Ok(data) => output.questions.push(Question::Multichoice(data)),
                    Err(r) => report.extend(r),
                },
                Token::Ident(..) => report.errors.push(Error::InvalidQuestionType(tok)),
                _ => report.errors.push(Error::ExpectedQuestionType(tok)),
            }
            continue;
        }

        report.errors.push(Error::ExpectedQuestionOrDirective(tok));
    }

    // If we aren't returning the questions, the consumer
    // may also want the warnings for each question, incase this
    // is the case, they are included within the error report.
    for (i, q) in output.questions.to_vec().iter_mut().enumerate() {
        q.validate();
        output.questions[i] = q.clone();
    }

    for q in output.questions.to_vec() {
        match q {
            Question::Multichoice(d) => {
                if d.warnings.len() != 0 {
                    report.warnings.append(&mut d.into());
                };
            }
            _ => (),
        };
    }

    if report.errors.is_empty() {
        Ok(output)
    } else {
        Err(report)
    }
}
