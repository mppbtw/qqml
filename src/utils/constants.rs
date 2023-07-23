use lazy_static::lazy_static;
use shellexpand::tilde;

lazy_static! {
    pub static ref YARR_DIR: String = format!("{}/.yarr", tilde("~").clone().to_string());
    pub static ref YARR_SECTIONS_DIR: String =
        format!("{}/.yarr/sections", tilde("~").clone().to_string());
}
