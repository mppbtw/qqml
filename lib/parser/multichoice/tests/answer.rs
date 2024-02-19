//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

use crate::lexer::core::Lexer;
use crate::parser::multichoice::data::MultichoiceAnswer;
use crate::parser::multichoice::parse_answer::parse_multichoice_answer;

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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
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
    let gotten = parse_multichoice_answer(&mut l).unwrap();
    assert_eq!(gotten, expected);
}
