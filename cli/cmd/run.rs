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
use std::process::exit;

use libqqml::render_error;
use libqqml::run;
use libqqml::run_from_state;
use libqqml::State;

use crate::argparse::CommandBuilder;
use crate::argparse::Flag;
use crate::argparse::FlagArgumentType;
use crate::Command;

pub fn init(parent: &mut Command) {
    parent.add_command(Command::new(CommandBuilder {
        usage: "run",
        short: "Run a QQML file from the specified path",
        long:  "Run a QQML file from the specified path, or report any parsing errors.",
        args:  1,
        run:   Some(|args, flags| {
            // Check for the path to a log file
            let log_path: Option<&String> = match flags.get("--log") {
                None => None,
                Some(f) => Some(f.string().unwrap()),
            };

            // We can be sure that args has a length of 1 because of my epic argparsing
            // library (tm)
            let path = args[0].to_owned();
            let f_contents = match read_to_string(&path) {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed to read from file {}: {}", path, e.to_string());
                    exit(1)
                }
            };

            match flags.get("--json") {
                None => {
                    match run(&f_contents, Some(&path), log_path) {
                        Ok(_) => (),
                        Err(e) => {
                            println!("{}", render_error(&f_contents, &e, Some(&path)));
                            exit(1);
                        }
                    }
                    exit(0);
                }
                Some(_) => {
                    let state = match State::from_json(f_contents) {
                        Ok(s) => s,
                        Err(e) => {
                            println!("Failed to construct state from JSON: {:?}", e);
                            exit(1);
                        }
                    };

                    run_from_state(state, log_path);
                }
            }
        }),
        flags: vec![
            Flag {
                aliases: vec!["-l"],
                long:    "--log",
                arg:     Some(FlagArgumentType::String),
            },
            Flag {
                aliases: vec!["-j"],
                long:    "--json",
                arg:     None,
            },
        ],
    }))
}
