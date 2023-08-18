use crate::parser::*;
use crate::lexer::Lexer;

#[test]
fn test_parse_multichoice_answer_explanation_marks() {
    let input = "'text' (1) -> 'this is the explanation';";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        marks: 1,
        explanation: Some("this is the explanation".to_owned()),
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer_explanation_marks_double_quotes() {
    let input = "\"text\" (1) -> \"this is the explanation\";";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        marks: 1,
        explanation: Some("this is the explanation".to_owned()),
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer_marks() {
    let input = "'text' (1);";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        marks: 1,
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer_marks_double_quotes() {
    let input = "\"text\" (1);";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        marks: 1,
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer_explanation() {
    let input = "'text' -> 'this is the explanation';";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        explanation: Some("this is the explanation".to_owned()),
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer_explanation_double_quotes() {
    let input = "\"text\" -> \"this is the explanation\";";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        explanation: Some("this is the explanation".to_owned()),
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer() {
    let input = "'text';";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}

#[test]
fn test_parse_multichoice_answer_double_quotes() {
    let input = "\"text\";";
    let expected = MultichoiceAnswer {
        text: Some("text".to_owned()),
        ..Default::default()
    };
    let mut l = Lexer::new(input);
    let gotten = multichoice::parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}
