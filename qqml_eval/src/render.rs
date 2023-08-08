use qqml_parser::MultichoiceAnswer;
use qqml_parser::MultichoiceData;
use qqml_parser::Question;

const ANSI_RESET: &'static str = "\x1b[0m";
const ANSI_BG_WHITE: &'static str = "\x1b[47m";
const ANSI_BLACK: &'static str = "\x1b[1;30m";
const ANSI_GREEN: &'static str = "\x1b[32m";
const ANSI_RED: &'static str = "\x1b[31m";

pub trait Render {
    fn render(&self) -> String;
}

#[derive(Debug, Clone)]
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
}
impl Render for Screen<'_> {
    fn render(&self) -> String {
        let mut output = String::new();
        match &self.version_line {
            Some(c) => output += &(c.render() + "\n"),
            None => (),
        }
        match &self.pathline {
            Some(c) => output += &(c.render() + "\n"),
            None => (),
        };
        match &self.q_select_line {
            Some(c) => output += &(c.render() + "\n\n"),
            None => ()
        }
        match &self.question_line {
            Some(c) => output += &(c.render() + "\n"),
            None => ()
        }
        match &self.question_body {
            Some(c) => output += &(c.render() + "\n\n"),
            None => () 
        };
        match &self.question_result_body {
            Some(c) => output += &(c.render() + "\n\n"),
            None => (),
        };
        match &self.hints_line {
            Some(c) => output += &(c.render() + "\n"),
            None => (),
        }
        match &self.hints_body {
            Some(c) => output += &c.render(),
            None => (),
        };

        output
    }
}

#[derive(Debug, Clone)]
pub struct QuestionResultBody<'a> {
    pub answers: &'a Vec<MultichoiceAnswer>,
    pub cols: &'a usize,
    pub question: &'a MultichoiceData,
}
impl Render for QuestionResultBody<'_> {
    fn render(&self) -> String {
        let mut body = String::new();
        let mut output = String::new();
        let mut qline = String::new();
        let mut answers = self.answers.clone();

        answers.sort_by_key(|a| a.marks);
        answers.reverse();

        let mut total_marks = 0;
        for a in answers {
            total_marks += a.marks;
            if a.marks != 0 {
                body += &format!("{}(+{}) '{}'\n", ANSI_GREEN, a.marks, a.text.unwrap());
            } else {
                body += &format!("{}(+{}) '{}'\n", ANSI_RED, a.marks, a.text.unwrap());
            }
            body += ANSI_RESET;
            if let Some(x) = a.explanation {
                body += &wrap_text_to_width(&x, self.cols / 2).unwrap();
            }
        }

        qline += &format!(
            "{} ({}/{})",
            self.question.text.clone().unwrap(),
            total_marks,
            self.question.max_marks.unwrap(),
        );

        output += &(qline + "\n");
        output += &body;
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
                output += ANSI_BLACK;
                output += ANSI_BG_WHITE;
                output += &("   ".to_owned() + &a.0);
                output += ANSI_RESET;
                if &i == self.selected {
                    output += " <";
                }
            } else {
                if &i == self.selected {
                    output += &format!("   {} <", a.0);
                } else {
                    output += &("   ".to_owned() + &a.0);
                }
            }
            output += "\n";
        }
        output
    }
}

#[derive(Debug, Clone)]
pub struct QuestionLine<'a> {
    pub q: &'a Question,
}
impl Render for QuestionLine<'_> {
    fn render(&self) -> String {
        match &self.q {
            Question::String() => "String questions are not supported.".to_owned(),
            Question::Calculation() => "Calculation questions are not supported.".to_owned(),
            Question::Multichoice(m) => {
                format!("{} ({})", m.text.clone().unwrap(), m.max_marks.unwrap())
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct HintsBody<'a> {
    pub hints: &'a [String],
}
impl Render for HintsBody<'_> {
    fn render(&self) -> String {
        let mut output = String::new();
        for i in self.hints {
            output += &("  ".to_owned() + i);
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
        pad_to_width(&self.path, *(self).cols).unwrap_or(self.path.clone())
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
        .unwrap()
    }
}

#[derive(Debug, Clone)]
pub struct HintsLine<'a> {
    pub max_hints: &'a usize,
    pub hints_used_total: &'a usize,
    pub hints_available: &'a usize,
}
impl Render for HintsLine<'_> {
    fn render(&self) -> String {
        format!(
            "Hints (used {}/{}, {} available for this question):",
            &self.hints_used_total, &self.max_hints, &self.hints_available
        )
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
        .unwrap()
    }
}

fn pad_to_width(input: &str, width: usize) -> Result<String, WidthTooSmallError> {
    let mut output = String::new();
    if output.len() > width {
        return Err(WidthTooSmallError);
    }
    (0..(width - input.len()) / 2).for_each(|_| output += " ");
    output += input;
    Ok(output)
}

fn wrap_text_to_width(input: &str, width: usize) -> Result<String, WidthTooSmallError> {
    return Ok(input.to_owned());
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct WidthTooSmallError;
