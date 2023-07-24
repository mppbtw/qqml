use super::parser::*;
use super::MultichoiceAnswer;

use qqml_lexer::*;

#[test]
fn test_parse_multichoice_answer_with_no_explanation_double_quotes() {
    let input = "\"guh\" (3)  ;;;";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("guh");
    expected.set_marks(3);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_with_no_explanation_single_quotes() {
    let input = "'guh' (3)  ;;;";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("guh");
    expected.set_marks(3);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_with_explanation_single_quotes() {
    let input = "'guh'  ( 2)   ->  'explanation'; ";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("guh");
    expected.set_marks(2);
    expected.set_explanation("explanation");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_with_explanation_double_quotes() {
    let input = "\"guh\"  ( 2)   ->  \"explanation\"; ";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("guh");
    expected.set_marks(2);
    expected.set_explanation("explanation");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}
