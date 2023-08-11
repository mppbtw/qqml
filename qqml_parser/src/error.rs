use std::fmt;

use qqml_lexer::LexerError;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    HintsDirectiveRequiresNumber,
    HintsDirectiveRepeated,

    /// Stores the data of where the literal began.
    UnterminatedLiteral(TokenData),
    IntegerTooLarge(TokenData),

    // Syntax for multichoice
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

    // Semantics for multichoice
    ImpossibleMaxMark(Token),
    OnlyOneMultichoiceAnswer(Token),
    NoMultichoiceAnswers(Token),


    // Multichoice answer syntax
    UnexpectedAnswerToken(Token, Vec<Token>),
    ExpectedAnswerText(Token),
    ExpectedAnswerSemicolon(Token),
    ExpectedAnswerExplanationText(Token),
    ExpectedRParenForAnswerMark(Token),
    ExpectedNumberForAnswerMark(Token),

    // General semantics
    MaxMarkIsZero(Token),

    /// Stores the token (hopefully Token::Identifier) of the question
    /// which was invalid.
    InvalidQuestionType(Token),
}
impl Error {
    pub fn get_token_data(&self) -> TokenData {
        match self {
            Self::OnlyOneMultichoiceAnswer(t) => t.get_data(),
            Self::NoMultichoiceAnswers(t) => t.get_data(),
            Self::MaxMarkIsZero(t) => t.get_data(),
            Self::ExpectedSemicolonAfterHintsDirective(t) => t.get_data(),
            Self::ExpectedQuestionOrDirective(t) => t.get_data(),
            Self::ExpectedQuestionType(t) => t.get_data(),
            Self::ExpectedCommaInHintsList(t) => t.get_data(),
            Self::InvalidQuestionType(t) => t.get_data(),
            Self::IntegerTooLarge(t) => t,
            Self::ExpectedHintText(t) => t.get_data(),
            Self::ImpossibleMaxMark(t) => t.get_data(),
            Self::ExpectedAnswerText(t) => t.get_data(),
            Self::UnexpectedBodyToken(t) => t.get_data(),
            Self::UnexpectedAnswerToken(t, _) => t.get_data(),
            Self::ExpectedAnswerSemicolon(t) => t.get_data(),
            Self::UnterminatedLiteral(t) => t,
            Self::ExpectedLSquirlyForQuestion(t) => t.get_data(),
            Self::ExpectedAnswerExplanationText(t) => t.get_data(),
            Self::ExpectedRParenForAnswerMark(t) => t.get_data(),
            Self::ExpectedRParenForQuestionMaxMark(t) => t.get_data(),
            Self::ExpectedLParenForQuestionMaxMark(t) => t.get_data(),
            Self::ExpectedNumberForAnswerMark(t) => t.get_data(),
            Self::ExpectedNumberForQuestionMaxMark(t) => t.get_data(),
            Self::ExpectedQuestionText(t) => t.get_data(),
            _ => &TokenData { col: 0, line: 0 }
        }
        .clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            _ => "ERROR MESSAGE PLACEHOLDER".to_owned(),
        };

        let _ = msg.replace(
            "doc",
            "https://github.com/MrPiggyPegasus/yarr/blob/main/doc/QQML.md",
        );
        write!(f, "{}", msg)
    }
}

#[derive(Default, Clone, PartialEq, Debug)]
pub struct ErrorReport {
    pub errors: Vec<Error>,
}
impl From<LexerError> for ErrorReport {
    fn from(value: LexerError) -> Self {
        println!("\n\n\n\nlawkhdlakwhdl");
        Self {
            errors: vec![match value {
                LexerError::IntegerTooLarge(d) => Error::IntegerTooLarge(d),
                LexerError::UnterminatedStringError(d) => Error::UnterminatedLiteral(d),
            }],
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
    }
}
