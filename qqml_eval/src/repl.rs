use crate::end_screen::end_screen;
use crate::exit::cleanup_and_exit;
use crate::render::*;
use crate::state::*;
use qqml_parser::parse;
use qqml_parser::Question;
use rtermutils::*;
use std::process::exit;

pub fn run(input: &String, path_to_source: Option<&String>, log_path: Option<&String>) -> ! {
    let parsed = match parse(input) {
        Ok(p) => p,
        Err(_) => {
            // Proper error reporting should be done by the caller
            exit(1);
        }
    };
    let mut s = {
        StateConstructor {
            path_to_source: path_to_source.cloned(),
            questions: parsed.questions,
            max_hints: parsed.max_hints,
        }
        .construct()
    };
    unsafe {
        enter_alt_screen();
        hide_cursor();
    }
    let mut refresh_needed = false;
    println!("{}", s.create_screen().render());
    loop {
        if refresh_needed {
            println!("{}", s.create_screen().render());
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
    cleanup_and_exit(Some(format!("{{\"finished\": false, \"file_data\": {}}}", s.to_json())), log_path.cloned());
}

fn help_menu() {
    unsafe {
        clear_screen();
        println!("{}", include_str!("./help_menu.txt"));
        read_single_char();
    }
}
