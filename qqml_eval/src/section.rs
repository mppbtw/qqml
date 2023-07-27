use qqml_parser::ParsedSection;
use qqml_parser::Question;

#[derive(PartialEq, Debug, Clone)]
pub struct Section {
    pub path_to_source: Option<String>,
    pub current_question: usize,

    pub max_hints: usize,
    pub hints_used: usize,

    pub questions: Vec<Question>,
}

impl Section {
    pub fn new(parsed: ParsedSection) -> Section {
        Section {
            path_to_source: None,
            current_question: 0,
            max_hints: parsed.max_hints,
            hints_used: 0,
            questions: parsed.questions,
        }
    }
}
