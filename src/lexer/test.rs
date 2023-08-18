use crate::lexer::lexer::Lexer;
use super::token::Token;
use super::token::TokenData;

/// Much quicker way to get the TokenData
/// which saves me writing that out manually
/// for Token::*.
fn d() -> TokenData {
    TokenData::default()
}

#[test]
fn test_token_column_numbers_for_long_tokens() {
    let input = "   ask
     multichoice
->";
    let mut l = Lexer::new(input);
    assert_eq!(l.next_token().unwrap().get_data().col, 3);
    assert_eq!(l.next_token().unwrap().get_data().col, 5);
    assert_eq!(l.next_token().unwrap().get_data().col, 0);
}

#[test]
fn test_token_column_numbers_for_single_char_tokens() {
    let input = "  2
   =
      (";
    let mut l = Lexer::new(input);
    assert_eq!(l.next_token().unwrap().get_data().col, 2);
    assert_eq!(l.next_token().unwrap().get_data().col, 3);
    assert_eq!(l.next_token().unwrap().get_data().col, 6);
}

#[test]
fn test_token_line_numbers() {
    let input = "1
        2
        3
        4
        5";

    let mut l = Lexer::new(input);
    let mut i = 0;
    loop {
        let tok = l.next_token().unwrap();
        if matches!(tok, Token::Eof(_)) {
            break;
        }
        let dat = tok.get_data();
        assert_eq!(dat.line, i);
        i += 1;
    }
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
        let tok = lexer.next_token().unwrap();
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
        let tok = lexer.next_token().unwrap();
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
        let tok = lexer.next_token().unwrap();
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
        let tok = lexer.next_token().unwrap();
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
        let tok = lexer.next_token().unwrap();
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
        let tok = lexer.next_token().unwrap();
        assert_eq!(tok, *expected_token);
        if matches!(tok, Token::Eof(_)) {
            break;
        }
        i += 1;
    }
}

#[test]
fn test_read_massive_numbers() {
    let input =
        "99999999999999999999999999999999999999999999999999999999999999999999999999999999999999999";
    let mut l = Lexer::new(input);
    assert!(l.next_token().is_err());
}
