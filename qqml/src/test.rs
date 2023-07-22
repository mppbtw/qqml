use super::lexer::Lexer;
use super::token::Token;

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
        if matches!(tok, Token::Eof) {
            break;
        }
        assert_eq!(tok, *expected_token);

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
        if matches!(tok, Token::Eof) {
            break;
        }
        assert_eq!(tok, *expected_token);

        i += 1;
    }
}

#[test]
fn test_single_char_tokens() {
    let input = ";=:{}[](),><*_%$#";

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
        Token::Asterisk,
        Token::Illegal,
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
        if matches!(tok, Token::Eof) {
            break;
        }
        assert_eq!(tok, *expected_token);

        i += 1;
    }
}
