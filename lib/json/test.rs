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

use super::lexer::Lexer;
use super::lexer::Token;
use super::lexer::TokenData;
use super::parser::parse;
use super::parser::JsonTreeNode;
use super::parser::JsonType;
use super::parser::JsonValue;
use crate::json::parser::JsonArray;

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

#[test]
fn test_parse_array_of_objects() {
    let input = "{\"ident\": [{\"ident\": \"value\"}]}";
    let expected = JsonTreeNode {
        values: vec![JsonValue {
            ident: "ident".to_owned(),
            value: JsonType::Array(JsonArray {
                values: vec![JsonType::Table(JsonTreeNode {
                    values: vec![JsonValue {
                        ident: "ident".to_owned(),
                        value: JsonType::String("value".to_owned()),
                    }],
                })],
            }),
        }],
    };
    let result = parse(&mut Lexer::new(input)).unwrap();
    assert_eq!(result, expected);
}
