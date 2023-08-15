mod error;
mod multichoice;
mod parser;

#[cfg(test)]
mod test_correct;

pub use self::error::Error;
pub use self::error::ErrorReport;
pub use self::multichoice::MultichoiceAnswer;
pub use self::multichoice::MultichoiceData;
pub use self::parser::parse;
pub use self::parser::ParsedFile;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}

impl Question {
    pub fn to_json(&self) -> String {
        let mut output = String::new();
        output += "{";
        if let Self::Multichoice(d) = self {
            output += &format!("\"type\": {},", "\"multichoice\"");
            output += &format!("\"max_marks\": {},", d.max_marks);
            output += &format!("\"line\": {},", d.line);
            output += &format!("\"selected_answer\": {},", d.selected_answer);
            output += &format!("\"is_answered\": {},", d.is_answered);
            output += &format!("\"text\": \"{}\",", d.text);
            output += &format!(
                "\"hints\": [{}],",
                d.hints
                    .iter()
                    .map(|h| format!("\"{}\"", h))
                    .collect::<Vec<String>>()
                    .join(",")
            );
            output += &format!("\"used_hints\": {},", d.used_hints);
            output += &format!(
                "\"answers\": [{}],",
                d.answers
                    .iter()
                    .map(|a| a.to_json())
                    .collect::<Vec<String>>()
                    .join(",")
            );

            output += &format!("\"col\": {}", 0); // TODO: implement this
        }
        output += "}";
        output
    }
}
