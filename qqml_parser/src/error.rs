use crate::Warning;
use qqml_lexer::Token;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    HintsDirectiveRequiresNumber,
    HintsDirectiveRepeated,

    ExpectedLSquirlyForQuestion(Token),
    ExpectedRParenForQuestionMaxMark(Token),
    ExpectedLParenForQuestionMaxMark(Token),
    ExpectedNumberForQuestionMaxMark(Token),
    ExpectedQuestionText(Token),
    ExpectedSemicolonAfterHintsDirective(Token),
    ExpectedQuestionOrDirective(Token),
    ExpectedQuestionType(Token),

    UnexpectedAnswerToken(Token, Vec<Token>),
    ExpectedAnswerText(Token),

    /// Stores the token (hopefully Token::Identifier) of the question
    /// which was invalid.
    InvalidQuestionType(Token),
}

#[derive(Default, Clone, PartialEq, Eq)]
pub struct ErrorReport {
    pub errors: Vec<Error>,
    pub warnings: Vec<Warning>,
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
