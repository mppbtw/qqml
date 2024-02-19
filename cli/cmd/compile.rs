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

use std::fs;
use std::process::exit;

use libqqml::parse;
use libqqml::render_error;

use crate::argparse::CommandBuilder;
use crate::argparse::Flag;
use crate::argparse::FlagArgumentType;
use crate::Command;

pub fn init(parent: &mut Command) {
    parent.add_command(Command::new(CommandBuilder {
        usage: "compile",
        long:  "Compile QQML to a JSON format, which can later be ran.",
        short: "Compile QQML to JSON",
        args:  1,
        run:   Some(|args, flags| {
            let src_path = &*args[0];
            let src = match fs::read_to_string(src_path) {
                Ok(s) => s,
                Err(e) => {
                    println!("Failed to read from path {}: {}", src_path, e);
                    exit(1);
                }
            };

            match parse(&src, Some(src_path.to_owned())) {
                Ok(j) => {
                    let output = j.to_json();
                    match flags.get("--output") {
                        Some(f) => {
                            let out_path = f.string().unwrap();
                            match fs::write(out_path, output) {
                                Ok(_) => exit(0),
                                Err(e) => {
                                    println!("Failed to write to {}: {}", out_path, e);
                                    exit(1);
                                }
                            }
                        }
                        None => println!("{}", output),
                    }
                    exit(0);
                }
                Err(e) => {
                    println!("{}", render_error(&src, &e, Some(&src_path.to_string())));
                    exit(1);
                }
            }
        }),
        flags: vec![Flag {
            usage:   "--output",
            aliases: vec!["-o"],
            arg:     Some(FlagArgumentType::String),
            long:    Some("The path to output to."),
        }],
    }))
}
