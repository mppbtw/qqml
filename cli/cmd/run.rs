use std::process::exit;

use crate::argparse::CommandBuilder;
use crate::Command;

pub fn init(parent: &mut Command) {
    parent.add_command(Command::new(CommandBuilder {
        usage: "run",
        short: "Run a QQML file from the specified path",
        long:  "Run a QQML file from the specified path",
        run:   Some(|_| {
            println!("Inside the run callback.");
            exit(0);
        }),
    }))
}
