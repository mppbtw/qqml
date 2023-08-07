use crate::render::*;
use crate::state::*;
use qqml_parser::Question;
use qqml_parser::parse;
use rtermutils::*;
use std::process::exit;

pub fn run(input: String, path_to_source: Option<String>) -> ! {
    let parsed = parse(input).unwrap();
    let mut s = {
        StateConstructor {
            path_to_source,
            questions: parsed.questions,
            max_hints: parsed.max_hints,
        }
        .construct()
    };
    unsafe {
        enter_alt_screen();
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
            b'n' => {
                if s.current_question_index + 1 != s.questions.len() {
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
            b'r' => (), // Refresh the page
            _ => refresh_needed = false,
        }
    }
    unsafe {
        exit_alt_screen();
    }

    exit(0)
}
