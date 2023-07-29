use crate::lexer::Lexer;
use crate::token::Token;
use crate::token::TokenData;

/// Much quicker way to get the TokenData
/// which saves me writing that out manually
/// for Token::*.
fn d() -> TokenData {
    TokenData::default()
}

#[test]
fn test_tokenise_numbers() {
    let input = "123;   2;";
    let expected = vec![
        Token::Number(d(), 123),
        Token::Semicolon(d()),
        Token::Number(d(), 2),
        Token::Semicolon(d()),
        Token::Eof(d()),
    ];
    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        dbg!(&expected_token);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof(_)) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_tokenise_string_literals() {
    let input = ";'literal';\"literal\";'l'\"l\"";
    let expected = vec![
        Token::Semicolon(d()),
        Token::Literal(d(), "literal".to_owned()),
        Token::Semicolon(d()),
        Token::Literal(d(), "literal".to_owned()),
        Token::Semicolon(d()),
        Token::Literal(d(), "l".to_owned()),
        Token::Literal(d(), "l".to_owned()),
        Token::Eof(d()),
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        dbg!(&expected_token);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof(_)) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_multi_char_tokens() {
    let input = "!=>< >= <= ->";
    let expected = vec![
        Token::NEqual(d()),
        Token::GThan(d()),
        Token::LThan(d()),
        Token::GThanOrEqual(d()),
        Token::LThanOrEqual(d()),
        Token::RArrow(d()),
        Token::Eof(d()),
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof(_)) {
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
        Token::Ask(d()),
        Token::Multichoice(d()),
        Token::Calculation(d()),
        Token::String(d()),
        Token::Inputs(d()),
        Token::Eof(d()),
    ];

    let mut i = 0;
    let mut lexer = Lexer::new(input);

    loop {
        let expected_token = &expected[i];
        let tok = lexer.next_token();
        dbg!(&tok);
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof(_)) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_keyword_tokens() {
    let input = "ask multichoice calculation string inputs hints";
    let expected = vec![
        Token::Ask(d()),
        Token::Multichoice(d()),
        Token::Calculation(d()),
        Token::String(d()),
        Token::Inputs(d()),
        Token::Hints(d()),
        Token::Eof(d()),
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
fn test_single_char_tokens() {
    let input = "/;=:{}[](),><*%$#";

    let expected = vec![
        Token::Divide(d()),
        Token::Semicolon(d()),
        Token::Equal(d()),
        Token::Colon(d()),
        Token::LSquirly(d()),
        Token::RSquirly(d()),
        Token::LSquare(d()),
        Token::RSquare(d()),
        Token::LParen(d()),
        Token::RParen(d()),
        Token::Comma(d()),
        Token::GThan(d()),
        Token::LThan(d()),
        Token::Asterisk(d()),
        Token::Illegal(d()),
        Token::Illegal(d()),
        Token::Illegal(d()),
        Token::Eof(d()),
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
