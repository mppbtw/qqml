use crate::json::parser::JsonConstructionError;
use crate::json::parser::JsonTreeNode;
use crate::json::parser::JsonType;
use crate::json::parser::JsonValue;

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
    pub fn from_json(json: &JsonTreeNode) -> Result<MultichoiceData, JsonConstructionError> {
        let text = if let Some(JsonValue {
            ident: _,
            value: JsonType::String(s),
        }) = json.get_ident("text")
        {
            s.to_owned()
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let max_marks = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(n),
        }) = json.get_ident("max_marks")
        {
            n
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let line = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(n),
        }) = json.get_ident("line")
        {
            n
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let selected_answer = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(n),
        }) = json.get_ident("selected_answer")
        {
            n
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let is_answered = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Bool(b),
        }) = json.get_ident("is_answered")
        {
            b
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let used_hints = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(n),
        }) = json.get_ident("used_hints")
        {
            n
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        Ok(MultichoiceData {
            text,
            used_hints,
            is_answered,
            line,
            max_marks,
            selected_answer,
            hints: vec![],
            answers: vec![],
        })
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
    pub fn from_json(json: &JsonTreeNode) -> Result<MultichoiceData, JsonConstructionError> {
        Err(JsonConstructionError::SemanticError)
    }

    pub fn to_json(&self) -> String {
        let mut output = String::new();
        output += "{";
        output += &format!("\"text\": \"{}\",", self.text.clone().unwrap());
        output += &format!("\"marks\": {},", self.marks);
        output += &format!(
            "\"explanation\": \"{}\",",
            self.explanation.clone().unwrap_or("".to_owned())
        );
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
