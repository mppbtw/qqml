use std::env;

pub static mut ARG_COUNT: usize = 0;

pub fn next_arg() -> Option<String> {
    let args = env::args().collect::<Vec<String>>()[1..].to_vec();
    unsafe {
        if ARG_COUNT == args.len() {
            return None;
        }
        ARG_COUNT += 1;
        args.into_iter()
            .collect::<Vec<String>>()
            .get(ARG_COUNT - 1)
            .cloned()
    }
}
