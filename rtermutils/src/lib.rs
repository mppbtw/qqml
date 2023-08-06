#[link(name = "termutils", kind = "static")]
extern "C" {
    pub fn get_cursor_lines() -> i32;
    pub fn get_cursor_cols() -> i32;
    pub fn clear_screen_with_height() -> i32;
    pub fn clear_screen_with_width() -> i32;
}
