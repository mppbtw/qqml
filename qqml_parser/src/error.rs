use qqml_lexer::Token;

#[derive(Debug, Clone)]
pub enum Error {
    HintsDirectiveRequiresNumber,
    HintsDirectiveRepeated,

    /// Stores the token which we got instead
    ExpectedQuestionOrDirective(Token),
}
