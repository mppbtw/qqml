use super::error::Error;
use super::error::ErrorReport;
use super::multichoice::parser::parse_multichoice;
use super::Question;
use crate::lexer::core::Lexer;
use crate::lexer::token::Token;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct ParsedFile {
    pub questions: Vec<Question>,
    pub max_hints: usize,
}
impl ParsedFile {
    pub fn to_json(&self) -> String {
        let mut output = String::new();
        output += "{";
        output += &format!("\"max_hints\": {},", self.max_hints);
        output += "\"questions\": [";
        output += &self
            .questions
            .iter()
            .map(|q| q.to_json())
            .collect::<Vec<String>>()
            .join(",");
        output += "]}";
        output
    }
}

pub fn parse<S: Into<String>>(inp: S) -> Result<ParsedFile, ErrorReport> {
    // This parser is replacement tolerant, meaning that an incorrect token
    // will only cause one error, and the parsing process can continue,
    // rather than returning more and more errors for every token in the file.
    // The other classifications being positively (extra tokens) and
    // negatively (missing tokens) tolerant. In order to hand-write a parser
    // which is able to correctly report errors under all of the above circumstances,
    // one must either create one parser which is negatively, positively and
    // replacement tolerant at the same time (more reasonable for AST
    // constructors), or have 3 seperate parsers and only run one unless an
    // error is detected, then run the others and get the lowest error count.
    // The latter approach is called tripartite parsing and is a good fit for
    // this kind of parsing (very strict grammar, no AST). The default parser
    // here is the replacement tolerant one as it requires less complex
    // decision making and is therefore faster. Which parser to use is
    // left up to each individual section (parse(), parse_multichoice and()
    // parse_multichoice_answer()).
    //
    // NOTE: The above doesn't actually apply to the top-level parser, for it
    // only reads at most 2 tokens in a row then calls other functions based
    // on question type, where tripartite parsing is needed for the best error
    // reporting.

    let inp: String = inp.into();
    let mut hints_directive_seen = false;
    let mut l = Lexer::new(inp);
    let mut output = ParsedFile::default();

    let mut report = ErrorReport::new();

    let mut tok;

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
                report.errors.push(Error::HintsDirectiveRepeated(tok));
                continue;
            } else {
                hints_directive_seen = true;
                tok = l.next_token()?;
                if let Token::Number(_, n) = tok {
                    output.max_hints = n;
                    tok = l.next_token()?;
                } else {
                    report
                        .errors
                        .push(Error::HintsDirectiveRequiresNumber(tok.clone()));
                }
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

        report.errors.push(Error::ExpectedQuestionType(tok.clone()));
        break;
    }

    if report.errors.is_empty() {
        Ok(output)
    } else {
        Err(report)
    }
}
