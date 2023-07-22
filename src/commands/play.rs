use crate::utils::{self, print_error};

pub fn play() {
    if !utils::dotfile_is_valid() {
        print_error("The ~/.yarr/ directory is not set up; please run yarr init to create it.");
        return;
    }
    println!("Play subcommand");
}
