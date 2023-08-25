use crate::{Question, parser::multichoice::data::{MultichoiceData, MultichoiceAnswer}};

use super::State;

#[test]
fn test_state_from_json() {
    let input = include_str!("./example.json").to_owned();

    let result = State::from_json(input).unwrap();
    let expected = State {
        questions: vec![
            Question::Multichoice(MultichoiceData {
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("To make it go faster".to_owned()),
                        marks: 0,
                        explanation: Some("".to_owned()),
                        is_chosen: true
                    },
                    MultichoiceAnswer {
                        text: Some("To help when the Random Access Memory is full".to_owned()),
                        marks: 1,
                        explanation: Some("".to_owned()),
                        is_chosen: false
                    },
                    MultichoiceAnswer {
                        text: Some("To increase the number of files you can store".to_owned()),
                        marks: 0,
                        explanation: Some("".to_owned()),
                        is_chosen: false
                    }
                ],
                text: "What is the purpose of virtual memory in a computer?".to_owned(),
                is_answered: true,
                selected_answer: 0,
                line: 2,
                max_marks: 1,
                used_hints: 0,
                hints: vec![
                    "Virtual memory is only used when lots of programs are open".to_owned()
                ]
            }),
            Question::Multichoice(MultichoiceData {
                max_marks: 2,
                line: 8,
                selected_answer: 0,
                used_hints: 0,
                is_answered: false,
                text: "Which of these fish are red?".to_owned(),
                hints: vec![
                    "The veiltail, a name coined by William T. Innes, originated in the United States in the 1890s when Franklin Barrett of Philadelphia crossed a Japanese-bred fringetail ryukin to a telescope eye goldfish that exhibited a short, square-edged caudal.".to_owned(),
                    "Bream fish are half white.".to_owned(),
                ],
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("Bream".to_owned()),
                        explanation: Some("".to_owned()),
                        marks: 1,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("Shubunkin".to_owned()),
                        explanation: Some("".to_owned()),
                        marks: 1,
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("Veiltail".to_owned()),
                        explanation: Some("".to_owned()),
                        marks: 0,
                        is_chosen: false,
                    },
                ]
            }),
            Question::Multichoice(MultichoiceData {
                max_marks: 1,
                line: 14,
                selected_answer: 0,
                is_answered: false,
                text: "Which organ is used for digestion?".to_owned(),
                hints: vec![],
                used_hints: 0,
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("Reed organ".to_owned()),
                        marks: 0,
                        explanation: Some("".to_owned()),
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("Stomach".to_owned()),
                        marks: 1,
                        explanation: Some("".to_owned()),
                        is_chosen: false,
                    },
                    MultichoiceAnswer {
                        text: Some("Pipe organ".to_owned()),
                        marks: 0,
                        explanation: Some("".to_owned()),
                        is_chosen: false,
                    },
                ],
            })
        ],
        ..Default::default()
    };
    assert_eq!(result, expected)
}
