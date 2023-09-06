//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'MrPiggyPegasus'
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
use crate::parser::multichoice::parse_answer::parse_multichoice_answer;

#[test]
fn test_replacement_tolerance() {
    let input1 = "
        'question text' (1x -> 4;
        ";

    let input2 = "
        x (1) -> 4;
        ";

    let input3 = "
        'question text' x1) -> 'explanation text';
        ";

    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input1))
            .unwrap_err()
            .errors
            .len(),
        2
    );
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input2))
            .unwrap_err()
            .errors
            .len(),
        2
    );
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input3))
            .unwrap_err()
            .errors
            .len(),
        1
    );
}

#[test]
fn test_positive_tolerance() {
    let input1 = "
        x'question text'/ (1) -> 'some explanation';
        ";

    let input2 = "
        'question text' x(1) -> 'some explanation';
        ";

    let input3 = "
        x'question text' (1) -> 'some explanation'x;
        ";

    let input4 = "
        x/2'question text' (1) -> 'some explanation';
        ";

    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input1))
            .unwrap_err()
            .errors
            .len(),
        2
    );
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input2))
            .unwrap_err()
            .errors
            .len(),
        1
    );
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input3))
            .unwrap_err()
            .errors
            .len(),
        2
    );
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input4))
            .unwrap_err()
            .errors
            .len(),
        3
    );
}

#[test]
fn test_negative_tolerance() {
    let input1 = "
        (1) -> 'some explanation';
        ";
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input1))
            .unwrap_err()
            .errors
            .len(),
        1
    );

    let input2 = "
        'question text' ()  'some explanation';
        ";
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input2))
            .unwrap_err()
            .errors
            .len(),
        2
    );

    let input3 = "
        'question text' (1) -> 'some explanation'
        ";
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input3))
            .unwrap_err()
            .errors
            .len(),
        1
    );

    let input4 = "
        1) -> 'some explanation'
        ";
    assert_eq!(
        parse_multichoice_answer(&mut Lexer::new(input4))
            .unwrap_err()
            .errors
            .len(),
        3
    );
}
