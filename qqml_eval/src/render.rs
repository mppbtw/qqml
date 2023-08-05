use qqml_parser::Question;

use crate::utils::get_term_size;
use crate::Error;
use crate::Target;

pub trait Render<E> {
    fn render(&self) -> Result<String, E>;
}

pub struct Screen {
    pub pathline: Option<PathLine>,
    pub version_line: VersionLine,
    pub q_select_line: QuestionSelectLine,
    pub question_line: QuestionLine,
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
impl Render<Error> for Screen {
    fn render(&self) -> Result<String, Error> {
        let mut output = String::new();
        output += &self.version_line.render()?;
        output += "\n";
        match &self.pathline {
            Some(p) => {
                output += &p.render()?;
                output += "\n";
            }
            None => (),
        };
        output += "\n";
        output += &self.q_select_line.render()?;
        output += "\n\n";
        output += &self.question_line.render()?;
        /* question body */
        output += "\n";
        output += &self.hints_line.render()?;
        output += "\n";
        match &self.hints_body {
            Some(h) => output += &h.render()?,
            None => (),
        };

        Ok(output)
    }
}

pub struct QuestionLine {
    pub q: Question,
}
impl Render<Error> for QuestionLine {
    fn render(&self) -> Result<String, Error> {
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
        Ok(output)
    }
}

pub struct HintsBody {
    pub hints: Vec<String>,
}
impl Render<Error> for HintsBody {
    fn render(&self) -> Result<String, Error> {
        let mut output = String::new();
        for i in &self.hints {
            output += i;
            output += "\n\n";
        }
        Ok(output)
    }
}

pub struct PathLine {
    pub path: String,
}
impl Render<Error> for PathLine {
    fn render(&self) -> Result<String, Error> {
        Ok(pad_to_width(&self.path, get_term_size()?.width)?)
    }
}

pub struct QuestionSelectLine {
    pub max_questions: usize,
    pub current_question: usize,
}
impl Render<Error> for QuestionSelectLine {
    fn render(&self) -> Result<String, Error> {
        let selector = format!(
            "<--({} / {})-->",
            &self.current_question.to_string(),
            &self.max_questions.to_string()
        );

        Ok(pad_to_width(selector, get_term_size()?.width)?)
    }
}

pub struct HintsLine {
    pub max_hints: usize,
    pub hints_used_total: usize,
    pub hints_available: usize,
}
impl Render<Error> for HintsLine {
    fn render(&self) -> Result<String, Error> {
        Ok(format!(
            "Hints: (used {}/{}, {} available for this question)",
            &self.hints_used_total, &self.max_hints, &self.hints_available
        ))
    }
}

pub struct VersionLine;
impl Render<Error> for VersionLine {
    fn render(&self) -> Result<String, Error> {
        let version = env!("CARGO_PKG_VERSION");
        let mut output = format!("QQML Version {}, press ? for help", version);
        output = pad_to_width(output, get_term_size()?.width)?;
        Ok(output)
    }
}

fn pad_to_width<S: Into<String>>(str: S, width: usize) -> Result<String, Error> {
    let mut output = String::new();
    let str: String = str.into();

    if str.len() > width {
        return Err(Error::TerminalTooSmall);
    }
    if str.len() == width {
        return Ok(str);
    }

    let total_needed_spaces = width - str.len();
    let left_spaces = total_needed_spaces / 2;
    let right_spaces = total_needed_spaces - left_spaces;

    for i in 0..left_spaces {
        output += " ";
    }
    output += &str;
    for i in 0..right_spaces {
        output += " ";
    }

    Ok(output)
}
