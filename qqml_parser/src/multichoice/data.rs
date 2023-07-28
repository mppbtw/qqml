use crate::Error;
use crate::Warning;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    pub text: Option<String>,
    pub max_marks: Option<usize>,
    pub answers: Vec<MultichoiceAnswer>,
    pub hints: Vec<String>,
    pub chosed_answer: Option<String>,
    pub warnings: Vec<Warning>,
    pub errors: Vec<Error>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    pub text: Option<String>,
    pub marks: Option<usize>,
    pub explanation: Option<String>,
}
