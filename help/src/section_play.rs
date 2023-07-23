use crate::TOPTEXT;
use lazy_static::lazy_static;

#[rustfmt::skip]
lazy_static! {
    pub static ref HELP: String = {
format!( "{}{}", TOPTEXT,

"Usage: yarr section play [options] <section>

Options:

    [-h | --help]    Show this help menu

    [-t | --test]    Play the section in test mode
                     such that you answers are only
                     checked after completing the 
                     section
")
};
}
