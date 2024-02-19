//  QQML or the Quiz Question Markup Language.
//  Copyright (C) 2024 'mppbtw'
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program. If not, see <https://www.gnu.org/licenses/>.

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
