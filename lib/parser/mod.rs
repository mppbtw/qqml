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

pub mod core;
pub mod error;
pub mod multichoice;

#[cfg(test)]
mod test_correct;

use multichoice::data::MultichoiceData;

use crate::json::parser::JsonConstructionError;
use crate::json::parser::JsonType;
use crate::json::parser::JsonValue;

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
                    if v.as_str() == "multichoice" {
                        return Ok(Self::Multichoice(MultichoiceData::from_json(t)?));
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
