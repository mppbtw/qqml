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

use std::fs::read_to_string;
use std::io::stdout;
use std::io::Write;
use std::process::exit;

use libqqml::render_error;
use libqqml::run;

use crate::argparse::CommandBuilder;
use crate::argparse::Flag;
use crate::Command;

pub fn init(parent: &mut Command) {
    parent.add_command(Command::new(CommandBuilder {
        usage: "run",
        short: "Run a QQML file from the specified path",
        long:  "Run a QQML file from the specified path",
        args:  1,
        run:   Some(|args| {
            let path = args[0].to_owned();
            match read_to_string(&path) {
                Ok(s) => match run(&s, Some(&path), None) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("{}", render_error(&s, &e, Some(&path)));
                    }
                },
                Err(e) => {
                    println!("Failed to read from file {}: {}", path, e.to_string());
                    exit(1)
                }
            }
            exit(0);
        }),
        flags: vec![Flag {
            short: Some("-l"),
            long:  "--log",
            arg:   None,
        }],
    }))
}
