use qqml_parser::Question;

#[derive(PartialEq, Debug, Clone)]
pub struct Quiz {
    current_question: usize,

    max_hints: usize,
    hints_used: usize,

    questions: Vec<Question>,
}
