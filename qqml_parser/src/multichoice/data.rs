#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    text: Option<String>,
    max_marks: Option<usize>,
    answers: Vec<MultichoiceAnswer>,
    hints: Vec<String>,
    chosed_answer: Option<String>,
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    text: Option<String>,
    marks: Option<usize>,
    explanation: Option<String>,
}

impl MultichoiceAnswer {
    pub fn set_explanation<S: Into<String>>(&mut self, explanation: S) {
        if self.explanation.is_none() {
            self.explanation = Some(explanation.into());
        }
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        if self.text.is_none() {
            self.text = Some(text.into());
        }
    }

    pub fn set_marks(&mut self, marks: usize) {
        if self.marks.is_none() {
            self.marks = Some(marks);
        }
    }

    pub fn new() -> MultichoiceAnswer {
        Self {
            text: None,
            marks: None,
            explanation: None,
        }
    }
}

impl MultichoiceData {
    pub fn add_answer(&mut self, answer: MultichoiceAnswer) {
        self.answers.push(answer);
    }

    pub fn count_answers(&self) -> usize {
        self.answers.len()
    }

    pub fn add_hint<S: Into<String>>(&mut self, hint: S) {
        self.hints.push(hint.into());
    }

    pub fn set_text<S: Into<String>>(&mut self, text: S) {
        if self.text.is_none() {
            self.text = Some(text.into());
        }
    }

    pub fn set_max_marks(&mut self, max_marks: usize) {
        if self.max_marks.is_none() {
            self.max_marks = Some(max_marks);
        }
    }

    pub fn new() -> MultichoiceData {
        Self {
            ..Default::default()
        }
    }
}
