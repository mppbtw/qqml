use crate::hashmap;
use crate::qqml::token::Token;
use crate::qqml::lexer::Lexer;

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
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        if matches!(tok, Token::Eof) {
            break;
        }
        assert_eq!(tok, *expected_token);

        i += 1;
    }
}
