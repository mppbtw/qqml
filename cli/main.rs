use std::convert::Infallible;
use std::env::args;
use std::process::exit;
use std::sync::Arc;

mod argparse;
mod cmd;

use argparse::Command;
use argparse::CommandBuilder;

fn main() -> Infallible {
    println!("running cli command parsing");
    let mut root_cmd = Command::new(CommandBuilder {
        short: "The QQML interpreter",
        long:  "The QQML interpreter.",
        run:   None,
        usage: "",
    });

    cmd::init(&mut root_cmd);
    let args = args().collect::<Arc<[String]>>();
    root_cmd.execute(&args[1..]);

    exit(0)
}
