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

pub struct LineSeparationError;

pub fn separate_lines<S>(inp: Vec<Vec<S>>, gaps_size: usize) -> Result<String, LineSeparationError>
where
    S: ToString,
{
    let mut output = String::new();
    inp.iter().for_each(|v| {
        let longest = match v.iter().map(|s| s.to_string().len()).max() {
            Some(l) => l,
            None => 0,
        };
    });
    Ok(output)
}
