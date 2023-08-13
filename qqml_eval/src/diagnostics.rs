use qqml_parser::Error;

const ANSI_RESET: &str = "\x1b[0m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_CYAN: &str = "\x1b[38;5;247m";
const ANSI_RED: &str = "\x1b[31m";

pub fn render_error(input: String, e: Error, path_to_source: Option<String>) -> String {
    let mut output = String::new();
    let mut dat = e.get_token_data();

    output += &format!("{}{}ERROR:{} {}\n", ANSI_RED, ANSI_BOLD, ANSI_RESET, e);

    // We should display the previous line instead of the actual EOF
    if e.is_eof() {
        'outer: loop {
            let l: String;
            if dat.line == input.lines().collect::<Vec<_>>().len() {
                l = input.lines().collect::<Vec<_>>().last().unwrap().to_string();
            } else {
                l = input.lines().collect::<Vec<_>>()[dat.line].to_owned();
            }
            for ch in l.chars() {
                if ch.is_ascii_alphanumeric() {
                    dat.col = l.len() - 1;
                    println!("awkdj");
                    break 'outer;
                }
            }
            dat.line -= 1;
            println!("awd");
        }
    }
    println!("line: {}, col: {}", dat.line, dat.col);

    match path_to_source {
        Some(p) => {
            output += &format!(
                " {}{}-->{} {}:{}:{}\n",
                ANSI_CYAN,
                ANSI_BOLD,
                ANSI_RESET,
                p,
                dat.line + 1,
                dat.col + 1
            )
        }
        None => {
            output += &format!(
                " {}{}-->{} {}:{}\n",
                ANSI_BOLD,
                ANSI_CYAN,
                ANSI_RESET,
                dat.line + 1,
                dat.col + 1
            )
        }
    };

    let line_number_width = format!("{}", dat.line + 1).len();
    let mut current_line = input.lines().collect::<Vec<_>>()[dat.line].to_owned();
    let width = current_line.len();
    remove_extra_spaces(&mut current_line);

    output += &format!(
        "{}{} {}|{}\n",
        padding(line_number_width),
        ANSI_BOLD,
        ANSI_CYAN,
        ANSI_RESET,
    );

    output += &format!(
        "{}{}{} |{} {}\n",
        ANSI_CYAN,
        ANSI_BOLD,
        dat.line + 1,
        ANSI_RESET,
        current_line,
    );

    output += &format!(
        "{}{} {}|{}{}{}{}^{}",
        padding(line_number_width),
        ANSI_BOLD,
        ANSI_CYAN,
        ANSI_RESET,
        padding((dat.col + 1) - (width - current_line.len())),
        ANSI_RED,
        ANSI_BOLD,
        ANSI_RESET,
    );

    output
}

fn padding(len: usize) -> String {
    let mut output = String::new();

    for _ in 0..len {
        output += " ";
    }

    output
}

fn remove_extra_spaces(s: &mut String) {
    loop {
        if s.starts_with(' ') {
            *s = s[1..].to_owned();
        } else {
            break;
        }
    }
    loop {
        if s.ends_with(' ') {
            *s = s[..(s.len() - 1)].to_owned();
        } else {
            break;
        }
    }
}
