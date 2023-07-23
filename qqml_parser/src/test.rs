use crate::parser::*;
use crate::multichoice::*;
use qqml_lexer::*;

#[test]
fn test_parse_multichoice_answer_with_no_explanation() {
    let input = "'guh' (3);";
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
