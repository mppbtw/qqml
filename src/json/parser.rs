use super::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonSyntaxError(pub Token);

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum JsonType {
    Number(i128),
    String(String),
    Bool(bool),
    Table(JsonTreeNode),
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JsonTreeNode {
    pub values: Vec<JsonValue>,
    pub ident: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsonValue {
    pub ident: String,
    pub value: JsonType,
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
                    Token::LSquirly(_) => node.values.push(JsonValue {
                        ident,
                        value: JsonType::Table(parse(l)?),
                    }),
                    _ => (),
                }
            }
            // Yes, this will allow extra commas it's not a bug it's a feature
            Token::Comma(_) => continue,
            Token::RSquirly(_) => break,
            _ => return Err(JsonSyntaxError(tok)),
        }
        tok = l.next_token();
    }

    Ok(node)
}
