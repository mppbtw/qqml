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

use std::fs::write;
use std::process::exit;

use crate::c_utils::*;

pub fn cleanup_and_exit_with_log<C: AsRef<[u8]> + std::fmt::Display>(log: C, path: &String) -> ! {
    unsafe {
        exit_alt_screen();
        show_cursor();
    }
    if let Err(e) = write(path, &log) {
        println!("Failed to write to log file {}: {}", log, e);
    }
    exit(0)
}

/// Undo all of the terminal configuration done by the evaluator and return to
/// the shell or however the QQML binary was run in the first place
pub fn cleanup_and_exit() -> ! {
    unsafe {
        exit_alt_screen();
        show_cursor();
    }
    exit(0)
}
