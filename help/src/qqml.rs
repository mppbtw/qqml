use crate::TOPTEXT;
use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref HELP: String = {

        format!( "{}{}", TOPTEXT,

"Usage: yarr qqml <command> <file>
Interact with QQML files.

Options:

    [-h | --help]     Show this help menu

Commands:

    check   Ensure validity of the specified QQML file
")
    };
}
