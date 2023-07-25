use std::env;
use super::strip_newline;

static mut ARG_COUNT: usize = 0;

pub fn next_arg() -> Option<String> {
    let args = env::args().collect::<Vec<String>>()[1..].to_vec();
    unsafe {
        if ARG_COUNT == args.len() {
            return None;
        }
        ARG_COUNT += 1;
        let arg = args.get(ARG_COUNT - 1).cloned();
        match arg {
            Some(a) => Some(strip_newline(a)),
            None => None
        }
    }
}
