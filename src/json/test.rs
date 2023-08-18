use super::lexer::Lexer;
use super::lexer::Token;
use super::lexer::TokenData;

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
