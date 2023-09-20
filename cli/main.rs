use std::convert::Infallible;
use std::process::exit;

mod cmd;
mod argparse;

use argparse::CommandBuilder;
use argparse::Command;

fn main() -> Infallible {
    println!("running cli command parsing");
    let root_cmd = Command::new(CommandBuilder {
        short: "wad",
        long: "adawdaw",
        ..Default::default()
    });

    root_cmd.execute();

    exit(0)
}
