use std::fmt;

use qqml_lexer::LexerError;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    HintsDirectiveRequiresNumber(Token),
    HintsDirectiveRepeated(Token),

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
            _ => &TokenData { col: 0, line: 0 },
        }
        .clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::ExpectedQuestionText(t) => {
                format!("Expected text for the question, found {}.", t)
            }
            Self::MaxMarkIsZero(_) => {
                "If you ever feel stupid, just remember that one complete and utter ".to_owned()
                    + "baffoon decided to make a question worth 0 marks. Sacriligious."
            }
            Self::IntegerTooLarge(_) => format!(
                "The parsing process could not continue because the maximum \"
                representable number in QQML on your machine is {}.",
                usize::MAX
            ),
            Self::ExpectedHintText(t) => {
                if matches!(t, Token::Comma(_)) {
                    "Please, to help with the current financial crisis, dont use extra commas."
                        .into()
                } else {
                    format!("Expected text for this hint, found {}", t)
                }
            }
            Self::HintsDirectiveRepeated(_) => {
                "Only one hints directive is allowed per file.".into()
            }
            Self::ImpossibleMaxMark(_) => {
                "The maximum mark for this question is not achievable.".into()
            }
            Self::UnterminatedLiteral(_) => {
                "What goes up must come down. This string is never terminated".into()
            }
            Self::ExpectedAnswerText(t) => format!("Expected text for this answer, found {}.", t),
            Self::NoMultichoiceAnswers(_) | Self::OnlyOneMultichoiceAnswer(_) => {
                "So the point of a multiple choice question is there ".to_owned()
                    + "are multiple choices to pick, right?"
            }
            Self::UnexpectedBodyToken(t) => format!("Who put that there? {}. See doc#body.", t),
            Self::HintsDirectiveRequiresNumber(t) => {
                format!("Expected a max hints number, found {}.", t)
            }
            Self::ExpectedAnswerSemicolon(_) => "The answer should end in a semicolon.".into(),
            Self::ExpectedCommaInHintsList(_) => {
                "The list of hints should be comma separated. do u even english bru?".into()
            }
            Self::InvalidQuestionType(_) => {
                format!("No such question type keyword, see doc#questions")
            }
            Self::ExpectedLParenForQuestionMaxMark(t) => format!(
                "Expected a '(' to set the max mark of the question, found {}. See doc#questions",
                t
                ),
            Self::ExpectedRParenForQuestionMaxMark(t) => format!(
                "Expected a ')' to finish setting the max mark of the question, found {}. See doc#questions",
                t
                ),
            Self::ExpectedNumberForAnswerMark(t) => format!(
                "Expected a number for the answer's mark, found {}. See doc#body",
                t
                ),
            Self::ExpectedNumberForQuestionMaxMark(t) => format!(
                "Expected a number for the question's max mark, found {}. See doc#questions",
                t
                ),
            Self::ExpectedRParenForAnswerMark(t) => format!(
                "Expected a ')' to finish setting the mark of this answer, found {}. See doc#body",
                t
                ),
            Self::ExpectedLSquirlyForQuestion(t) => format!(
                "Expected a '{{' to start the body of the question, found {}. See doc#body",
                t
                ),
            Self::ExpectedSemicolonAfterHintsDirective(t) =>
                format!("Expected a semicolon after the hints directive, found {}. See doc#hints", t),
            Self::ExpectedQuestionOrDirective(t) => format!(
                "Expected either a question (ask) or a directive e.g hints, found {}. See doc#questions and doc#hints",
                t
                ),
            Self::ExpectedQuestionType(t) => format!(
                "Expected a question type keyword e.g. 'multichoice', found {}. See doc#questions",
                t
                ),
            Self::UnexpectedAnswerToken(t, _) => format!(
                "Unexpected answer token: {}. See doc#body",
                t
                ),
            Self::ExpectedAnswerExplanationText(t) => format!(
                "Expected the text for your answer explanation, found {}. See doc#body",
                t
                ),
        }.replace(
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
