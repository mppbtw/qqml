use super::parser::*;
use super::MultichoiceAnswer;

use qqml_lexer::*;

#[test]
fn test_parse_multichoice_answer_marks_no_explanation() {
    let input = "'foo' (3)  ;;;";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(3);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_no_marks_no_explanation() {
    let input = "'foo' ;;;";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(0);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_marks_explanation() {
    let input = "'foo' (3) -> 'bar';;;";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(3);
    expected.set_explanation("bar");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_no_marks_explanation() {
    let input = "'foo' -> 'bar';;;";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(0);
    expected.set_explanation("bar");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}
