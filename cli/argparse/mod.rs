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

use std::convert::Infallible;
use std::process::exit;

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
        Self {
            children: vec![],
            usage:    cmd_info.usage,
            long:     cmd_info.long,
            short:    cmd_info.short,
            run:      cmd_info.run,
            args:     cmd_info.args,
            flags:    cmd_info.flags,
        }
    }

    pub fn add_command(&mut self, cmd: Command) {
        self.children.push(cmd);
    }

    pub fn help_screen(&self) -> Infallible {
        println!("placeholder help screen");
        exit(0);
    }

    fn lookup_command(&self, arg: &str) -> Option<&Command> {
        for child in self.children.iter() {
            if child.usage == arg {
                return Some(&child);
            }
        }
        None
    }

    /// Call this on the root command to initate the parsing sequence.
    pub fn execute(&self, args: &[String]) -> Infallible {
        // We only want the positional arguments from this, flags are handled seperately
        let mut args: Vec<String> = args.into();
        let mut answered_flags: Vec<AnsweredFlag> = vec![];

        let mut i = 0;
        while i < args.len() {
            let arg = args.get(i).unwrap().to_owned();
            for flag in self.flags.iter() {
                if flag.long == &arg || flag.aliases.contains(&arg.as_str()) {
                    args.remove(i);

                    answered_flags.push(AnsweredFlag {
                        long: flag.long,
                        arg:  'a: {
                            if flag.arg.is_none() {
                                break 'a None;
                            }

                            if !matches!(flag.clone().arg.unwrap(), FlagArgumentType::String) {
                                unreachable!("Only the string argument type is implemented yet.");
                            }

                            let flag_argument = args.get(i).cloned();
                            if flag_argument.is_none() {
                                println!(
                                    "The flag {} requires an argument of type STRING",
                                    flag.long
                                );
                                exit(1);
                            }
                            args.remove(i);
                            Some(AnsweredFlagArgument::String(
                                flag_argument.unwrap().to_owned(),
                            ))
                        },
                    })
                }
            }
            i += 1;
        }

        // If it's a leaf command
        if self.run.is_some() {
            if args.len() != self.args {
                println!("Expected {} arguments, got {}", self.args, args.len());
                exit(0);
            }
            self.run.unwrap()(&args[..], AnsweredFlags {
                flags: answered_flags,
            });

        // It has subcommands
        } else {
            match args.get(0) {
                Some(arg) => {
                    if let Some(c) = self.lookup_command(arg) {
                        c.execute(&args[1..])
                    } else {
                        exit(1);
                    };
                }
                None => {
                    self.help_screen();
                }
            }
        }

        exit(0)
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

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Flag {
    pub long:    &'static str,
    pub aliases: Vec<&'static str>,
    pub arg:     Option<FlagArgumentType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlagArgumentType {
    Int,
    Uint,
    String,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnsweredFlag {
    pub long: &'static str,
    pub arg:  Option<AnsweredFlagArgument>,
}
impl AnsweredFlag {
    pub fn int(&self) -> Result<&isize, ErrIncorrectArgType> {
        self.arg.as_ref().unwrap().int()
    }

    pub fn uint(&self) -> Result<&usize, ErrIncorrectArgType> {
        self.arg.as_ref().unwrap().uint()
    }

    pub fn string(&self) -> Result<&String, ErrIncorrectArgType> {
        self.arg.as_ref().unwrap().string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnsweredFlagArgument {
    Int(isize),
    Uint(usize),
    String(String),
}
impl AnsweredFlagArgument {
    pub fn int(&self) -> Result<&isize, ErrIncorrectArgType> {
        match self {
            Self::Int(i) => Ok(i),
            _ => Err(ErrIncorrectArgType),
        }
    }

    pub fn uint(&self) -> Result<&usize, ErrIncorrectArgType> {
        match self {
            Self::Uint(u) => Ok(u),
            _ => Err(ErrIncorrectArgType),
        }
    }

    pub fn string(&self) -> Result<&String, ErrIncorrectArgType> {
        match self {
            Self::String(s) => Ok(s),
            _ => Err(ErrIncorrectArgType),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErrIncorrectArgType;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct AnsweredFlags {
    pub flags: Vec<AnsweredFlag>,
}
impl AnsweredFlags {
    pub fn get(&self, name: &str) -> Option<&AnsweredFlag> {
        for flag in self.flags.iter() {
            if flag.long == name {
                return Some(&flag);
            }
        }
        None
    }
}
