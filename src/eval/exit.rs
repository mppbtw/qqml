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
