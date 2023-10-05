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

const SPACE_BETWEEN_CMD_AND_DESC: usize = 2;

use std::convert::Infallible;
use std::process::exit;

use super::flag::*;
use super::utils::separate_lines;
use crate::argparse::utils::the_one_and_only_left_pad;

/// This is a pretty simple argparsing solution inspired by Go's Cobra library.
pub struct Command {
    children: Vec<Command>,
    usage:    &'static str,
    long:     &'static str,
    short:    &'static str,
    flags:    Vec<Flag>,
    run:      Option<fn(&[String], AnsweredFlags) -> Infallible>,
    args:     usize,
}

impl Command {
    pub fn new(cmd_info: CommandBuilder) -> Self {
        let mut new = Self {
            children: vec![],
            usage:    cmd_info.usage,
            long:     cmd_info.long,
            short:    cmd_info.short,
            run:      cmd_info.run,
            args:     cmd_info.args,
            flags:    cmd_info.flags,
        };

        if !new.flags.is_empty() && new.run.is_none() {
            panic!("Only leaf commands (without subcommands) can have custom flags!");
        }

        // Check if the --help flag already exists
        if !new
            .flags
            .iter()
            .map(|f| f.usage)
            .collect::<Vec<&str>>()
            .contains(&"--help")
        {
            let mut help_flag = Flag {
                usage:   "--help",
                aliases: vec![],
                arg:     None,
                long:    Some("Show this help message"),
            };

            // Check if the -h alias has already been taken (either by an alias or the full
            // name of a flag)
            let alias_avaliable = 'a: {
                for f in new.flags.iter() {
                    if f.aliases.contains(&"-h") || f.usage == "-h" {
                        break 'a false;
                    }
                }
                true // treesitter chokes on this keyword for some reason
            };
            if alias_avaliable {
                help_flag.aliases.push("-h");
            }
            new.flags.push(help_flag);
        }
        new
    }

    pub fn add_command(&mut self, cmd: Command) {
        self.children.push(cmd);
    }

    pub fn help_screen(&self) -> Infallible {
        // Long description
        println!("{}\n", self.long);

        // Usage part
        let mut usage_msg = String::new();
        usage_msg += "Usage:\n  ";
        usage_msg += self.usage;
        usage_msg += " ";

        if !self.children.is_empty() {
            usage_msg += "[command]";
            usage_msg += " "
        }

        if self.args > 0 {
            usage_msg += &format!("[{} arguments]", self.args);
            usage_msg += " ";
        }

        if self.flags.len() > 1 {
            usage_msg += "[flags]";
            usage_msg += " ";
        };

        println!("{}\n", usage_msg);

        // Commands list
        if !self.children.is_empty() {
            println!(
                "Commands:\n{}\n",
                separate_lines(
                    self.children
                        .iter()
                        .map(|c| vec![c.usage.to_string(), c.short.to_string()])
                        .collect::<Vec<Vec<String>>>(),
                    2
                )
                .unwrap()
                .split('\n')
                .map(|line| the_one_and_only_left_pad(line, 3, ' '))
                .collect::<Vec<String>>()
                .join("\n"),
            );
        }

        // Flags list
        if !self.flags.is_empty() {
            println!(
                "Flags:\n{}",
                separate_lines(
                    self.flags
                        .iter()
                        .map(|c| vec![
                            c.usage.to_string(),
                            c.aliases.join(", "),
                            c.long.unwrap_or("").to_string()
                        ])
                        .collect::<Vec<Vec<String>>>(),
                    2
                )
                .unwrap()
                .split('\n')
                .map(|line| the_one_and_only_left_pad(line, 3, ' '))
                .collect::<Vec<String>>()
                .join("\n"),
            );
        }
        exit(0);
    }

    fn lookup_command(&self, arg: &str) -> Option<&Command> {
        self.children.iter().find(|&child| child.usage == arg)
    }

    fn execute_leaf(&self, mut args: Vec<String>) -> Infallible {
        let mut answered_flags: Vec<AnsweredFlag> = vec![];

        let mut flag_indeces: Vec<usize> = vec![];
        let mut i = 0;
        while i < args.len() {
            let arg = args.get(i).unwrap();
            let f = match self.lookup_flag(arg) {
                None => {
                    i += 1;
                    continue;
                }
                Some(f) => f,
            };

            flag_indeces.push(i);
            answered_flags.push(AnsweredFlag {
                usage: f.usage,
                arg:   'a: {
                    if f.arg.is_none() {
                        break 'a None;
                    }

                    if !matches!(f.arg, Some(FlagArgumentType::String)) {
                        unimplemented!("Only the string argument type is implemented yet.");
                    }

                    let flag_argument = args.get(i + 1).cloned();
                    if flag_argument.is_none() {
                        println!("The f {} requires an argument of type STRING", f.usage);
                        exit(1);
                    }

                    flag_indeces.push(i + 1);
                    i += 1;
                    Some(AnsweredFlagArgument::String(
                        flag_argument.unwrap().to_owned(),
                    ))
                },
            });
            i += 1;
        }

        // Sort backwards so we dont have to shift the indeces when removing them from
        // the args
        flag_indeces.sort_by(|a, b| b.cmp(a));

        for i in flag_indeces {
            args.remove(i);
        }

        let flags_result = AnsweredFlags {
            flags: answered_flags,
        };

        if flags_result.get("--help").is_some() {
            self.help_screen();
        }

        if args.len() != self.args {
            println!("Expected {} arguments, got {}", self.args, args.len());
            exit(0);
        }

        self.run.unwrap()(&args[..], flags_result);

        exit(0);
    }

    /// Call this on the root command to initate the parsing sequence.
    pub fn execute(&self, args: &[String]) -> Infallible {
        // We only want the positional arguments from this, flags are handled seperately
        let mut args: Vec<String> = args.into();

        // If it's a leaf command
        if self.run.is_some() {
            self.execute_leaf(args);
        }

        // If it has more subcommands
        match args.get(0) {
            Some(arg) => {
                if let Some(c) = self.lookup_command(arg) {
                    c.execute(&args[1..]);
                } else if match self.lookup_flag(arg) {
                    Some(f) => f.usage == "--help",
                    None => false,
                } {
                    self.help_screen();
                } else {
                    println!("Unknown argument or subcommand");
                    exit(1);
                };
            }
            None => {
                self.help_screen();
            }
        }
        exit(0)
    }

    fn lookup_flag(&self, arg: &str) -> Option<&Flag> {
        self.flags
            .iter()
            .find(|&f| f.usage == arg || f.aliases.contains(&arg))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CommandBuilder {
    pub usage: &'static str,
    pub long:  &'static str,
    pub short: &'static str,
    pub run:   Option<fn(&[String], AnsweredFlags) -> Infallible>,
    pub args:  usize,
    pub flags: Vec<Flag>,
}