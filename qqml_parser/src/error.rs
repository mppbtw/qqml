use qqml_lexer::Token;
use crate::Warning;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    HintsDirectiveRequiresNumber,
    HintsDirectiveRepeated,

    /// Stores the token (hopefully Token::Identifier) of the question
    /// which was invalid.
    InvalidQuestionType(Token),

    /// Stores the token which we got instead
    ExpectedQuestionOrDirective(Token),
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ErrorReport {
    pub errors: Vec<Error>,
    pub warnings: Vec<Warning>
}

impl ErrorReport {
    pub fn new() -> ErrorReport {
        ErrorReport {
            errors: vec![],
            warnings: vec![],
        }
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

