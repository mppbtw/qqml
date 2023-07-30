use crate::Warning;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    pub text: Option<String>,
    pub max_marks: Option<usize>,
    pub answers: Vec<MultichoiceAnswer>,
    pub hints: Vec<String>,
    pub chosen_answer: Option<String>,
    pub warnings: Vec<Warning>,
    pub line: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    pub text: Option<String>,
    pub marks: Option<usize>,
    pub explanation: Option<String>,
}

impl Default for MultichoiceAnswer {
    fn default() -> Self {
        Self {
            text: Some("".to_owned()),
            marks: Some(0),
            explanation: None,
        }
    }
}
