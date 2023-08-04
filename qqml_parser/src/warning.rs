use crate::MultichoiceData;

/// This is a collector of all of the question warnings.
/// The only purpose of this is when the parse function returns
/// and error, but the consumer (qqml_eval) wants to see all of
/// the warnings anyway.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Warning {
    // Multichoice stuff
    MaxMarkImpossible(MultichoiceData),
    OnlyOneAnswerForMultichoiceQuestion(MultichoiceData),
    NoAnswersForMultichoiceQuestion(MultichoiceData),
}
