use crate::multichoice::parse_multichoice;
use qqml_lexer::Lexer;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

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
}
