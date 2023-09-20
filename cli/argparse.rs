use std::{convert::Infallible, process::exit};

/// This is a pretty simple argparsing solution inspired by Go's Cobra library.
pub struct Command {
    children: Vec<Command>,
    usage:    &'static str,
    long:     &'static str,
    short:    &'static str,
    run:      Option<fn()>,
}

impl Command {
    pub fn new(cmd_info: CommandBuilder) -> Self {
        Self {
            children: vec![],
            usage:    cmd_info.usage,
            long:     cmd_info.long,
            short:    cmd_info.short,
            run:      cmd_info.run,
        }
    }

    pub fn add_command(&mut self, cmd: Command) {
        self.children.push(cmd);
    }

    /// Call this on the root command to initate the parsing sequence.
    pub fn execute(&self) -> Infallible {
        exit(0)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct CommandBuilder {
    pub usage: &'static str,
    pub long:  &'static str,
    pub short: &'static str,
    pub run:   Option<fn()>,
}
