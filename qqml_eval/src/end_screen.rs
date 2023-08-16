use crate::state::State;
use rtermutils::*;
use std::io::stdout;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub fn end_screen(s: &State) {
    unsafe { clear_screen() }
    let bart = "
                                     |\\/\\/\\/|
                                     |      |
                                     | (o)(o)
                                     C      _)   good job
                                      |  ___|
                                      |   /
                                     /____\\
                                    /      \\"
        .to_owned();
    ascii_scroll(bart, 100);
    unsafe { read_single_char() };
}

fn ascii_scroll(art: String, time_per_line: u64) {

    let mut art_top_line_position = -(art.lines().count() as i64);
    loop {
        unsafe { clear_screen() }
        let height = unsafe { clear_screen_with_height() } as i64;
        let art_height = art.lines().count() as i64;
        if art_top_line_position == height {
            return;
        }
        if art_top_line_position >= 0 {
            if (art_top_line_position + art_height) >= height {
                let visible_height = height - art_top_line_position;

                // If the user starts spamming keys here, visible_height might be a really big
                // number for some reason, need to do some bounds checking
                if art
                    .split("\n")
                        .map(|s| s.to_string())
                        .collect::<Vec<String>>()
                        .len() as i64
                        <= visible_height
                        {
                            let visible_art = &art
                                .split("\n")
                                .map(|s| s.to_string())
                                .collect::<Vec<String>>()[0..visible_height as usize]
                                .join("\n");

                            (0..(height - visible_height)).for_each(|_| println!());
                            print!("{}", visible_art);
                            stdout().flush().unwrap();
                        } else {
                            return;
                        }
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
}
