use super::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonSyntaxError(pub Token);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonType {
    Number(i128),
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
        for val in self.values.iter() {
            if val.ident == ident {
                return Some(&val);
            }
        }
        None
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
        dbg!(&tok);
        std::fs::write("./log.txt", format!("{:?}", tok)).unwrap();
        // Should only loop once per value
        match tok {
            Token::Eof(_) => return Err(JsonSyntaxError(tok)),
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

            Token::Eof(_) => return Err(JsonSyntaxError(tok)),
            _ => (),
        };
        dbg!("oooooo",&tok);
        tok = l.next_token();
        dbg!("er",&tok);
        match tok {
            Token::Comma(_) => (),
            Token::RSquare(_) => break,
            _ => return Err(JsonSyntaxError(tok)),
        }
    }

    Ok(JsonArray { values })
}
