use crate::parser::*;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BG_WHITE: &str = "\x1b[47m";
const ANSI_BLACK: &str = "\x1b[1;30m";
const ANSI_GREEN: &str = "\x1b[32m";
const ANSI_YELLOW: &str = "\x1b[0;33m";
const ANSI_RED: &str = "\x1b[31m";

pub trait Render {
    fn render(&self) -> String;
}

#[derive(Debug, Clone, Default)]
pub struct Screen<'a> {
    // Each component of the TUI is modular,
    // such that this system can be used to
    // render multiple different question types.
    pub pathline: Option<PathLine<'a>>,
    pub version_line: Option<VersionLine<'a>>,
    pub q_select_line: Option<QuestionSelectLine<'a>>,
    pub question_line: Option<QuestionLine<'a>>,
    pub question_body: Option<QuestionBody<'a>>,
    pub question_result_body: Option<QuestionResultBody<'a>>,
    pub hints_line: Option<HintsLine<'a>>,
    pub hints_body: Option<HintsBody<'a>>,
    pub question_result_line: Option<QuestionResultLine<'a>>,
}
impl Render for Screen<'_> {
    fn render(&self) -> String {
        let mut output = String::new();
        match &self.version_line {
            Some(c) => output += &(c.render() + "\n"),
            None => (),
        };
        match &self.pathline {
            Some(c) => output += &(c.render() + "\n"),
            None => (),
        };
        match &self.q_select_line {
            Some(c) => output += &(c.render() + "\n\n"),
            None => (),
        };
        match &self.question_line {
            Some(c) => output += &(c.render() + "\n\n"),
            None => (),
        };
        match &self.question_body {
            Some(c) => output += &(c.render() + "\n\n"),
            None => (),
        };
        match &self.question_result_line {
            Some(c) => output += &(c.render() + "\n\n"),
            None => (),
        };
        match &self.question_result_body {
            Some(c) => output += &(c.render() + "\n\n"),
            None => (),
        };
        match &self.hints_line {
            Some(c) => output += &(c.render() + "\n"),
            None => (),
        };
        match &self.hints_body {
            Some(c) => output += &c.render(),
            None => (),
        };

        output
    }
}

#[derive(Debug, Clone)]
pub struct QuestionResultLine<'a> {
    pub question: &'a String,
    pub achieved_marks: &'a usize,
    pub max_marks: &'a usize,
}
impl Render for QuestionResultLine<'_> {
    fn render(&self) -> String {
        if *self.achieved_marks == 0 {
            format!(
                "{}{} ({}/{}){}",
                self.question, ANSI_RED, self.achieved_marks, self.max_marks, ANSI_RESET
            )
        } else if self.achieved_marks == self.max_marks {
            format!(
                "{}{} ({}/{}){}",
                self.question, ANSI_GREEN, self.achieved_marks, self.max_marks, ANSI_RESET
            )
        } else {
            format!(
                "{}{} ({}/{}){}",
                self.question, ANSI_YELLOW, self.achieved_marks, self.max_marks, ANSI_RESET
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct QuestionResultBody<'a> {
    pub answers: &'a Vec<MultichoiceAnswer>,
    pub cols: &'a usize,
}
impl Render for QuestionResultBody<'_> {
    fn render(&self) -> String {
        let mut output = String::new();

        for a in self.answers.clone() {
            if a.is_chosen {
                if a.marks != 0 {
                    output += &format!(
                        "   {}{}{}{}",
                        ANSI_BG_WHITE,
                        ANSI_BLACK,
                        a.text.unwrap(),
                        ANSI_RESET
                    );
                    output += &format!(" {}(+{})", ANSI_GREEN, a.marks);
                } else {
                    output += &format!(
                        "   {}{}{}{}",
                        ANSI_BG_WHITE,
                        ANSI_BLACK,
                        a.text.unwrap(),
                        ANSI_RESET
                    );
                    output += &format!(" {}(X)", ANSI_RED);
                }
            } else if a.marks != 0 {
                output += &format!("   {}{} (+{})", a.text.unwrap(), ANSI_GREEN, a.marks);
            } else {
                output += &format!("   {}{} (X)", a.text.unwrap(), ANSI_RED);
            }
            if let Some(x) = a.explanation {
                output += " -> ";
                output += ANSI_RESET;
                output += &wrap_text_to_width(&x, self.cols / 2).unwrap();
            }
            output += ANSI_RESET;
            output += "\n";
        }

        output
    }
}

#[derive(Debug, Clone)]
pub struct QuestionBody<'a> {
    pub answers: Vec<(String, bool)>,
    pub selected: &'a usize,
}
impl Render for QuestionBody<'_> {
    fn render(&self) -> String {
        let mut output = String::new();
        for (i, a) in self.answers.iter().enumerate() {
            if a.1 {
                output += "  ";
                output += ANSI_BLACK;
                output += ANSI_BG_WHITE;
                output += &a.0;
                output += ANSI_RESET;
                if &i == self.selected {
                    output += " <";
                }
            } else if &i == self.selected {
                output += &format!("   {} <", a.0);
            } else {
                output += &("   ".to_owned() + &a.0);
            }
            output += "\n";
        }
        output
    }
}

#[derive(Debug, Clone)]
pub struct QuestionLine<'a> {
    pub q: &'a MultichoiceData,
}
impl Render for QuestionLine<'_> {
    fn render(&self) -> String {
        format!("{} ({})", self.q.text.clone(), self.q.max_marks)
    }
}

#[derive(Debug, Clone)]
pub struct HintsBody<'a> {
    pub hints: &'a [String],
}
impl Render for HintsBody<'_> {
    fn render(&self) -> String {
        let mut output = String::new();
        output += "\n";
        for hint in self.hints {
            output += &format!("   *  {}", hint);
            output += "\n\n";
        }
        output
    }
}

#[derive(Debug, Clone)]
pub struct PathLine<'a> {
    pub path: &'a String,
    pub cols: &'a usize,
}
impl Render for PathLine<'_> {
    fn render(&self) -> String {
        pad_to_width(self.path, *(self).cols).unwrap_or(self.path.clone())
    }
}

#[derive(Debug, Clone)]
pub struct QuestionSelectLine<'a> {
    pub max_questions: &'a usize,
    pub current_question: &'a usize,
    pub cols: &'a usize,
}
impl Render for QuestionSelectLine<'_> {
    fn render(&self) -> String {
        pad_to_width(
            &format!(
                "<--({} / {})-->",
                self.current_question + 1,
                &self.max_questions.to_string()
            ),
            *(self).cols,
        )
        .unwrap_or(format!(
            "<--({} / {})-->",
            self.current_question + 1,
            &self.max_questions.to_string()
        ))
    }
}

#[derive(Debug, Clone)]
pub struct HintsLine<'a> {
    pub max_hints: &'a usize,
    pub hints_used_total: &'a usize,
    pub hints_available: &'a usize,
    pub is_answered: &'a bool,
}
impl Render for HintsLine<'_> {
    fn render(&self) -> String {
        if *self.is_answered {
            format!("   Used {} hints:", &self.hints_used_total)
        } else {
            format!(
                "   Hints (used {}/{}, {} available for this question)",
                &self.hints_used_total, &self.max_hints, &self.hints_available
            )
        }
    }
}

#[derive(Debug, Clone)]
pub struct VersionLine<'a> {
    pub cols: &'a usize,
}
impl Render for VersionLine<'_> {
    fn render(&self) -> String {
        let version = env!("CARGO_PKG_VERSION");
        pad_to_width(
            &format!("QQML Version {}, press ? for help", version),
            *(self).cols,
        )
        .unwrap_or(format!("QQML Version {}, press ? for help", version))
    }
}

pub fn pad_to_width(input: &str, width: usize) -> Result<String, WidthTooSmallError> {
    let mut output = String::new();
    if input.len() > width {
        return Err(WidthTooSmallError);
    }
    (0..(width - input.len()) / 2).for_each(|_| output += " ");
    output += input;
    Ok(output)
}

fn wrap_text_to_width(input: &str, _width: usize) -> Result<String, WidthTooSmallError> {
    Ok(input.to_owned())
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WidthTooSmallError;
