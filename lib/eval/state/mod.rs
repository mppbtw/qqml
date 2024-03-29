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

#[cfg(test)]
mod test;

use crate::eval::render::*;
use crate::json::lexer::*;
use crate::json::parser::JsonConstructionError;
use crate::json::parser::*;
use crate::parser::Question;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct State {
    pub hints_used:             usize,
    pub max_hints:              usize,
    pub questions:              Vec<Question>,
    pub current_question_index: usize,
    pub path_to_source:         Option<String>,

    // This stuff is used internally, because when creating screens they must
    // only hold references to values, these fields shouldnt matter during
    // construction.
    current_achieved_marks:    usize,
    questions_len:             usize,
    current_hints_available:   usize,
    cols:                      usize,
    has_watched_final_cutsene: bool,
}

#[allow(clippy::field_reassign_with_default)]
impl State {
    pub fn create_screen(&mut self, screen_width: usize) -> Screen {
        self.cols = screen_width;
        self.questions_len = self.questions.len();
        self.current_hints_available = match &self.questions[self.current_question_index] {
            Question::Multichoice(d) => d.hints.len() - d.used_hints,
            _ => unimplemented!(),
        };
        let mut s = Screen::default();

        // Some components which are universally wanted
        s.version_line = Some(VersionLine { cols: &self.cols });
        s.q_select_line = Some(QuestionSelectLine {
            cols:             &self.cols,
            max_questions:    &self.questions_len,
            current_question: &self.current_question_index,
        });

        // Other components are added depending on the state
        if let Some(p) = &self.path_to_source {
            s.pathline = Some(PathLine {
                path: p,
                cols: &self.cols,
            })
        }

        if let Question::Multichoice(d) = &self.questions[self.current_question_index] {
            if d.used_hints != 0 {
                s.hints_body = Some(HintsBody {
                    hints: &d.hints[0..d.used_hints],
                })
            }

            if d.is_answered {
                s.hints_line = Some(HintsLine {
                    max_hints:        &self.max_hints,
                    hints_available:  &self.current_hints_available,
                    hints_used_total: &self.hints_used,
                    is_answered:      &true,
                });

                s.question_result_body = Some(QuestionResultBody {
                    cols:    &self.cols,
                    answers: &d.answers,
                });

                self.current_achieved_marks = 0;
                for a in d.answers.iter() {
                    if a.is_chosen {
                        self.current_achieved_marks += a.marks;
                    }
                }

                s.question_result_line = Some(QuestionResultLine {
                    max_marks:      &d.max_marks,
                    question:       &d.text,
                    achieved_marks: &self.current_achieved_marks,
                });
            } else {
                s.hints_line = Some(HintsLine {
                    max_hints:        &self.max_hints,
                    hints_available:  &self.current_hints_available,
                    hints_used_total: &self.hints_used,
                    is_answered:      &false,
                });

                s.question_line = Some(QuestionLine { q: d });
                s.question_body = Some(QuestionBody {
                    answers:  (d
                        .answers
                        .iter()
                        .map(|d| ((d.text.as_ref().unwrap()), &d.is_chosen))
                        .collect()),
                    selected: &d.selected_answer,
                })
            }
        };

        s
    }

    pub fn every_question_answered(&self) -> bool {
        for q in self.questions.iter() {
            if let Question::Multichoice(d) = q {
                if !d.is_answered {
                    return false;
                }
            }
        }
        true
    }

    pub fn to_json(&self) -> String {
        let mut output = String::new();
        output += "{";
        output += &format!("\"max_hints\": {},", self.max_hints);
        output += &format!("\"hints_used\": {},", self.hints_used);
        output += &format!(
            "\"path_to_source\": \"{}\",",
            self.path_to_source.clone().unwrap_or("".to_owned())
        );
        output += &format!(
            "\"has_watched_final_cutsene\": {},",
            self.has_watched_final_cutsene
        );
        output += &format!(
            "\"current_question_index\": {},",
            self.current_question_index
        );
        output += &format!(
            "\"questions\": [{}]",
            self.questions
                .iter()
                .map(|q| q.to_json())
                .collect::<Vec<String>>()
                .join(",")
        );
        output += "}\n";
        output
    }

    pub fn get_max_marks(&self) -> usize {
        self.questions
            .iter()
            .map(|q| match q {
                Question::Multichoice(d) => d.max_marks,
                _ => 0,
            })
            .sum()
    }

    pub fn achieved_marks(&self) -> usize {
        self.questions
            .iter()
            .map(|q| match q {
                Question::Multichoice(d) => {
                    if d.is_answered {
                        d.answers
                            .iter()
                            .map(|a| if a.is_chosen { a.marks } else { 0 })
                            .sum()
                    } else {
                        0
                    }
                }
                _ => 0,
            })
            .sum()
    }

    pub fn watch_final_cutsene(&mut self) {
        self.has_watched_final_cutsene = true;
    }

    pub fn has_watched_final_cutsene(&self) -> bool {
        self.has_watched_final_cutsene
    }

    pub fn from_json(input: String) -> Result<Self, JsonConstructionError> {
        let mut lexer = Lexer::new(input);
        let json = parse(&mut lexer)?;

        let questions = if let Some(q) = json.get_ident("questions") {
            match &q.value {
                JsonType::Array(a) => {
                    let mut questions: Vec<Question> = vec![];
                    for v in a.values.iter() {
                        questions.push(Question::from_json(v)?);
                    }
                    questions
                }
                _ => return Err(JsonConstructionError::SemanticError),
            }
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let max_hints = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(n),
        }) = json.get_ident("max_hints")
        {
            n
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let current_question_index = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Number(n),
        }) = json.get_ident("current_question_index")
        {
            n
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let has_watched_final_cutsene = *if let Some(JsonValue {
            ident: _,
            value: JsonType::Bool(b),
        }) = json.get_ident("has_watched_final_cutsene")
        {
            b
        } else {
            return Err(JsonConstructionError::SemanticError);
        };

        let path_to_source = Some(
            if let Some(JsonValue {
                ident: _,
                value: JsonType::String(s),
            }) = json.get_ident("path_to_source")
            {
                s.to_owned()
            } else {
                return Err(JsonConstructionError::SemanticError);
            },
        );

        Ok(State {
            questions,
            max_hints,
            has_watched_final_cutsene,
            path_to_source,
            current_question_index,
            ..Default::default()
        })
    }
}
