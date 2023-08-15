#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    pub text: String,
    pub max_marks: usize,
    pub answers: Vec<MultichoiceAnswer>,
    pub hints: Vec<String>,
    pub used_hints: usize,
    pub selected_answer: usize,
    pub line: usize,
    pub is_answered: bool,
}

impl MultichoiceData {
    pub fn get_total_marks(&self) -> usize {
        let mut total: usize = 0;
        for i in self.answers.iter().cloned() {
            total += i.marks;
        }
        total
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    pub text: Option<String>,
    pub marks: usize,
    pub explanation: Option<String>,
    pub is_chosen: bool,
}
impl MultichoiceAnswer {
    pub fn to_json(&self) -> String {
        let mut output = String::new();
        output += "{";
        output += &format!("\"text\": \"{}\",", self.text.clone().unwrap());
        output += &format!("\"marks\": {},", self.marks);
        output += &format!("\"explanation\": \"{}\",", self.explanation.clone().unwrap_or("".to_owned()));
        output += &format!("\"is_chosen\": {}", self.is_chosen);
        output += "}";
        output
    }
}

impl Default for MultichoiceAnswer {
    fn default() -> Self {
        Self {
            text: Some("".to_owned()),
            marks: 0,
            explanation: None,
            is_chosen: false,
        }
    }
}
