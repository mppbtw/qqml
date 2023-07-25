use crate::TOPTEXT;
use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref HELP: String = {

        format!( "{}{}", TOPTEXT,

"Usage: yarr <command> <args>

Options:

    [-h | --help]     Show this help menu

    [-v | --version]  Display version information

Commands:

    qqml    Interact with local QQML files
")
    };
}
