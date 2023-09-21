use std::convert::Infallible;
use std::env::args;
use std::process::exit;
use std::sync::Arc;

/// This is a pretty simple argparsing solution inspired by Go's Cobra library.
pub struct Command {
    children: Vec<Command>,
    usage:    &'static str,
    long:     &'static str,
    short:    &'static str,
    run:      Option<fn(&[String]) -> Infallible>,
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

    pub fn help_screen(&self) -> Infallible {
        println!("placeholder help screen");
        exit(0);
    }

    fn lookup_command(&self, arg: &str) -> Option<&Command> {
        for child in self.children.iter() {
            if child.usage == arg {
                return Some(&child)
            }
        }
        None
    }

    /// Call this on the root command to initate the parsing sequence.
    pub fn execute<A: Into<Arc<[String]>>>(&self, args: A) -> Infallible {
        let args: &[String] = &args.into();

        // If it's a leaf command
        if self.run.is_some() {
            self.run.unwrap()(&args[..]);
        // It has subcommands
        } else {
            match args.get(0) {
                Some(arg) => {
                    if let Some(c) = self.lookup_command(arg) {
                        c.execute(&args[1..])
                    } else {
                        println!("Unknown command: {}", arg);
                        exit(1);
                    };
                },
                None => {self.help_screen();},
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
    pub run:   Option<fn(&[String]) -> Infallible>,
}
