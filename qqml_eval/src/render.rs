use qqml_parser::Question;

use crate::Target;

pub trait Render {
    fn render(&self) -> String;
}

pub struct Screen {
    pub pathline: Option<PathLine>,
    pub version_line: VersionLine,
    pub q_select_line: QuestionSelectLine,
    pub question_line: QuestionLine,
    pub question_body: QuestionBody,
    pub hints_line: HintsLine,
    pub hints_body: Option<HintsBody>,
}
impl From<Target> for Screen {
    fn from(value: Target) -> Self {
        let pathline = match value.path_to_source {
            Some(p) => Some(PathLine { path: p }),
            None => None,
        };
        let current_question = value.questions.get(value.current_question).unwrap().clone();
        Self {
            hints_line: HintsLine {
                max_hints: value.max_hints,
                hints_available: {
                    match &current_question {
                        Question::Multichoice(d) => d.hints.len() - d.used_hints,
                        _ => 0,
                    }
                },
                hints_used_total: value.hints_used,
            },
            question_body: QuestionBody {
                answers: match &current_question {
                    Question::Multichoice(d) => {
                        let mut answers = vec![];
                        for a in &d.answers {
                            answers.push(a.text.clone().unwrap());
                        }
                        answers
                    },
                    _ => vec![],
                },
                selected: match &current_question {
                    Question::Multichoice(d) => d.chosen_answer,
                    _ => 0,
                }
            },
            pathline,
            question_line: QuestionLine {
                q: current_question,
            },
            q_select_line: QuestionSelectLine {
                max_questions: value.questions.len(),
                current_question: value.current_question,
            },
            hints_body: {
                Some(HintsBody {
                    hints: match value.questions.get(value.current_question).unwrap().clone() {
                        Question::Multichoice(d) => d.hints[0..d.used_hints].to_vec(),
                        _ => vec![],
                    },
                })
            },
            version_line: VersionLine {},
        }
    }
}
impl Render for Screen {
    fn render(&self) -> String {
        let mut output = String::new();
        output += &self.version_line.render();
        output += "\n";
        match &self.pathline {
            Some(p) => {
                output += &p.render();
                output += "\n";
            }
            None => (),
        };
        output += "\n";
        output += &self.q_select_line.render();
        output += "\n\n";
        output += &self.question_line.render();
        output += "\n";
        output += &self.question_body.render();
        output += "\n";
        output += &self.hints_line.render();
        output += "\n";
        match &self.hints_body {
            Some(h) => output += &h.render(),
            None => (),
        };

        output
    }
}

pub struct QuestionBody {
    pub answers: Vec<String>,
    pub selected: usize,
}
impl Render for QuestionBody {
    fn render(&self) -> String {
        let mut output = String::new();
        for (i, a) in self.answers.iter().enumerate() {
            if i == self.selected {
                output += &format!("   {} <", a);
            } else {
                output += &("   ".to_owned() + a);
            }
            output += "\n";
        };
        output
    }
}

pub struct QuestionLine {
    pub q: Question,
}
impl Render for QuestionLine {
    fn render(&self) -> String {
        let mut output = match &self.q {
            Question::String() => "String questions are not supported.".to_owned(),
            Question::Calculation() => "Calculation questions are not supported.".to_owned(),
            Question::Multichoice(m) => {
                format!(
                    "{} ({})",
                    m.text.clone().unwrap(),
                    m.max_marks.clone().unwrap()
                )
            }
        };
        output
    }
}

pub struct HintsBody {
    pub hints: Vec<String>,
}
impl Render for HintsBody {
    fn render(&self) -> String {
        let mut output = String::new();
        for i in &self.hints {
            output += &("  ".to_owned() + i);
            output += "\n\n";
        }
        output
    }
}

pub struct PathLine {
    pub path: String,
}
impl Render for PathLine {
    fn render(&self) -> String {
        self.path.clone()
    }
}

pub struct QuestionSelectLine {
    pub max_questions: usize,
    pub current_question: usize,
}
impl Render for QuestionSelectLine {
    fn render(&self) -> String {
        format!(
            "<--({} / {})-->",
            &self.current_question.to_string(),
            &self.max_questions.to_string()
        )
    }
}

pub struct HintsLine {
    pub max_hints: usize,
    pub hints_used_total: usize,
    pub hints_available: usize,
}
impl Render for HintsLine {
    fn render(&self) -> String {
        format!(
            "Hints (used {}/{}, {} available for this question):",
            &self.hints_used_total, &self.max_hints, &self.hints_available
        )
    }
}

pub struct VersionLine;
impl Render for VersionLine {
    fn render(&self) -> String {
        let version = env!("CARGO_PKG_VERSION");
        let output = format!("QQML Version {}, press ? for help", version);
        output
    }
}
