use crate::TOPTEXT;
use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref HELP: String = {
format!( "{}{}", TOPTEXT,

"Usage: yarr section <command> <args>

Options:

    [-h | --help]    Show this help menu

Commands:

    play    Play a certian section

    list    List all installed sections

    create  Create a new section from a QQML file
")
};
}
