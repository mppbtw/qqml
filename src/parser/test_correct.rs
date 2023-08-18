use super::multichoice::data::MultichoiceAnswer;
use super::multichoice::data::MultichoiceData;
use super::core::parse;
use super::core::ParsedFile;
use super::Question;

#[test]
fn test_parse_hints_directive() {
    let input1 = "
        hints 1;
        "
    .to_string();
    assert_eq!(parse(input1).unwrap().max_hints, 1);

    let input2 = "
        hints 2;
        "
    .to_string();
    assert_eq!(parse(input2).unwrap().max_hints, 2);
}

#[test]
fn test_parse_multichoice_questions_with_hints() {
    let input = "
        hints 2;

        ask multichoice (2) 'question1' {
            * 'correct1' (1) -> 'explan1';
            * 'incorrect1' -> 'explan1';
            * 'wrong1';
            * 'right1' (1);
        };


        ask multichoice (1) 'question2' {
            * 'correct2' (1) -> 'explan2';
            * 'incorrect2' -> 'explan2';
            * 'wrong2';
            * 'right2' (1);
        } hints 'hint1', 'hint2';
        ";

    let result = parse(input).unwrap();
    let expected = ParsedFile {
        max_hints: 2,
        questions: vec![
            // question1
            Question::Multichoice(MultichoiceData {
                is_answered: false,
                selected_answer: 0,
                text: "question1".to_owned(),
                line: 3,
                used_hints: 0,
                max_marks: 2,
                hints: vec![],
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 1,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 0,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong1".to_owned()),
                        marks: 0,
                        explanation: None,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("right1".to_owned()),
                        marks: 1,
                        explanation: None,
                        is_chosen: false,
                    },
                ],
            }),
            // question2
            Question::Multichoice(MultichoiceData {
                text: "question2".to_owned(),
                line: 11,
                max_marks: 1,
                hints: vec!["hint1".to_owned(), "hint2".to_owned()],
                used_hints: 0,
                selected_answer: 0,
                is_answered: false,
                answers: vec![
                    MultichoiceAnswer {
                        is_chosen: false,
                        text: Some("correct2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect2".to_owned()),
                        is_chosen: false,
                        explanation: Some("explan2".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        is_chosen: false,
                        text: Some("wrong2".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        is_chosen: false,
                        text: Some("right2".to_owned()),
                        marks: 1,
                        explanation: None,
                    },
                ],
            }),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_multichoice_questions_with_hints_double_quotes() {
    let input = "
        hints 2;

        ask multichoice (2) \"question1\" {
            * \"correct1\" (1) -> \"explan1\";
            * \"incorrect1\" -> \"explan1\";
            * \"wrong1\";
            * \"right1\" (1);
        };


        ask multichoice (1) \"question2\" {
            * \"correct2\" (1) -> \"explan2\";
            * \"incorrect2\" -> \"explan2\";
            * \"wrong2\";
            * \"right2\" (1);
        } hints \"hint1\", \"hint2\";
        ";

    let result = parse(input).unwrap();
    let expected = ParsedFile {
        max_hints: 2,
        questions: vec![
            // question1
            Question::Multichoice(MultichoiceData {
                used_hints: 0,
                is_answered: false,
                selected_answer: 0,
                text: "question1".to_owned(),
                line: 3,
                max_marks: 2,
                hints: vec![],
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 1,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 0,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong1".to_owned()),
                        marks: 0,
                        explanation: None,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("right1".to_owned()),
                        marks: 1,
                        explanation: None,
                        is_chosen: false,
                    },
                ],
            }),
            // question2
            Question::Multichoice(MultichoiceData {
                text: "question2".to_owned(),
                used_hints: 0,
                is_answered: false,
                selected_answer: 0,
                line: 11,
                max_marks: 1,
                hints: vec!["hint1".to_owned(), "hint2".to_owned()],
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 1,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 0,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong2".to_owned()),
                        marks: 0,
                        explanation: None,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("right2".to_owned()),
                        marks: 1,
                        explanation: None,
                        is_chosen: false,
                    },
                ],
            }),
        ],
    };
    assert_eq!(result, expected);
}

#[test]
fn test_parse_multichoice_questions_with_hints_mixed_quotes() {
    let input = "
        hints 2;

        ask multichoice (2) 'question1' {
            * 'correct1' (1) -> 'explan1';
            * 'incorrect1' -> 'explan1';
            * 'wrong1';
            * 'right1' (1);
        };


        ask multichoice (1) \"question2\" {
            * \"correct2\" (1) -> \"explan2\";
            * \"incorrect2\" -> \"explan2\";
            * \"wrong2\";
            * \"right2\" (1);
        } hints \"hint1\", \"hint2\";
        ";

    let result = parse(input).unwrap();
    let expected = ParsedFile {
        max_hints: 2,
        questions: vec![
            Question::Multichoice(MultichoiceData {
                max_marks: 2,
                used_hints: 0,
                selected_answer: 0,
                text: "question1".to_owned(),
                line: 3,
                hints: vec![],
                is_answered: false,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct1".to_owned()),
                        marks: 1,
                        is_chosen: false,
                        explanation: Some("explan1".to_owned()),
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect1".to_owned()),
                        marks: 0,
                        is_chosen: false,
                        explanation: Some("explan1".to_owned()),
                    },
                    MultichoiceAnswer {
                        text: Some("wrong1".to_owned()),
                        marks: 0,
                        is_chosen: false,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right1".to_owned()),
                        marks: 1,
                        is_chosen: false,
                        explanation: None,
                    },
                ],
            }),
            Question::Multichoice(MultichoiceData {
                max_marks: 1,
                text: "question2".to_owned(),
                line: 11,
                is_answered: false,
                selected_answer: 0,
                used_hints: 0,
                hints: vec!["hint1".to_owned(), "hint2".to_owned()],
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct2".to_owned()),
                        marks: 1,
                        is_chosen: false,
                        explanation: Some("explan2".to_owned()),
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect2".to_owned()),
                        marks: 0,
                        is_chosen: false,
                        explanation: Some("explan2".to_owned()),
                    },
                    MultichoiceAnswer {
                        text: Some("wrong2".to_owned()),
                        marks: 0,
                        is_chosen: false,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right2".to_owned()),
                        marks: 1,
                        is_chosen: false,
                        explanation: None,
                    },
                ],
            }),
        ],
    };
    assert_eq!(result, expected);
}
