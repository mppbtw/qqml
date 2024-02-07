//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2023 'mppbtw'
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
use std::env::args;
use std::process::exit;
use std::sync::Arc;

#[allow(unused)]
mod argparse;
mod cmd;

use argparse::Command;
use argparse::CommandBuilder;

fn main() -> Infallible {
    let mut root_cmd = Command::new(CommandBuilder {
        short: "The QQML interpreter",
        long: "The QQML interpreter.",
        usage: "qqml",
        args: 0,
        ..Default::default()
    });

    cmd::init(&mut root_cmd);
    let args = args().collect::<Arc<[String]>>();
    root_cmd.execute(&args[1..]);

    exit(0)
}
