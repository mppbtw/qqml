#[derive(Debug, Default)]
pub struct MultichoiceData {
    pub text: String,
    pub max_marks: usize,
    pub answers: Vec<MultichoiceAnswer>,
}

#[derive(Debug, Default)]
pub struct MultichoiceAnswer {
    pub text: String,
    pub marks: usize,
    pub explanation: Option<String>,
}

impl MultichoiceData {
    pub fn new(text: String, max_marks: usize, answers: Vec<MultichoiceAnswer>) -> MultichoiceData {
        Self {
            text,
            max_marks,
            answers,
        }
    }
}
