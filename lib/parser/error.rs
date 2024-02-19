//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

use std::fmt;

use crate::lexer::core::LexerError;
use crate::lexer::token::Token;
use crate::lexer::token::TokenData;

#[derive(Debug, Clone, PartialEq)]
pub enum Error {
    ExpectedNumberForAnswerMark(Token),
    HintsDirectiveRequiresNumber(Token),
    HintsDirectiveRepeated(Token),

    /// Stores the data of where the literal began.
    UnterminatedLiteral(TokenData),
    IntegerTooLarge(TokenData),

    // Syntax for multichoice
    ExpectedRSquirlyForQuestion(Token),
    UnexpectedBodyToken(Token),
    ExpectedLSquirlyForQuestion(Token),
    ExpectedRParenForQuestionMaxMark(Token),
    ExpectedLParenForQuestionMaxMark(Token),
    ExpectedNumberForQuestionMaxMark(Token),
    ExpectedQuestionText(Token),
    ExpectedSemicolonAfterHintsDirective(Token),
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

    // General semantics
    MaxMarkIsZero(Token),

    /// Stores the token (hopefully Token::Identifier) of the question
    /// which was invalid.
    InvalidQuestionType(Token),
}
impl Error {
    pub fn is_eof(&self) -> bool {
        matches!(self.get_token(), Token::Eof(_))
    }

    pub fn get_token(&self) -> Token {
        match self {
            Self::ExpectedNumberForAnswerMark(t) => t.clone(),
            Self::OnlyOneMultichoiceAnswer(t) => t.clone(),
            Self::NoMultichoiceAnswers(t) => t.clone(),
            Self::MaxMarkIsZero(t) => t.clone(),
            Self::ExpectedSemicolonAfterHintsDirective(t) => t.clone(),
            Self::ExpectedQuestionType(t) => t.clone(),
            Self::ExpectedCommaInHintsList(t) => t.clone(),
            Self::InvalidQuestionType(t) => t.clone(),
            Self::ExpectedHintText(t) => t.clone(),
            Self::ImpossibleMaxMark(t) => t.clone(),
            Self::ExpectedAnswerText(t) => t.clone(),
            Self::UnexpectedBodyToken(t) => t.clone(),
            Self::UnexpectedAnswerToken(t, _) => t.clone(),
            Self::ExpectedAnswerSemicolon(t) => t.clone(),
            Self::ExpectedLSquirlyForQuestion(t) => t.clone(),
            Self::ExpectedAnswerExplanationText(t) => t.clone(),
            Self::ExpectedRParenForAnswerMark(t) => t.clone(),
            Self::ExpectedRParenForQuestionMaxMark(t) => t.clone(),
            Self::ExpectedLParenForQuestionMaxMark(t) => t.clone(),
            Self::ExpectedNumberForQuestionMaxMark(t) => t.clone(),
            Self::ExpectedQuestionText(t) => t.clone(),
            Self::ExpectedRSquirlyForQuestion(t) => t.clone(),
            Self::HintsDirectiveRequiresNumber(t) => t.clone(),
            Self::HintsDirectiveRepeated(t) => t.clone(),

            Self::IntegerTooLarge(t) => Token::Illegal(t.clone()),
            Self::UnterminatedLiteral(t) => Token::Illegal(t.clone()),
        }
    }

    pub fn get_token_data(&self) -> TokenData {
        self.get_token().get_data().clone()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            Self::ExpectedNumberForAnswerMark(t) => {
                format!("Expected numbeer for the answer's mark, found {}.", t)
            }
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
            Self::InvalidQuestionType(_) => "No such question type keyword, see doc#questions".into(),
            Self::ExpectedLParenForQuestionMaxMark(t) => format!(
                "Expected a '(' to set the max mark of the question, found {}. See doc#questions",
                t
                ),
            Self::ExpectedRParenForQuestionMaxMark(t) => format!(
                "Expected a ')' to finish setting the max mark of the question, found {}. See doc#questions",
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
            Self::ExpectedRSquirlyForQuestion(t) => format!(
                "Expected a '}}' to end the question, found {}. See doc#body",
                t
                ),
        }.replace(
            "doc",
            "https://github.com/MrPiggyPegasus/yarr/blob/main/doc/QQML.md",
        );
        write!(f, "{}", msg)
    }
}

impl From<LexerError> for Error {
    fn from(value: LexerError) -> Self {
        match value {
            LexerError::IntegerTooLarge(t) => Self::IntegerTooLarge(t),
            LexerError::UnterminatedStringError(t) => Self::UnterminatedLiteral(t),
        }
    }
}
