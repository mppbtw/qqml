use crate::qqml::Token;
use crate::hashmap;

#[test]
fn test_next_token_single_chars() {
    let input = ";=:{}[](),><#";

    let expected = vec![
        Token::Semicolon,
        Token::Equal,
        Token::Colon,
        Token::LSquirly,
        Token::RSquirly,
        Token::LSquare,
        Token::RSquare,
        Token::LParen,
        Token::RParen,
        Token::Comma,
        Token::GThan,
        Token::LThan,
        Token::Illegal,
        Token::Eof,
    ];

    let mut i = 0;
    loop {

        let expected_token = expected.get(i).unwrap();
        assert_eq!(expected_token, next_token());
        if expected_token == Token::Eof {
            break;
        }

        i += 1;
    }
}
