use crate::*;

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
    assert!(matches!(
        output.unwrap_err().errors.get(0).unwrap(),
        Error::UnterminatedLiteral(_)
    ));
}

#[test]
fn test_max_marks_impossible_with_other_errors() {
    let input = "
        somethingUpHereToCauseAnError
        ask multichoice (10) 'how many of your bases are belong to us' {
            * 'all of them' (3);
            * 'some of them';
        };
        ";
    let output = parse(input);
    dbg!(output.clone().unwrap_err());

    // In the case that a warning is emitted even if the parsing fails,
    // the report should (if im not stupid (not guaranteed)) extract
    // the warnings which it can from questions and returns them as
    // part of the report.
    assert_eq!(output.unwrap_err().warnings.len(), 1);
}

#[test]
fn test_max_marks_impossible() {
    let input = "
        ask multichoice (10) 'how many of your bases are belong to us' {
            * 'all of them' (3);
            * 'some of them';
        };
        ";
    let output = parse(input);
    dbg!(output.clone().unwrap());

    let questions = output.unwrap().questions;

    // In the case that a warning is emitted even if the parsing fails,
    // the report should (if im not stupid (not guaranteed)) extract
    // the warnings which it can from questions and returns them as
    // part of the report.
    assert!(matches!(questions[0], Question::Multichoice(_)));

    for q in questions.to_vec() {
        match q {
            Question::Multichoice(d) => {
                assert_eq!(d.warnings.len(), 1);
            }
            _ => (),
        };
    }
}
