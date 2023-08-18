use super::lexer::Lexer;
use super::lexer::Token;
use super::parser::parse;
use super::lexer::TokenData;
use super::parser::JsonTreeNode;
use super::parser::JsonType;

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
    let input = "
        {\"table\": {\"nested_ident\": \"nested_value\"}}
        ";
    let result = parse(&mut Lexer::new(input)).unwrap();
    let expected = JsonTreeNode {
        node_type: JsonType::Table,
        ident: None,
        children: vec![
            JsonTreeNode {
                node_type: JsonType::Table,
                ident: Some("table".to_owned()),
                children: vec![
                    JsonTreeNode {
                        node_type: JsonType::String("nested_value".to_owned()),
                        ident: Some("nested_ident".to_owned()),
                        children: vec![]
                    }
                ]
            }
        ]
    };
    assert_eq!(result, expected)
}
