pub mod core;
pub mod error;
pub mod multichoice;

#[cfg(test)]
mod test_correct;

use crate::json::parser::{JsonConstructionError, JsonTreeNode, JsonType, JsonValue};
use multichoice::data::MultichoiceData;

#[derive(Debug, PartialEq, Eq, Clone)]
#[allow(unused)]
pub enum Question {
    Multichoice(MultichoiceData),
    Calculation(),
    String(),
}

impl Question {
    pub fn from_json(json: &JsonType) -> Result<Question, JsonConstructionError> {
        match json {
            JsonType::Table(t) => {
                if let Some(JsonValue {
                    ident: _,
                    value: JsonType::String(v),
                }) = t.get_ident("type")
                {
                    match v.as_str() {
                        "multichoice" => {
                            return Ok(Self::Multichoice(MultichoiceData::from_json(t)?))
                        }
                        _ => (),
                    };
                }
            }
            _ => return Err(JsonConstructionError::SemanticError),
        }

        Err(JsonConstructionError::SemanticError)
    }

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

            output += &format!("\"col\": {}", 0);
        }
        output += "}";
        output
    }
}
