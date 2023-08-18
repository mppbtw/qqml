use super::lexer::*;

#[derive(Debug, Clone, PartialEq)]
pub struct JsonSyntaxError(pub Token);

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum JsonType {
    Number(i128),
    String(String),
    Bool(bool),

    #[default]
    Table,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct JsonTreeNode {
    pub children: Vec<JsonTreeNode>,
    pub node_type: JsonType,
    pub ident: Option<String>,
}

pub fn parse(l: &mut Lexer) -> Result<JsonTreeNode, JsonSyntaxError> {
    let mut node = JsonTreeNode::default();
    let mut tok = l.next_token();
    dbg!(&tok);

    loop {
        match tok {
            Token::Eof(_) => return Err(JsonSyntaxError(tok)),
            Token::String(_, s) => {

            },
            Token::RSquirly(_) => break,
            Token::LSquirly(_) => {
                node.children.push(parse(l)?);
                node.node_type = JsonType::Table;
                tok = l.next_token();
                if !matches!(tok, Token::Comma(_)) {
                    return Ok(node)
                }
            }
            _ => ()
        }
        tok = l.next_token();
    }

    Ok(node)
}
