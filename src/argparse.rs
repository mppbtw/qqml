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

use std::env::args;

const OPTIONS: [&str; 12] = [
    "-h",
    "--help",
    "-v",
    "--version",
    "-c",
    "--check",
    "-j",
    "--json",
    "-l",
    "--log",
    "-r",
    "--resume",
];

pub fn has_help() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned())
}

pub fn has_check() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-c".to_owned()) || args.contains(&"--check".to_owned())
}

pub fn has_json() -> bool {
    let args = args().collect::<Vec<String>>();
    args.contains(&"-j".to_owned()) || args.contains(&"--json".to_owned())
}

pub fn get_logfile() -> Option<String> {
    let args = args().collect::<Vec<String>>();
    for (i, a) in args.iter().enumerate() {
        if a == "-l" || a == "--log" {
            return args.get(i + 1).cloned();
        }
    }
    None
}

pub fn get_resume_file() -> Option<String> {
    let args: Vec<String> = args().collect();
    for (i, a) in args.iter().enumerate() {
        if a == "-r" || a == "--resume" {
            return args.get(i + 1).cloned();
        }
    }
    None
}

pub fn get_file_arg() -> Option<String> {
    let mut skip_next_arg = false;

    for a in &args().collect::<Vec<String>>()[1..] {
        if skip_next_arg {
            skip_next_arg = false;
            continue;
        }
        if a == "-l" || a == "--log" || a == "-r" || a == "--resume" {
            skip_next_arg = true;
            continue;
        }
        if !OPTIONS.contains(&a.as_str()) {
            return Some(a.clone());
        }
    }
    None
}
