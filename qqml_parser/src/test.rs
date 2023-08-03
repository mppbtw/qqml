use crate::*;

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
        warnings: vec![],
        max_hints: 2,
        questions: vec![

            // question1
            Question::Multichoice(MultichoiceData {
                text: Some("question1".to_owned()),
                warnings: vec![],
                line: 3,
                max_marks: Some(2),
                hints: vec![],
                chosen_answer: None,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong1".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right1".to_owned()),
                        marks: 1,
                        explanation: None
                    },
                ],
            }),

            // question2
            Question::Multichoice(MultichoiceData {
                text: Some("question2".to_owned()),
                warnings: vec![],
                line: 11,
                max_marks: Some(1),
                hints: vec!["hint1".to_owned(), "hint2".to_owned()],
                chosen_answer: None,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong2".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right2".to_owned()),
                        marks: 1,
                        explanation: None
                    },
                ],
            })
        ]
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
        warnings: vec![],
        max_hints: 2,
        questions: vec![

            // question1
            Question::Multichoice(MultichoiceData {
                text: Some("question1".to_owned()),
                warnings: vec![],
                line: 3,
                max_marks: Some(2),
                hints: vec![],
                chosen_answer: None,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong1".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right1".to_owned()),
                        marks: 1,
                        explanation: None
                    },
                ],
            }),

            // question2
            Question::Multichoice(MultichoiceData {
                text: Some("question2".to_owned()),
                warnings: vec![],
                line: 11,
                max_marks: Some(1),
                hints: vec!["hint1".to_owned(), "hint2".to_owned()],
                chosen_answer: None,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong2".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right2".to_owned()),
                        marks: 1,
                        explanation: None
                    },
                ],
            })
        ]
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
        warnings: vec![],
        max_hints: 2,
        questions: vec![

            // question1
            Question::Multichoice(MultichoiceData {
                text: Some("question1".to_owned()),
                warnings: vec![],
                line: 3,
                max_marks: Some(2),
                hints: vec![],
                chosen_answer: None,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect1".to_owned()),
                        explanation: Some("explan1".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong1".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right1".to_owned()),
                        marks: 1,
                        explanation: None
                    },
                ],
            }),

            // question2
            Question::Multichoice(MultichoiceData {
                text: Some("question2".to_owned()),
                warnings: vec![],
                line: 11,
                max_marks: Some(1),
                hints: vec!["hint1".to_owned(), "hint2".to_owned()],
                chosen_answer: None,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("correct2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 1,
                    },
                    MultichoiceAnswer {
                        text: Some("incorrect2".to_owned()),
                        explanation: Some("explan2".to_owned()),
                        marks: 0,
                    },
                    MultichoiceAnswer {
                        text: Some("wrong2".to_owned()),
                        marks: 0,
                        explanation: None,
                    },
                    MultichoiceAnswer {
                        text: Some("right2".to_owned()),
                        marks: 1,
                        explanation: None
                    },
                ],
            })
        ]
    };
    assert_eq!(result, expected);
}

#[test]
fn test_unterminated_literal() {
    let input = "
        hints 2;
        ask multichoice (1) 'where is the sun {
            * 'the moon' (1);
            * 'the earth' (0);
        };
        ";
    let output = parse(input);
    dbg!(&output);
    assert!(output.is_err());
}
