//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'MrPiggyPegasus'
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

use super::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonSyntaxError(pub Token);

#[derive(Debug, Clone, PartialEq)]
pub enum JsonConstructionError {
    JsonSyntaxError(JsonSyntaxError),
    SemanticError,
}
impl From<JsonSyntaxError> for JsonConstructionError {
    fn from(value: JsonSyntaxError) -> Self {
        Self::JsonSyntaxError(value)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonType {
    Number(usize),
    String(String),
    Bool(bool),
    Array(JsonArray),
    Table(JsonTreeNode),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JsonTreeNode {
    pub values: Vec<JsonValue>,
}
impl JsonTreeNode {
    pub fn get_ident<S: Into<String>>(&self, ident: S) -> Option<&JsonValue> {
        let ident: String = ident.into();
        self.values.iter().find(|&val| val.ident == ident)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValue {
    pub ident: String,
    pub value: JsonType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonArray {
    pub values: Vec<JsonType>,
}

pub fn parse(l: &mut Lexer) -> Result<JsonTreeNode, JsonSyntaxError> {
    let mut node = JsonTreeNode::default();
    let mut tok = l.next_token();

    if l.tok_count == 1 {
        // Skip over the initial squirly bracket
        tok = l.next_token();
    }

    loop {
        // Should only loop once per value
        match tok {
            Token::Eof(_) => return Err(JsonSyntaxError(tok)),
            Token::LSquirly(_) => (),
            Token::String(_, ident) => {
                tok = l.next_token();
                if !matches!(tok, Token::Colon(_)) {
                    return Err(JsonSyntaxError(tok));
                }
                tok = l.next_token();
                match tok {
                    Token::String(_, value) => node.values.push(JsonValue {
                        ident,
                        value: JsonType::String(value),
                    }),
                    Token::LSquare(_) => {
                        let arr = parse_array(l)?;
                        node.values.push(JsonValue {
                            ident,
                            value: JsonType::Array(arr),
                        });
                    }
                    Token::LSquirly(_) => node.values.push(JsonValue {
                        ident,
                        value: JsonType::Table(parse(l)?),
                    }),
                    Token::Number(_, value) => node.values.push(JsonValue {
                        ident,
                        value: JsonType::Number(value),
                    }),
                    Token::True(_) => node.values.push(JsonValue {
                        ident,
                        value: JsonType::Bool(true),
                    }),
                    Token::False(_) => node.values.push(JsonValue {
                        ident,
                        value: JsonType::Bool(false),
                    }),
                    _ => (),
                }
            }
            // Yes, this will allow extra commas it's not a bug it's a feature
            Token::Comma(_) => {
                tok = l.next_token();
                continue;
            }
            Token::RSquirly(_) => break,

            _ => return Err(JsonSyntaxError(tok)),
        }
        tok = l.next_token();
    }

    Ok(node)
}

pub fn parse_array(l: &mut Lexer) -> Result<JsonArray, JsonSyntaxError> {
    let mut tok;
    let mut values: Vec<JsonType> = vec![];
    loop {
        tok = l.next_token();
        match tok {
            Token::String(_, ref s) => values.push(JsonType::String(s.clone())),
            Token::True(_) => values.push(JsonType::Bool(true)),
            Token::False(_) => values.push(JsonType::Bool(false)),
            Token::Number(_, n) => values.push(JsonType::Number(n)),
            Token::RSquare(_) => break,
            Token::LSquirly(_) => values.push(JsonType::Table(parse(l)?)),
            Token::Eof(_) => return Err(JsonSyntaxError(tok)),
            _ => (),
        };
        tok = l.next_token();
        match tok {
            Token::Comma(_) => (),
            Token::RSquare(_) => break,
            _ => return Err(JsonSyntaxError(tok)),
        }
    }

    Ok(JsonArray { values })
}
