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

use crate::argparse::utils::separate_lines;
use crate::argparse::utils::strip_whitespace;
use crate::argparse::utils::the_one_and_only_left_pad;

#[test]
pub fn test_separate_lines() {
    let input: Vec<Vec<String>> =
        vec![vec!["a".into(), "b".into()], vec!["cc".into(), "dd".into()]];
    let expected = "a   b\ncc  dd";
    let result = separate_lines(input, 2).unwrap();
    dbg!(&result);
    assert_eq!(result, expected);
}

#[test]
pub fn test_left_pad() {
    let input = "super mario in real life";
    let expected = "    super mario in real life";
    let result = the_one_and_only_left_pad(input.to_string(), 4);
    dbg!(&result);
    assert_eq!(result, expected);
}

#[test]
pub fn test_strip_whitespace() {
    let mut input = "ich bin ein berliner    ".to_owned();
    strip_whitespace(&mut input);
    dbg!(&input);
    assert_eq!("ich bin ein berliner", input);
}
