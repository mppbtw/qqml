use std::convert::Infallible;
use std::process::exit;

mod argparse;

 fn main() -> Infallible {
    println!("running cli command parsing");
    let rootCmd = argparse::Command {

    };
    exit(0)
}
