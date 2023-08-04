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

impl MultichoiceData {
    pub fn validate(&self) -> Result<(), Vec<Warning>> {
        let total_mark = self.get_total_marks();
        let mut warnings: Vec<Warning> = vec![];
        if self.max_marks.unwrap() > total_mark {
            warnings.push(Warning::MaxMarkImpossible(self.max_marks.unwrap(), total_mark));
        }

        if warnings.is_empty() {
            Ok(())
        } else {
            Err(warnings)
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
}

impl Default for MultichoiceAnswer {
    fn default() -> Self {
        Self {
            text: Some("".to_owned()),
            marks: 0,
            explanation: None,
        }
    }
}
