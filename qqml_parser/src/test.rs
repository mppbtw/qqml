use crate::parser::*;
use crate::multichoice::*;
use qqml_lexer::*;

#[test]
fn test_parse_multichoice_answer_with_no_explanation_double_quotes() {
    let input = "\"guh\" (3)  ;;;";
    let mut l = Lexer::new(input).unwrap();

    let expected = MultichoiceAnswer {
        explanation: None,
        text: "guh".to_owned(),
        marks: 3,
    };

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected.marks, gotten.marks);
    assert_eq!(expected.text, gotten.text);
    assert_eq!(expected.explanation, gotten.explanation);
}

#[test]
fn test_parse_multichoice_answer_with_no_explanation_single_quotes() {
    let input = "'guh' (3)  ;;;";
    let mut l = Lexer::new(input).unwrap();

    let expected = MultichoiceAnswer {
        explanation: None,
        text: "guh".to_owned(),
        marks: 3,
    };

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected.marks, gotten.marks);
    assert_eq!(expected.text, gotten.text);
    assert_eq!(expected.explanation, gotten.explanation);
}

#[test]
fn test_parse_multichoice_answer_with_explanation_single_quotes() {
    let input = "'guh'  ( 2)   ->  'explanation'; ";
    let mut l = Lexer::new(input).unwrap();

    let expected = MultichoiceAnswer {
        explanation: Some("explanation".to_owned()),
        text: "guh".to_owned(),
        marks: 2
    };

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected.marks, gotten.marks);
    assert_eq!(expected.text, gotten.text);
    assert_eq!(expected.explanation, gotten.explanation);
}

#[test]
fn test_parse_multichoice_answer_with_explanation_double_quotes() {
    let input = "\"guh\"  ( 2)   ->  \"explanation\"; ";
    let mut l = Lexer::new(input).unwrap();

    let expected = MultichoiceAnswer {
        explanation: Some("explanation".to_owned()),
        text: "guh".to_owned(),
        marks: 2
    };

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected.marks, gotten.marks);
    assert_eq!(expected.text, gotten.text);
    assert_eq!(expected.explanation, gotten.explanation);
}
