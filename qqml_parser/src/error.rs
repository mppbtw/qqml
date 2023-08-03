use std::fmt;

use crate::Warning;
use qqml_lexer::Token;
use qqml_lexer::TokenData;
use qqml_lexer::UnterminatedStringError;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    HintsDirectiveRequiresNumber,
    HintsDirectiveRepeated,

    /// Stores the data of where the literal began.
    UnterminatedLiteral(TokenData),

    UnexpectedBodyToken(Token),
    ExpectedLSquirlyForQuestion(Token),
    ExpectedRParenForQuestionMaxMark(Token),
    ExpectedLParenForQuestionMaxMark(Token),
    ExpectedNumberForQuestionMaxMark(Token),
    ExpectedQuestionText(Token),
    ExpectedSemicolonAfterHintsDirective(Token),
    ExpectedQuestionOrDirective(Token),
    ExpectedQuestionType(Token),
    ExpectedHintText(Token),
    ExpectedCommaInHintsList(Token),

    UnexpectedAnswerToken(Token, Vec<Token>),
    ExpectedAnswerText(Token),
    ExpectedAnswerSemicolon(Token),
    ExpectedAnswerExplanationText(Token),
    ExpectedRParenForAnswerMark(Token),
    ExpectedNumberForAnswerMark(Token),

    /// Stores the token (hopefully Token::Identifier) of the question
    /// which was invalid.
    InvalidQuestionType(Token),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::InvalidQuestionType(t) =>
                format!("The question type {} is invalid, see doc#", t),
            Self::ExpectedHintText(_) =>
                format!("The `hints` keyword should precede the hints themselves, see doc#hints"),
            Self::ExpectedAnswerText(_) => 
                format!("The answer should contain a string, see doc#multichoice"),
            Self::HintsDirectiveRepeated => "".to_owned(),
            _ => format!("guh")
        };

        let _ = msg.replace("$", "https://github.com/MrPiggyPegasus/yarr/blob/main/doc/QQML.md");
        write!(f, "{}", msg)
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ErrorReport {
    pub errors: Vec<Error>,
    pub warnings: Vec<Warning>,
}
impl From<UnterminatedStringError> for ErrorReport {
    fn from(value: UnterminatedStringError) -> Self {
        Self {
            errors: vec![Error::UnterminatedLiteral(value.0)],
            warnings: vec![]
        }
    }
}
impl Into<Vec<ErrorReport>> for ErrorReport {
    fn into(self) -> Vec<ErrorReport> {
        vec![self]
    }
}

impl ErrorReport {
    pub fn new() -> ErrorReport {
        Self::default()
    }

    pub fn extend(&mut self, other: ErrorReport) {
        for error in other.errors {
            self.errors.push(error);
        }
        for warning in other.warnings {
            self.warnings.push(warning);
        }
    }
}
