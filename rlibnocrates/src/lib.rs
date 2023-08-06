#[link(name = "nocrates", kind = "static")]
extern "C" {
     pub fn read_single_char() -> u8;
}
