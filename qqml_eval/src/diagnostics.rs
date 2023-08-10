use qqml_parser::Error;

const ANSI_RESET: &'static str = "\x1b[0m";
const ANSI_BG_WHITE: &'static str = "\x1b[47m";
const ANSI_BLACK: &'static str = "\x1b[1;30m";
const ANSI_GREEN: &'static str = "\x1b[32m";
const ANSI_YELLOW: &'static str = "\x1b[0;33m";
const ANSI_RED: &'static str = "\x1b[31m";

pub fn render_error(input: String, e: Error, path_to_source: Option<String>) -> String {
    let mut output = String::new();
    let dat = e.get_token_data();

    output += &format!("{}error:{} {}\n", ANSI_RED, ANSI_RESET, e.to_string());

    match path_to_source {
        Some(p) => output += &format!("-->{}:{}:{}\n", p, dat.line + 1, dat.col + 1),
        None => output += &format!("-->{}:{}\n", dat.line + 1, dat.col + 1),
    };
    output += "  | \n";

    output += &format!("{} | {}\n", dat.line + 1, input.lines().collect::<Vec<_>>()[dat.line]);

    output += &format!("  | {}{}^{}\n",ANSI_RED, padding(dat.col), ANSI_RESET);

    output
}

fn padding(len: usize) -> String {
    let mut output = String::new();

    for _ in 0..len {
        output += " ";
    }

    output
}
