//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
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
use crate::lexer::token::Token;
use crate::lexer::token::TokenData;
use crate::parser::multichoice::data::MultichoiceAnswer;
use crate::parser::multichoice::data::MultichoiceData;
use crate::parser::multichoice::parser::parse_multichoice;

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
        is_answered:     false,
        used_hints:      0,
        selected_answer: 0,
        max_marks:       1,
        hints:           vec![],
        answers:         vec![],
        line:            1,
        text:            "title".to_owned(),
    };

    expected.answers.push(MultichoiceAnswer {
        is_chosen:   false,
        text:        Some("correct".to_owned()),
        marks:       1,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        is_chosen:   false,
        text:        Some("incorrect".to_owned()),
        marks:       0,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        is_chosen:   false,
        text:        Some("inc".to_owned()),
        marks:       0,
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
        is_answered:     false,
        used_hints:      0,
        selected_answer: 0,
        max_marks:       1,
        hints:           vec![],
        answers:         vec![],
        line:            1,
        text:            "title".to_owned(),
    };

    expected.answers.push(MultichoiceAnswer {
        is_chosen:   false,
        text:        Some("correct".to_owned()),
        marks:       1,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        is_chosen:   false,
        text:        Some("incorrect".to_owned()),
        marks:       0,
        explanation: Some("explanation".to_owned()),
    });

    expected.answers.push(MultichoiceAnswer {
        is_chosen:   false,
        text:        Some("inc".to_owned()),
        marks:       0,
        explanation: None,
    });

    assert_eq!(expected, result);
}
ult);
}
