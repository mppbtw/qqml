use qqml_lexer::Lexer;
use crate::multichoice::*;

#[test]
fn test_replacement_tolerance() {
    let input1 = "
        'question text' (1x -> 4;
        ";

    let input2 = "
        x (1) -> 4;
        ";

    let input3 = "
        'question text' x1) -> 'explanation text';
        ";

    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input1)).unwrap_err().errors.len(), 2);
    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input2)).unwrap_err().errors.len(), 2);
    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input3)).unwrap_err().errors.len(), 1);
}

#[test]
fn test_positive_tolerance() {
    let input1 = "
        x'question text' (1) -> 'some explanation';
        ";

    let input2 = "
        'question text' x(1) -> 'some explanation';
        ";

    let input3 = "
        x'question text' (1) -> 'some explanation'x;
        ";

    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input1)).unwrap_err().errors.len(), 1);
    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input2)).unwrap_err().errors.len(), 1);
    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input3)).unwrap_err().errors.len(), 2);
}

#[test]
fn test_negative_tolerance() {
    let input1 = "
        (1) -> 'some explanation';
        ";

    let input2 = "
        'question text' ()  'some explanation';
        ";

    let input3 = "
        'question text' (1) -> 'some explanation'
        ";

    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input1)).unwrap_err().errors.len(), 1);
    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input2)).unwrap_err().errors.len(), 2);
    assert_eq!(parse_multichoice_answer(&mut Lexer::new(input3)).unwrap_err().errors.len(), 1);
}
