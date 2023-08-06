#[link(name = "termutils", kind = "static")]
extern "C" {
     pub fn clear_screen_with_width() -> i32;
     pub fn clear_screen_with_height() -> i32;
     pub fn enter_alt_screen();
     pub fn exit_alt_screen();
}
