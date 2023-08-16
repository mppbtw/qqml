use crate::render::pad_to_width;
use crate::state::State;
use rtermutils::*;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub fn end_screen(s: &State) {
    let width = unsafe { clear_screen_with_width() };
    let bart = make_lines_same_len(
        "
  |\\/\\/\\/|
  |      |
  | (o)(o)
  C      _)   unnaceptable
  |  ___|
  |   /
 /____\\
/      \\"
            .to_owned(),
    )
    .lines()
    .map(|l| pad_to_width(l, width.try_into().unwrap()).unwrap())
    .collect::<Vec<String>>()
    .join("\n");
    ascii_scroll(bart, 200);
    println!("you got {}/{}", s.achieved_marks(), s.get_max_marks());
    unsafe { read_single_char() };
}

fn ascii_scroll(art: String, time_per_line: u64) {
    let mut art_top_line_position = -(art.lines().count() as i32);
    loop {
        let height = unsafe { clear_screen_with_height() };
        if height == 0 {
            continue;
        }
        let art_height = art.lines().count() as i32;
        if art_top_line_position == height {
            break;
        }
        if art_top_line_position >= 0 {
            if (art_top_line_position + art_height) >= height {
                let visible_height = height - art_top_line_position;

                let visible_art = &art
                    .split("\n")
                    .map(|s| s.to_string())
                    .collect::<Vec<String>>()[0..visible_height as usize]
                    .join("\n");

                (0..(height - visible_height)).for_each(|_| println!());
                print!("{}", visible_art);
                stdout().flush().unwrap();
            } else {
                (0..art_top_line_position).for_each(|_| println!());
                print!("{}", art);
                stdout().flush().unwrap();
            }
        } else if art_top_line_position < 0 {
            print!("{}", art);
            stdout().flush().unwrap();
            (0..height - (art_height + art_top_line_position)).for_each(|_| println!());
        }
        art_top_line_position += 1;
        sleep(Duration::from_millis(time_per_line));
    }
    unsafe { clear_screen() }
}

fn make_lines_same_len(s: String) -> String {
    let mut lines: Vec<String> = s.lines().map(|s| s.to_string()).collect();
    let mut longest_line_length_lol = 0;
    lines.iter().for_each(|l| longest_line_length_lol = l.len().max(longest_line_length_lol));
    lines
        .iter_mut()
        .for_each(|l| (0..longest_line_length_lol - l.len()).for_each(|_| *l += " "));
    lines.join("\n")
}
