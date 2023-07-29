use crate::error::ErrorReport;
use crate::multichoice::parse_multichoice;
use crate::Error;
use crate::Question;
use crate::Warning;
use qqml_lexer::Lexer;
use qqml_lexer::Token;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ParsedFile {
    pub questions: Vec<Question>,
    pub max_hints: usize,
    pub warnings: Vec<Warning>,
}

pub fn parse(inp: String) -> Result<ParsedFile, Vec<Error>> {
    let mut max_hints = 0;
    let mut hints_directive_seen = false;
    let mut l = Lexer::new(inp);
    let mut output = ParsedFile::default();

    let mut report = ErrorReport::new();

    loop {
        let mut tok = l.next_token();
        if matches!(tok, Token::Semicolon(_)) {
            break;
        }

        // Parse 'hints' directive
        if matches!(tok, Token::Hints(_)) {
            if hints_directive_seen {
                report.errors.push(Error::HintsDirectiveRepeated);
                continue;
            } else {
                hints_directive_seen = true;
                tok = l.next_token();
                match tok {
                    Token::Number(_, n) => max_hints = n,
                    _ => report.errors.push(Error::HintsDirectiveRepeated),
                }
                tok = l.next_token();
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
            // Get the type of the next token
            tok = l.next_token();
            match tok {
                Token::Multichoice(_) => match parse_multichoice(&mut l) {
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

    Ok(ParsedFile {
        questions: vec![],
        max_hints,
        warnings: vec![],
    })
}
