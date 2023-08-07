use qqml_parser::Question;

const INFO_SECTION_WIDTH: usize = 50;

pub trait Render {
    fn render(&self) -> String;
}

#[derive(Debug, Clone)]
pub struct Screen<'a> {
    pub pathline: Option<PathLine<'a>>,
    pub version_line: VersionLine,
    pub q_select_line: QuestionSelectLine<'a>,
    pub question_line: QuestionLine<'a>,
    pub question_body: QuestionBody<'a>,
    pub hints_line: HintsLine<'a>,
    pub hints_body: Option<HintsBody<'a>>,
}
impl Render for Screen<'_> {
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

#[derive(Debug, Clone)]
pub struct QuestionBody<'a> {
    pub answers: Vec<String>,
    pub selected: &'a usize,
}
impl Render for QuestionBody<'_> {
    fn render(&self) -> String {
        let mut output = String::new();
        for (i, a) in self.answers.iter().enumerate() {
            if &i == self.selected {
                output += &format!("   {} <", a);
            } else {
                output += &("   ".to_owned() + a);
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
}
impl Render for PathLine<'_> {
    fn render(&self) -> String {
        pad_to_width(&self.path, INFO_SECTION_WIDTH).unwrap_or(self.path.clone())
    }
}

#[derive(Debug, Clone)]
pub struct QuestionSelectLine<'a> {
    pub max_questions: &'a usize,
    pub current_question: &'a usize,
}
impl Render for QuestionSelectLine<'_> {
    fn render(&self) -> String {
        pad_to_width(
            &format!(
                "<--({} / {})-->",
                &self.current_question.to_string(),
                &self.max_questions.to_string()
            ),
            INFO_SECTION_WIDTH,
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
pub struct VersionLine;
impl Render for VersionLine {
    fn render(&self) -> String {
        let version = env!("CARGO_PKG_VERSION");
        pad_to_width(
            &format!("QQML Version {}, press ? for help", version),
            INFO_SECTION_WIDTH,
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

#[derive(Clone, Copy, Debug, PartialEq)]
struct WidthTooSmallError;
