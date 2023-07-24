use crate::MultichoiceData;

use super::parser::*;
use super::MultichoiceAnswer;

use qqml_lexer::*;

//
// parse_multichoice()
//

#[test]
fn test_parse_multichoice_no_explanations() {
    let input = "(2) 'foo' {
        * 'bar' (2);
        * 'bazz';
    };";

    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceData::new();
    expected.set_text("foo");
    expected.set_max_marks(2);

    let mut bar = MultichoiceAnswer::new();
    bar.set_text("bar");
    bar.set_marks(2);

    let mut bazz = MultichoiceAnswer::new();
    bazz.set_marks(0);
    bazz.set_text("bazz");

    expected.add_answer(bar);
    expected.add_answer(bazz);

    let gotten = parse_multichoice(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_explanations() {
    let input = "(2) 'foo' {
        * 'bar' (2) -> 'bar explanation';
        * 'bazz' -> 'bazz explanation';
    };";

    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceData::new();
    expected.set_text("foo");
    expected.set_max_marks(2);

    let mut bar = MultichoiceAnswer::new();
    bar.set_text("bar");
    bar.set_marks(2);
    bar.set_explanation("bar explanation");

    let mut bazz = MultichoiceAnswer::new();
    bazz.set_marks(0);
    bazz.set_text("bazz");
    bazz.set_explanation("bazz explanation");

    expected.add_answer(bar);
    expected.add_answer(bazz);

    let gotten = parse_multichoice(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

// Exactly the same but with double not single quotes
//
#[test]
fn test_parse_multichoice_no_explanations_double_quotes() {
    let input = "(2) \"foo\" {
        * \"bar\" (2);
        * \"bazz\";
    };";

    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceData::new();
    expected.set_text("foo");
    expected.set_max_marks(2);

    let mut bar = MultichoiceAnswer::new();
    bar.set_text("bar");
    bar.set_marks(2);

    let mut bazz = MultichoiceAnswer::new();
    bazz.set_marks(0);
    bazz.set_text("bazz");

    expected.add_answer(bar);
    expected.add_answer(bazz);

    let gotten = parse_multichoice(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

//
// parse_multichoice_answer()
//

#[test]
fn test_parse_multichoice_answer_marks_no_explanation() {
    let input = "'foo' (3);";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(3);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_no_marks_no_explanation() {
    let input = "'foo';";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(0);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_marks_explanation() {
    let input = "'foo' (3) -> 'bar';";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(3);
    expected.set_explanation("bar");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_no_marks_explanation() {
    let input = "'foo' -> 'bar';";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(0);
    expected.set_explanation("bar");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

//
// Exactly the same but with double not single quotes.
//

#[test]
fn test_parse_multichoice_answer_marks_no_explanation_double_quotes() {
    let input = "\"foo\" (3);";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(3);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_no_marks_no_explanation_double_quotes() {
    let input = "\"foo\";";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(0);

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_marks_explanation_double_quotes() {
    let input = "\"foo\" (3) -> 'bar';";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(3);
    expected.set_explanation("bar");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}

#[test]
fn test_parse_multichoice_answer_no_marks_explanation_double_quotes() {
    let input = "\"foo\" -> 'bar';";
    let mut l = Lexer::new(input).unwrap();

    let mut expected = MultichoiceAnswer::new();
    expected.set_text("foo");
    expected.set_marks(0);
    expected.set_explanation("bar");

    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(expected, gotten);
}
