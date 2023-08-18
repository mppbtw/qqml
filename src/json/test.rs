use crate::json::parser::JsonArray;

use super::lexer::Lexer;
use super::lexer::Token;
use super::lexer::TokenData;
use super::parser::parse;
use super::parser::JsonTreeNode;
use super::parser::JsonType;
use super::parser::JsonValue;
use std::{sync::mpsc, thread, time::Duration};

#[test]
fn test_tokenise() {
    let input = "
        {,: 	    % !  }
        true false s \"lol\"
        "
    .to_owned();

    let expected = vec![
        Token::LSquirly(TokenData::default()),
        Token::Comma(TokenData::default()),
        Token::Colon(TokenData::default()),
        Token::Illegal(TokenData::default()),
        Token::Illegal(TokenData::default()),
        Token::RSquirly(TokenData::default()),
        Token::True(TokenData::default()),
        Token::False(TokenData::default()),
        Token::Illegal(TokenData::default()),
        Token::String(TokenData::default(), "lol".to_owned()),
        Token::Eof(TokenData::default()),
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof(_)) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_parse() {
    let input = "{\"table\": {\"ident\": \"value\", \"table\": {\"ident\":\"value\"}}}";
    let expected = JsonTreeNode {
        values: vec![JsonValue {
            ident: "table".to_owned(),
            value: JsonType::Table(JsonTreeNode {
                values: vec![
                    JsonValue {
                        ident: "ident".to_owned(),
                        value: JsonType::String("value".to_owned()),
                    },
                    JsonValue {
                        ident: "table".to_owned(),
                        value: JsonType::Table(JsonTreeNode {
                            values: vec![JsonValue {
                                ident: "ident".to_owned(),
                                value: JsonType::String("value".to_owned()),
                            }],
                        }),
                    },
                ],
            }),
        }],
    };
    let result = parse(&mut Lexer::new(input)).unwrap();
    assert_eq!(result, expected);
}

#[test]
fn test_parse_arrays() {
    let input = "{\"ident\": [\"val1\", \"val2\", 3]}";
    let expected = JsonTreeNode {
        values: vec![JsonValue {
            ident: "ident".to_owned(),
            value: JsonType::Array(JsonArray {
                values: vec![
                    JsonType::String("val1".to_owned()),
                    JsonType::String("val2".to_owned()),
                    JsonType::Number(3),
                ],
            }),
        }],
    };
    let result = parse(&mut Lexer::new(input)).unwrap();
    assert_eq!(result, expected);
}
