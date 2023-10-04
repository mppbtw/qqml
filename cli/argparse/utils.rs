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

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct LineSeparationError;

/// Will transform
/// [
///     ["foo", "bar"],
///     ["fizzle", "bazz"]
/// ]
/// into
/// "foo     bar \
///  fizzle  bazz"
///
///        ^^ gaps_size
pub fn separate_lines<S>(
    mut inp: Vec<Vec<S>>, // This is mutable because its already a nice data structure to construct
    // the output with.
    gaps_size: usize,
) -> Result<String, LineSeparationError>
where
    S: ToString + From<String>,
{
    let mut output: Vec<Vec<String>> = vec![];
    for (i, line) in inp.iter().enumerate() {
        output.push(vec![]);
        for col in line {
            output[i].push("".to_owned());
        }
    }

    if inp.len() == 0 {
        return Ok("".to_owned());
    }

    // Make sure that every one of the inner vectors is the same size.
    let longest_line = match inp.iter().map(|v| v.len()).max() {
        Some(l) => l,
        None => return Err(LineSeparationError),
    };

    for line in inp.iter_mut() {
        (0..line.len() - longest_line).for_each(|_| line.push(String::from("").into()));
    }
    for line in output.iter_mut() {
        (0..line.len() - longest_line).for_each(|_| line.push(String::from("")));
    }

    for (i, col) in inp[0].iter().enumerate() {
        'a: {
            let longest_in_col = match inp.iter().map(|l| l[i].to_string().len()).max() {
                Some(n) => n,
                None => return Err(LineSeparationError),
            };

            let chars_before_next_col = longest_in_col + gaps_size + 1;
            for (j, lines) in inp.iter().enumerate() {
                let item: String = lines[i].to_string();
                output[j][i] = inp[j][i].to_string();
                (0..chars_before_next_col - item.len()).for_each(|_| output[j][i] += " ");
            }
        }
    }

    Ok(output
        .iter()
        .map(|l| l.join(""))
        .collect::<Vec<String>>()
        .join("\n"))
}
