//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::json::parser::JsonConstructionError;
use crate::json::parser::JsonTreeNode;
use crate::json::parser::JsonType;
use crate::json::parser::JsonValue;

#[derive(Debug, Default, PartialEq, Eq, Clone)]
pub struct MultichoiceData {
    pub text:            String,
    pub max_marks:       usize,
    pub answers:         Vec<MultichoiceAnswer>,
    pub hints:           Vec<String>,
    pub used_hints:      usize,
    pub selected_answer: usize,
    pub line:            usize,
    pub is_answered:     bool,
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

        let answers: Vec<MultichoiceAnswer> = if let Some(JsonValue {
            ident: _,
            value: JsonType::Array(a),
        }) = json.get_ident("answers")
        {
            let mut answers: Vec<MultichoiceAnswer> = vec![];
            for r in a.values.iter().map(|v| {
                if let JsonType::Table(t) = v {
                    Ok(t)
                } else {
                    Err(JsonConstructionError::SemanticError)
                }
            }) {
                answers.push(match r {
                    Ok(t) => MultichoiceAnswer::from_json(t),
                    Err(_) => return Err(JsonConstructionError::SemanticError),
                }?)
            }
            answers
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let hints: Vec<String> = if let Some(JsonValue {
            ident: _,
            value: JsonType::Array(a),
        }) = json.get_ident("hints")
        {
            let mut hints: Vec<String> = vec![];
            for r in a
                .values
                .iter()
                .map(|v| {
                    if let JsonType::String(s) = v {
                        Ok(s)
                    } else {
                        Err(JsonConstructionError::SemanticError)
                    }
                })
                .collect::<Vec<Result<&String, _>>>()
                .iter()
            {
                hints.push(r.to_owned()?.to_owned())
            }
            hints
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
            hints,
            answers,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct MultichoiceAnswer {
    /// This should never be None after parsing, dont ask why its an Option<_>
    /// in the first place I'm not sure to be honest.
    pub text:        Option<String>,
    pub marks:       usize,
    pub explanation: Option<String>,
    pub is_chosen:   bool,
}
impl MultichoiceAnswer {
    pub fn from_json(json: &JsonTreeNode) -> Result<MultichoiceAnswer, JsonConstructionError> {
        let text = Some(
            if let Some(JsonValue {
                ident: _,
                value: JsonType::String(s),
            }) = json.get_ident("text")
            {
                s.to_owned()
            } else {
                return Err(JsonConstructionError::SemanticError);
            },
        );

        let marks = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(s),
        }) = json.get_ident("marks")
        {
            s
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let explanation = if let Some(JsonValue {
            ident: _,
            value: JsonType::String(s),
        }) = json.get_ident("explanation")
        {
            if s.is_empty() {
                None
            } else {
                Some(s.to_owned())
            }
        } else {
            None
        };

        let is_chosen = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Bool(b),
        }) = json.get_ident("is_chosen")
        {
            b
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        Ok(MultichoiceAnswer {
            text,
            marks,
            explanation,
            is_chosen,
        })
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
            text:        Some("".to_owned()),
            marks:       0,
            explanation: None,
            is_chosen:   false,
        }
    }
}
