use crate::utils::*;

pub fn init() {
    if path_exists(YARR_DIR.to_owned()) {
        if path_exists(YARR_SECTIONS_DIR.to_owned()) {
            println!("~/.yarr is already set up.");
        }
    }
}
