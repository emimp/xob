use std::fs::{self, File};
use std::io::{self, Write};
use std::panic;

use crossterm::terminal::{self, disable_raw_mode};

pub fn setup_panic_hook() {
    panic::set_hook(Box::new(|info| {
        let mut stdout = io::stdout();

        disable_raw_mode().unwrap();

        let (_cols, rows) = terminal::size().unwrap_or((80, 24));
        // - \x1b[{rows};1H → Move cursor to bottom-left
        // - \x1b[?25h → Show cursor
        let _ = write!(stdout, "\x1b[{};1H\x1b[?25h", rows);
        let _ = stdout.flush();

        // Print panic message
        eprintln!("Panic!!!: {}", info);
        std::process::exit(1);
    }));
}

pub fn control_backspace(text_buf: &mut Vec<char>) {
    // This is Control backspace functionality
    if !text_buf.contains(&' ') {
        text_buf.clear();
    } else {
        // Checks and removes all trailing spaces & punctuation
        let last_pos = text_buf
            .iter()
            .rposition(|&c| [' ', '.', '?', '!'].contains(&c)); // find last position of repeating char
        if let Some(last_pos) = last_pos {
            text_buf.drain(last_pos + 1..text_buf.len());
        }

        // Delete the entie word until previous space
        if let Some(last_ch) = text_buf.last() {
            let mut last_non_space_index = None;
            for (index, ch) in text_buf.iter().rev().enumerate() {
                if ch != last_ch {
                    last_non_space_index = Some(index);

                    break; // Stop at the first non-space character found from the end
                }
            }
            if let Some(last_non_space_index) = last_non_space_index {
                if last_non_space_index != 0 {
                    let to = text_buf.len() - last_non_space_index;
                    text_buf.drain(to..text_buf.len());
                };
            }
        }
    }
}

pub fn debug_init() {
    File::create("target/debug/debug.xob").expect("should be able to create debug.xob");
}

pub fn debug_write<T: std::fmt::Debug>(input: T) {
    let text = format!("DEBUG: {:?}\n", input);
    fs::write("target/debug/debug.xob", text).expect("Should be able to write to `debug.xob`");
}

pub fn debug_read() -> Vec<String> {
    let text = fs::read_to_string("target/debug/debug.xob")
        .expect("Should be able to read debug.xob file")
        .lines()
        .map(|l| l.to_string())
        .collect::<Vec<String>>();
    File::create("target/debug/debug.xob").expect("should be able to create debug.xob");
    text
}

pub fn subtract_wrap(index: usize, vec_len: usize) -> usize {
    if vec_len == 0 {
        panic!("Vector length cannot be zero");
    }
    if index == 0 { vec_len - 1 } else { index - 1 }
}
