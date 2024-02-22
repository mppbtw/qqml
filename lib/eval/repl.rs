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

use std::convert::Infallible;
use std::io::stdout;
use std::io::Write;

use super::end_screen::end_screen;
use super::exit::cleanup_and_exit;
use super::render::Render;
use crate::parser::core::parse;
use crate::parser::Question;
use crate::termutils::*;
use crate::Error;
use crate::State;

pub fn run_from_state(mut s: State, log_path: Option<&String>) -> ! {
    unsafe {
        enter_alt_screen();
        hide_cursor();
    }
    let mut refresh_needed = false;
    unsafe {
        print!(
            "{}",
            s.create_screen(clear_screen_with_width() as usize).render()
        );
        stdout().flush().unwrap();
    }
    loop {
        if refresh_needed {
            unsafe {
                // We break cursor here to prevent any frame rendering from happening while the
                // screen is cleared, causing a flicker every frame.
                let next_frame = s.create_screen(break_cursor_with_width() as usize).render();
                clear_screen();
                print!("{}", next_frame);
                stdout().flush().unwrap();
            }
        }

        refresh_needed = true;
        match unsafe { read_single_char() } {
            b'q' => break,
            b'?' => help_menu(),
            b'h' => match s.questions[s.current_question_index] {
                Question::Multichoice(ref mut d) => 'block: {
                    if d.is_answered || s.hints_used == s.max_hints || d.hints.len() == d.used_hints
                    {
                        refresh_needed = false;
                        break 'block;
                    }
                    s.hints_used += 1;
                    d.used_hints += 1;
                }
                _ => refresh_needed = false,
            },
            b'\n' => match s.questions[s.current_question_index] {
                Question::Multichoice(ref mut d) => 'block: {
                    if d.is_answered {
                        if s.every_question_answered() {
                            end_screen(&mut s, log_path.cloned());
                        } else {
                            refresh_needed = false;
                        }
                        break 'block;
                    }
                    let mut total_chosen = 0;
                    d.answers.iter().for_each(|a| {
                        if a.is_chosen {
                            total_chosen += 1;
                        }
                    });
                    if total_chosen == d.max_marks {
                        d.is_answered = true;
                    } else {
                        refresh_needed = false;
                    }
                }
                _ => refresh_needed = false,
            },
            b' ' => match s.questions[s.current_question_index] {
                Question::Multichoice(ref mut d) => 'block: {
                    if d.is_answered {
                        refresh_needed = false;
                        break 'block;
                    }
                    let mut total_chosen = 0;
                    d.answers.iter().for_each(|a| {
                        if a.is_chosen {
                            total_chosen += 1;
                        }
                    });

                    if total_chosen == d.max_marks || d.answers[d.selected_answer].is_chosen {
                        d.answers[d.selected_answer].is_chosen = false;
                    } else if !(total_chosen == d.max_marks
                        || d.answers[d.selected_answer].is_chosen)
                    {
                        d.answers[d.selected_answer].is_chosen = true;
                    } else {
                        refresh_needed = false;
                    }
                }
                _ => refresh_needed = true,
            },
            b'n' => {
                if s.current_question_index + 1 != s.questions.len()
                    && match s.questions[s.current_question_index] {
                        Question::Multichoice(ref d) => d.is_answered,
                        _ => true,
                    }
                {
                    s.current_question_index += 1;
                } else {
                    refresh_needed = false;
                }
            }
            b'p' => {
                if s.current_question_index != 0 {
                    s.current_question_index -= 1;
                } else {
                    refresh_needed = false;
                }
            }
            b'k' => match s.questions[s.current_question_index] {
                Question::Multichoice(ref mut d) => {
                    if d.selected_answer != 0 {
                        d.selected_answer -= 1;
                        s.questions[s.current_question_index] = Question::Multichoice(d.clone());
                    }
                }
                _ => refresh_needed = false,
            },
            b'j' => match s.questions[s.current_question_index] {
                Question::Multichoice(ref mut d) => {
                    if d.selected_answer + 1 != d.answers.len() {
                        d.selected_answer += 1;
                        s.questions[s.current_question_index] = Question::Multichoice(d.clone());
                    }
                }
                _ => refresh_needed = false,
            },
            b'r' => (), // Refresh the page
            _ => refresh_needed = false,
        }
    }
    if log_path.is_some() {
        cleanup_and_exit(Some(s.to_json()), log_path.cloned());
    } else {
        cleanup_and_exit(None, log_path.cloned());
    }
}

pub fn run(
    input: &String,
    path_to_source: Option<&String>,
    log_path: Option<&String>,
) -> Result<Infallible, Error> {
    run_from_state(parse(input, path_to_source.cloned())?, log_path)
}

fn help_menu() {
    unsafe {
        clear_screen();
        println!("{}", include_str!("./help_menu.txt"));
        read_single_char();
    }
}
