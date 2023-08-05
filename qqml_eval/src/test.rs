use super::utils::*;
use crate::*;
use qqml_parser::*;

#[test]
fn test_get_term_size() {
    // The only good way to test this function unfortunately.
    dbg!(get_term_size().unwrap().width);
    dbg!(get_term_size().unwrap().height);
}

#[test]
fn manually_inspect_output() {
    let t = Target {
        path_to_source: Some("~/some/path/to/source".to_owned()),
        max_hints: 3,
        hints_used: 1,
        current_question: 0,
        questions: vec![Question::Multichoice(MultichoiceData {
            text: Some("question".to_owned()),
            max_marks: Some(1),
            answers: vec![],
            hints: vec!["hint1".to_owned(), "hint2".to_owned()],
            chosen_answer: 0,
            warnings: vec![],
            used_hints: 1,
            line: 0,
        })],
    };

    let screen = Screen::from(t);
    let output = screen.render().unwrap();
    println!("{}", output);

    // Change this to false to test the output lol
    assert!(true);
}
