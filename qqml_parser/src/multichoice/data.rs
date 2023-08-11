#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    pub text: String,
    pub max_marks: usize,
    pub answers: Vec<MultichoiceAnswer>,
    pub hints: Vec<String>,
    pub used_hints: usize,
    pub selected_answer: usize,
    pub line: usize,
    pub is_answered: bool,
}

impl MultichoiceData {
    pub fn get_total_marks(&self) -> usize {
        let mut total: usize = 0;
        for i in self.answers.to_vec() {
            total += i.marks;
        }
        total
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    pub text: Option<String>,
    pub marks: usize,
    pub explanation: Option<String>,
    pub is_chosen: bool,
}

impl Default for MultichoiceAnswer {
    fn default() -> Self {
        Self {
            text: Some("".to_owned()),
            marks: 0,
            explanation: None,
            is_chosen: false,
        }
    }
}
