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

use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

use super::exit::cleanup_and_exit;
use super::render::pad_to_width;
use super::state::State;
use crate::c_utils::*;
use crate::eval::exit::cleanup_and_exit_with_log;

pub fn end_screen(s: &mut State, log_path: Option<String>) {
    let percent = if s.achieved_marks() == 0 {
        0.0
    } else {
        ((s.achieved_marks() as f64 / s.get_max_marks() as f64) * 100.0).floor()
    };

    let art: String;
    if percent <= 25.0 {
        art = include_str!("../../ascii/bart.ascii").to_string();
    } else if percent <= 50.0 {
        art = include_str!("../../ascii/squidward.ascii").to_string();
    } else if percent <= 75.0 {
        art = include_str!("../../ascii/popeye.ascii").to_string();
    } else {
        art = include_str!("../../ascii/homer.ascii").to_string();
    }

    if !s.has_watched_final_cutsene() {
        ascii_scroll(art, 200);
        s.watch_final_cutsene();
    } else {
        unsafe { clear_screen() }
    }
    println!("You got {}/{}", s.achieved_marks(), s.get_max_marks());
    println!("Press enter to review your answers, or any other key to quit.");
    if unsafe { read_single_char() } != b'\n' {
        if let Some(p) = log_path {
            cleanup_and_exit_with_log(s.to_json(), &p);
        }
        cleanup_and_exit();
    }
}

fn ascii_scroll(art: String, time_per_line: u64) {
    let width = unsafe { clear_screen_with_width() };
    let art = make_lines_same_len(art)
        .lines()
        .map(|l| pad_to_width(l, width.try_into().unwrap()).unwrap_or(l.to_string()))
        .collect::<Vec<String>>()
        .join("\n");

    let mut art_top_line_position = -(art.lines().count() as i32);
    loop {
        let height = unsafe { clear_screen_with_height() };
        if height == 0 {
            continue;
        }
        let art_height = art.lines().count() as i32;
        if art_top_line_position == height {
            break;
        }
        if art_top_line_position >= 0 {
            if (art_top_line_position + art_height) >= height {
                let visible_height = height - art_top_line_position;

                let visible_art = &art
                    .split('\n')
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[0..visible_height as usize]
                    .join("\n");

                (0..(height - visible_height)).for_each(|_| println!());
                print!("{}", visible_art);
                stdout().flush().unwrap();
            } else {
                (0..art_top_line_position).for_each(|_| println!());
                print!("{}", art);
                stdout().flush().unwrap();
            }
        } else if art_top_line_position < 0 {
            print!("{}", art);
            stdout().flush().unwrap();
            (0..height - (art_height + art_top_line_position)).for_each(|_| println!());
        }
        art_top_line_position += 1;
        sleep(Duration::from_millis(time_per_line));
    }
    unsafe { clear_screen() }
}

fn make_lines_same_len(s: String) -> String {
    let mut lines: Vec<String> = s.lines().map(|s| s.to_string()).collect();
    let mut longest_line_length_lol = 0;
    lines
        .iter()
        .for_each(|l| longest_line_length_lol = l.len().max(longest_line_length_lol));
    lines
        .iter_mut()
        .for_each(|l| (0..longest_line_length_lol - l.len()).for_each(|_| *l += " "));
    lines.join("\n")
}
