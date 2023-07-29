use qqml_parser::Question;

use crate::utils::get_term_size;
use crate::Error;

pub trait Render<E> {
    fn render(&self) -> Result<String, E>;
}

pub struct Screen {
    pathline: PathLine,
    q_select_line: QuestionSelectLine,
    question_line: QuestionLine,
    hints_line: HintsLine,
    hints_body: Option<HintsBody>,
}
impl Render<Error> for Screen {
    fn render(&self) -> Result<String, Error> {
        let mut output = String::new();
        output += &self.pathline.render()?;
        output += "\n\n"; // The lines dont have their own \n
        output += &self.q_select_line.render()?;
        output += "\n\n";

        Ok(output)
    }
}

struct QuestionLine {
    q: Question,

}
impl Render<Error> for QuestionLine {
    fn render(&self) -> Result<String, Error> {
        Ok(match &self.q {
            Question::String() => "String questions are not supported.".to_owned(),
            Question::Calculation() => "Calculation questions are not supported.".to_owned(),
            Question::Multichoice(m) => {
                m.text.clone().unwrap()
            }
        })
    }
}

struct HintsBody { }

struct PathLine {
    path: String,
}
impl Render<Error> for PathLine {
    fn render(&self) -> Result<String, Error> {
        Ok(pad_to_width(&self.path, get_term_size()?.width)?)
    }
}

struct QuestionSelectLine {
    max_questions: usize,
    current_question: usize,
}
impl Render<Error> for QuestionSelectLine {
    fn render(&self) -> Result<String, Error> {
        let selector = format!(
            "<-({}/{})->",
            &self.current_question.to_string(),
            &self.max_questions.to_string()
        );

        Ok(pad_to_width(selector, get_term_size()?.width)?)
    }
}

struct HintsLine {
    max_hints: usize,
    hints_left: usize,
}

fn pad_to_width<S: Into<String>>(str: S, width: usize) -> Result<String, Error> {
    let mut output = String::new();
    let str: String = str.into();

    if str.len() > width {
        return Err(Error::TerminalTooSmall);
    }
    if str.len() == width {
        return Ok(str)
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
