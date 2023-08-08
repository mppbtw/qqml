use crate::render::*;
use qqml_parser::Question;
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

#[derive(Debug, Clone, Default)]
pub struct State {
    pub hints_used: usize,
    pub max_hints: usize,
    pub questions: Vec<Question>,
    pub current_question_index: usize,
    pub path_to_source: Option<String>,
    questions_len: usize,
    current_hints_available: usize,
    cols: usize,

    // Used only for purpose of handing references to TUI components
    current_achieved_marks: usize,
}
impl State {
    pub fn create_screen(&mut self) -> Screen {
        self.cols = unsafe { clear_screen_with_width() } as usize;
        self.questions_len = self.questions.len();
        self.current_hints_available = match &self.questions[self.current_question_index] {
            Question::Multichoice(d) => d.hints.len(),
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
                    path: &p,
                    cols: &self.cols,
                })
            }
            None => (),
        };

        match &self.questions[self.current_question_index] {
            Question::Multichoice(d) => {
                if d.is_answered {
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
                    s.question_line = Some(QuestionLine { q: &d });
                    s.question_body = Some(QuestionBody {
                        answers: (d
                            .answers
                            .iter()
                            .map(|d| (d.text.clone().unwrap(), d.is_chosen))
                            .collect()),
                        selected: &d.selected_answer,
                    })
                }
            }
            // Other question types are not yet supported
            _ => (),
        };

        s
    }
}
