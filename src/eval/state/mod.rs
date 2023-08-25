#[cfg(test)]
mod test;

use crate::eval::render::*;
use crate::json::lexer::*;
use crate::json::parser::JsonConstructionError;
use crate::json::parser::*;
use crate::parser::Question;
use rtermutils::*;

#[derive(Debug, Clone)]
pub struct StateConstructor {
    pub max_hints: usize,
    pub questions: Vec<Question>,
    pub path_to_source: Option<String>,
}
impl StateConstructor {
    pub fn construct(self) -> State {
        State {
            max_hints: self.max_hints,
            questions: self.questions,
            path_to_source: self.path_to_source,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    pub hints_used: usize,
    pub max_hints: usize,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub path_to_source: Option<String>,
    questions_len: usize,
    current_hints_available: usize,
    cols: usize,
    has_watched_final_cutsene: bool,

    // Used only for purpose of handing references to TUI components
    current_achieved_marks: usize,
}

#[allow(clippy::field_reassign_with_default)]
impl State {
    pub fn create_screen(&mut self) -> Screen {
        self.cols = unsafe { clear_screen_with_width() } as usize;
        self.questions_len = self.questions.len();
        self.current_hints_available = match &self.questions[self.current_question_index] {
            Question::Multichoice(d) => d.hints.len() - d.used_hints,
            _ => 0,
        };
        let mut s = Screen::default();

        // Some components which are almost universally wanted
        s.version_line = Some(VersionLine { cols: &self.cols });
        s.q_select_line = Some(QuestionSelectLine {
            cols: &self.cols,
            max_questions: &self.questions_len,
            current_question: &self.current_question_index,
        });

        // Other components are added depending on the state
        match &self.path_to_source {
            Some(p) => {
                s.pathline = Some(PathLine {
                    path: p,
                    cols: &self.cols,
                })
            }
            None => (),
        };

        if let Question::Multichoice(d) = &self.questions[self.current_question_index] {
            if d.used_hints != 0 {
                s.hints_body = Some(HintsBody {
                    hints: &d.hints[0..d.used_hints],
                })
            }

            if d.is_answered {
                s.hints_line = Some(HintsLine {
                    max_hints: &self.max_hints,
                    hints_available: &self.current_hints_available,
                    hints_used_total: &self.hints_used,
                    is_answered: &true,
                });

                s.question_result_body = Some(QuestionResultBody {
                    cols: &self.cols,
                    answers: &d.answers,
                });

                self.current_achieved_marks = 0;
                for a in d.answers.iter() {
                    if a.is_chosen {
                        self.current_achieved_marks += a.marks;
                    }
                }

                s.question_result_line = Some(QuestionResultLine {
                    max_marks: &d.max_marks,
                    question: &d.text,
                    achieved_marks: &self.current_achieved_marks,
                });
            } else {
                s.hints_line = Some(HintsLine {
                    max_hints: &self.max_hints,
                    hints_available: &self.current_hints_available,
                    hints_used_total: &self.hints_used,
                    is_answered: &false,
                });

                s.question_line = Some(QuestionLine { q: d });
                s.question_body = Some(QuestionBody {
                    answers: (d
                        .answers
                        .iter()
                        .map(|d| (d.text.clone().unwrap(), d.is_chosen))
                        .collect()),
                    selected: &d.selected_answer,
                })
            }
        };

        s
    }

    pub fn every_question_answered(&self) -> bool {
        for q in self.questions.iter() {
            if let Question::Multichoice(d) = q {
                if !d.is_answered {
                    return false;
                }
            }
        }
        true
    }

    pub fn to_json(&self) -> String {
        let mut output = String::new();
        output += &format!(
            "{{\"questions\": [{}]",
            self.questions
                .iter()
                .map(|q| q.to_json())
                .collect::<Vec<String>>()
                .join(",")
        );
        output += "}";
        output
    }

    pub fn get_max_marks(&self) -> usize {
        self.questions
            .iter()
            .map(|q| match q {
                Question::Multichoice(d) => d.max_marks,
                _ => 0,
            })
            .sum()
    }

    pub fn achieved_marks(&self) -> usize {
        self.questions
            .iter()
            .map(|q| match q {
                Question::Multichoice(d) => {
                    if d.is_answered {
                        d.answers
                            .iter()
                            .map(|a| if a.is_chosen { a.marks } else { 0 })
                            .sum()
                    } else {
                        0
                    }
                }
                _ => 0,
            })
            .sum()
    }
    pub fn watch_final_cutsene(&mut self) {
        self.has_watched_final_cutsene = true;
    }

    pub fn has_watched_final_cutsene(&self) -> bool {
        self.has_watched_final_cutsene
    }

    #[allow(unused)]
    pub fn from_json(input: String) -> Result<Self, JsonConstructionError> {
        let mut lexer = Lexer::new(input);
        let json = parse(&mut lexer)?;

        let questions = if let Some(q) = json.get_ident("questions") {
            match &q.value {
                JsonType::Array(a) => {
                    let mut questions: Vec<Question> = vec![];
                    for v in a.values.iter() {
                        questions.push(Question::from_json(v)?);
                    }
                    questions
                }
                _ => return Err(JsonConstructionError::SemanticError),
            }
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        Ok(State {
            questions,
            ..Default::default()
        })
    }
}
