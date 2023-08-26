#[link(name = "termutils", kind = "static")]
extern "C" {
    pub fn get_cursor_lines() -> i32;
    pub fn get_cursor_cols() -> i32;
    pub fn clear_screen_with_height() -> i32;
    pub fn clear_screen_with_width() -> i32;

    // By 'break cursor' I mean that the cursor will be in some arbitrary place
    // after calling the function. This is OK because the we might need to do
    // more computation before clearing the screen to prevent flickering.
    pub fn break_cursor_with_height() -> i32;

    // By 'break cursor' I mean that the cursor will be in some arbitrary place
    // after calling the function. This is OK because the we might need to do
    // more computation before clearing the screen to prevent flickering.
    pub fn break_cursor_with_width() -> i32;
    pub fn read_single_char() -> u8;
    pub fn enter_alt_screen();
    pub fn exit_alt_screen();
    pub fn hide_cursor();
    pub fn show_cursor();
    pub fn clear_screen();
    pub fn close_stdin();
}
