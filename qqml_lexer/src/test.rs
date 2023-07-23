use crate::lexer::Lexer;
use crate::token::Token;

#[test]
fn test_tokenise_numbers() {
    let input = "123;   2;";
    let expected = vec![
        Token::Number(123),
        Token::Semicolon,
        Token::Number(2),
        Token::Semicolon,
        Token::Eof,
    ];
    let mut i = 0;
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        dbg!(&expected_token);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_tokenise_string_literals() {
    let input = ";'literal';\"literal\";'l'\"l\"";
    let expected = vec![
        Token::Semicolon,
        Token::Literal("literal".to_owned()),
        Token::Semicolon,
        Token::Literal("literal".to_owned()),
        Token::Semicolon,
        Token::Literal("l".to_owned()),
        Token::Literal("l".to_owned()),
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        dbg!(&expected_token);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_multi_char_tokens() {
    let input = "!=>< >= <= ->";
    let expected = vec![
        Token::NEqual,
        Token::GThan,
        Token::LThan,
        Token::GThanOrEqual,
        Token::LThanOrEqual,
        Token::RArrow,
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_keyword_tokens_with_spaces() {
    let input = "ask
    multichoice    		calculation  string


                inputs";

    let expected = vec![
        Token::Ask,
        Token::Multichoice,
        Token::Calculation,
        Token::String,
        Token::Inputs,
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_keyword_tokens() {
    let input = "ask multichoice calculation string inputs";
    let expected = vec![
        Token::Ask,
        Token::Multichoice,
        Token::Calculation,
        Token::String,
        Token::Inputs,
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_single_char_tokens() {
    let input = "/;=:{}[](),><*%$#";

    let expected = vec![
        Token::Divide,
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
        Token::Asterisk,
        Token::Illegal,
        Token::Illegal,
        Token::Illegal,
        Token::Eof,
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input).unwrap();

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof) {
            break;
        }
        i += 1;
    }
}
