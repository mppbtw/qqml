use super::State;
use crate::parser::multichoice::data::MultichoiceAnswer;
use crate::parser::multichoice::data::MultichoiceData;
use crate::Question;

#[test]
fn test_state_from_json() {
    let input = include_str!("./example.json").to_owned();

    let result = State::from_json(input).unwrap();
    let expected = State {
        cols: 0,
        current_question_index: 0,
        questions_len: 0,
        current_achieved_marks: 0,
        current_hints_available: 0,
        has_watched_final_cutsene: false,
        hints_used: 0,
        max_hints: 2,
        path_to_source: Some("example.qqml".to_owned()),
        questions: vec![
            Question::Multichoice(MultichoiceData {
                answers: vec![
                    MultichoiceAnswer {
                        text: Some("To make it go faster".to_owned()),
                        marks: 0,
                        explanation: Some("Virtual memory does not help the CPU or GPU run any faster, ".to_owned()
                                          + "it only helps memory."),
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
                        explanation: Some("Virtual memory actually takes away some of your disk space.".to_owned()),
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
                line: 11,
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
                        explanation: Some("Veiltail actually come in many colours".to_owned()),
                        marks: 0,
                        is_chosen: false,
                    },
                ]
            }),
            Question::Multichoice(MultichoiceData {
                max_marks: 1,
                line: 21,
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
                        explanation: Some("Well, if your talking about the oesophagus then...".to_owned()),
                        is_chosen: false,
                    },
                ],
            })
        ],
    };
    assert_eq!(result, expected)
}
