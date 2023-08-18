use crate::lexer::core::Lexer;
use crate::lexer::token::Token;
use crate::lexer::token::TokenData;
use crate::parser::multichoice::parser::parse_multichoice;

#[test]
fn test_replacement_tolerance() {
    let input = "(2) x {
        * ' one' (1);
        * 'tow' (1) -> 'what is the meaning of life';
        * 'freee';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        1
    );

    let input = "x2x 'debunk gay marriage' {
        * ' one' (2);
        * 'tow' (1) -> 'what is the meaning of life';
        * 'freee';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );

    let input = "(2) 'debunk gay marriage' {
        * ' one' (2);
        * 'tow' (1) -> 'what is the meaning of life';
        * 'freee';
    ;";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        1
    );

    let input = "(2) 'debunk gay marriage' {
        * ' one' (2)x
        * 'tow' (1) -> 'what is the meaning of life';
        * 'freee';
    ";

    dbg!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
    );
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );

    let input = "(2) 'debunk gay marriage' {
        * x (2);
        * 'tow' (2) x 'what is the meaning of life';
        * 'freee';
    ";
    dbg!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
    );
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        3
    );
}

#[test]
fn test_negative_tolerance() {
    let input = "(1 'one two three' {
        * (1);
        * 'kung fu';
        * 'six feet deep';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );

    let input = "(1) {
        * 'diggin dem pockets' (1);
        * 'kung fu'
        * 'six feet deep';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );

    let input = "(1) 'you live for the fonk?'
        * 'i die for the fonk' (1);
        * 'i live for the fonk'
        * 'six feet deep';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );
}

#[test]
fn test_possitive_tolerance() {
    let input = "x(1) 'where is the sun' {
        * 'over there' (1);
        * 'no';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        1
    );

    let input = "x(1) 'where is the sun' {
        x* 'over there' (1);
        * 'no';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );

    let input = "(1) 'where is the sun' x{
        * 'over there' (1);
        * 'no';
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        1
    );

    let input = "(1) i'where is the sun' {
        * 'over there' (1);
        * 'no'x;
    };";
    assert_eq!(
        parse_multichoice(&mut Lexer::new(input), Token::Ask(TokenData::default()))
            .unwrap_err()
            .errors
            .len(),
        2
    );
}