#[link(name = "nocrates", kind = "static")]
extern "C" {
     pub fn read_single_char() -> u8;
     pub fn switch_to_alt_screen();
     pub fn return_from_alt_screen();
     pub fn clear_screen_with_width() -> i32;
}
