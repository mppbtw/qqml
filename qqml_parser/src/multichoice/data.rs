use crate::Warning;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    pub text: Option<String>,
    pub max_marks: Option<usize>,
    pub answers: Vec<MultichoiceAnswer>,
    pub hints: Vec<String>,
    pub used_hints: usize,
    pub selected_answer: usize,
    pub warnings: Vec<MultichoiceWarning>,
    pub line: usize,
    pub is_answered: bool,
}

impl MultichoiceData {
    pub fn validate(&mut self) {
        let total_mark = self.get_total_marks();
        if self.max_marks.unwrap() > total_mark {
            self.warnings.push(MultichoiceWarning::MaxMarkImpossible(
                self.max_marks.unwrap(),
                total_mark,
            ));
        }
        match self.answers.len() {
            0 => self
                .warnings
                .push(MultichoiceWarning::NoAnswersForMultichoiceQuestion),
            1 => self
                .warnings
                .push(MultichoiceWarning::OnlyOneAnswerForMultichoiceQuestion),
            _ => (),
        }
    }

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

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum MultichoiceWarning {
    /// The max mark and the
    /// maximum possible mark.
    MaxMarkImpossible(usize, usize),
    OnlyOneAnswerForMultichoiceQuestion,
    NoAnswersForMultichoiceQuestion,
}

impl Into<Vec<Warning>> for MultichoiceData {
    fn into(self) -> Vec<Warning> {
        self.warnings
            .to_vec()
            .iter()
            .map(|w| match w {
                MultichoiceWarning::OnlyOneAnswerForMultichoiceQuestion => {
                    Warning::OnlyOneAnswerForMultichoiceQuestion(self.clone())
                }
                MultichoiceWarning::MaxMarkImpossible(..) => {
                    Warning::MaxMarkImpossible(self.clone())
                }
                MultichoiceWarning::NoAnswersForMultichoiceQuestion => {
                    Warning::NoAnswersForMultichoiceQuestion(self.clone())
                }
            })
            .collect()
    }
}
