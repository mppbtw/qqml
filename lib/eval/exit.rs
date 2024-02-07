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

use std::fs::write;
use std::process::exit;

use rtermutils::*;

pub fn cleanup_and_exit(exit_print: Option<String>, log_file_path: Option<String>) -> ! {
    unsafe {
        exit_alt_screen();
        show_cursor();
    }
    if let Some(msg) = exit_print {
        if let Some(path) = log_file_path {
            write(path, msg).unwrap();
        }
    }
    exit(0)
}
