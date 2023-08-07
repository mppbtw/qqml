use qqml_parser::Question;

use crate::render::*;

#[derive(Debug, Clone)]
pub struct StateConstructor {
    pub hints_used: usize,
    pub max_hints: usize,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub path_to_source: Option<String>,
}
impl StateConstructor {
    pub fn construct(self) -> State {
        State {
            hints_used: self.hints_used,
            max_hints: self.max_hints,
            questions: self.questions,
            path_to_source: self.path_to_source,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    pub hints_used: usize,
    pub max_hints: usize,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub path_to_source: Option<String>,
    questions_len: usize,
    current_hints_available: usize,
}
impl State {
    pub fn create_screen(&self) -> Screen {
        Screen {
            pathline: {
                match &self.path_to_source {
                    Some(p) => Some(PathLine { path: &p }),
                    None => None,
                }
            },
            hints_line: HintsLine {
                max_hints: &self.max_hints,
                hints_available: {
                    match &self.questions[self.current_question_index] {
                        Question::Multichoice(_) => &self.current_hints_available,
                        _ => &0,
                    }
                },
                hints_used_total: &self.hints_used,
            },
            version_line: VersionLine,
            q_select_line: QuestionSelectLine {
                max_questions: &(self.questions_len),
                current_question: &self.current_question_index,
            },
            question_line: QuestionLine { q: &self.questions[self.current_question_index] },
            question_body: QuestionBody {
                answers: {
                    match &self.questions[self.current_question_index] {
                        Question::Multichoice(d) => d.answers.iter().map(|a| a.text.clone().unwrap()).collect(),
                        _ => vec![],
                    }.clone() // We can't move any data from this struct
                },
                selected: {
                    match &self.questions[self.current_question_index] {
                        Question::Multichoice(d) => &d.chosen_answer,
                        _ => &0,
                    }
                }
            },
            hints_body: {
                match &self.questions[self.current_question_index] {
                    Question::Multichoice(d) => {
                        Some(HintsBody {
                            hints: &d.hints[0..d.used_hints],
                        })
                    }
                    _ => None
                }
            }
        }
    }
}
