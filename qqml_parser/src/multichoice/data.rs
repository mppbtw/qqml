use crate::Error;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    text: Option<String>,
    max_marks: Option<usize>,
    answers: Vec<MultichoiceAnswer>,
    hints: Vec<String>,
    chosed_answer: Option<String>,
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

    pub fn get_text(&self) -> String {
        self.text.clone().unwrap()
    }

    pub fn new() -> MultichoiceData {
        Self {
            ..Default::default()
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        match &self.text {
            Some(t) => {
                if t.is_empty() {
                    return Err(Error::EmptyQuestionText);
                }
            }
            None => return Err(Error::NoQuestionText),
        }

        match &self. max_marks {
            Some(m) => if *m < 1 {
                return Err(Error::MaximumMarkNotPositive);
            },
            None => return Err(Error::NoMaxMark),
        }

        let mut total_marks = 0;
        for a in self.answers.iter() {
            match a.marks {
                Some(n) => total_marks += n,
                None => continue,
            }
        }
        if total_marks < self.max_marks.unwrap() {
            return Err(Error::MaxMarksNotPossible);
        }
        for a in self.answers.iter() {
            a.validate()?;
        }

        Ok(())
    }
}

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    text: Option<String>,
    marks: Option<usize>,
    explanation: Option<String>,
}

impl MultichoiceAnswer {
    pub fn validate(&self) -> Result<(), Error> {
        Ok(())
    }

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
