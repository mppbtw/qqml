use std::process::exit;
use rtermutils::*;

pub fn cleanup_and_exit(exit_print: Option<String>) -> ! {
    unsafe {
        exit_alt_screen();
        show_cursor();
    }
    if let Some(msg) = exit_print {
        println!("{}", msg);
    }
    exit(0)
}
