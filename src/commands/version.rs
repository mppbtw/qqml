use crate::utils::print_error;

pub fn version() {
    match option_env!("CARGO_PKG_VERSION") {
        Some(v) => println!("yarr {}", v),
        None => print_error("Failed to get version metadata!"),
    }
}
