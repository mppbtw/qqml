use qqml_parser::ParsedFile;
use qqml_parser::Question;

#[derive(PartialEq, Debug, Clone)]
pub struct Target {
    pub path_to_source: Option<String>,
    pub current_question: usize,

    pub max_hints: usize,
    pub hints_used: usize,

    pub questions: Vec<Question>,
}

impl Target {
    pub fn new(parsed: ParsedFile, path_to_source: Option<String>) -> Self {
        Self {
            path_to_source,
            current_question: 0,
            max_hints: parsed.max_hints,
            hints_used: 0,
            questions: parsed.questions,
        }
    }
}
