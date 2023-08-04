use crate::multichoice::parse_multichoice;
use crate::MultichoiceAnswer;
use crate::MultichoiceData;
use qqml_lexer::Lexer;
use qqml_lexer::Token;
use qqml_lexer::TokenData;

#[test]
fn test_parse_multichoice() {
    let input = "
        (1) 'title' {
            * 'correct' (1) -> 'explanation';
            * 'incorrect' -> 'explanation';
            * 'inc';
        };
    ";
    let mut l = Lexer::new(input);
    let result = parse_multichoice(&mut l, Token::Ask(TokenData { col: 0, line: 1 })).unwrap();

    let mut expected = MultichoiceData {
        max_marks: Some(1),
        hints: vec![],
        answers: vec![],
        line: 1,
        chosen_answer: None,
        text: Some("title".to_owned()),
        warnings: vec![],
    };

    expected.answers.push(MultichoiceAnswer {
        text: Some("correct".to_owned()),
        marks: 1,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        text: Some("incorrect".to_owned()),
        marks: 0,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        text: Some("inc".to_owned()),
        marks: 0,
        explanation: None,
    });

    assert_eq!(expected, result);
}

#[test]
fn test_parse_multichoice_double_quotes() {
    let input = "
        (1) \"title\" {
            * \"correct\" (1) -> \"explanation\";
            * \"incorrect\" -> \"explanation\";
            * \"inc\";
        };
    ";
    let mut l = Lexer::new(input);
    let result = parse_multichoice(&mut l, Token::Ask(TokenData { col: 0, line: 1 })).unwrap();

    let mut expected = MultichoiceData {
        max_marks: Some(1),
        hints: vec![],
        answers: vec![],
        line: 1,
        chosen_answer: None,
        text: Some("title".to_owned()),
        warnings: vec![],
    };

    expected.answers.push(MultichoiceAnswer {
        text: Some("correct".to_owned()),
        marks: 1,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        text: Some("incorrect".to_owned()),
        marks: 0,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        text: Some("inc".to_owned()),
        marks: 0,
        explanation: None,
    });

    assert_eq!(expected, result);
}
