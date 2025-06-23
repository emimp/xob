mod xob;

use crate::xob::*;
use TextPosition::*;
use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use rand::{
    rng,
    seq::{IndexedRandom, SliceRandom},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

struct State {
    text_buf: Vec<char>,
    text_buf_x: usize,
    text_buf_y: usize,
}
fn main() {
    enable_raw_mode().unwrap(); // Enter raw mode (no Enter needed)
    let mut stdout = io::stdout();
    // Hide the cursor
    print!("\x1b[?25l");

    // Clear the screen and move cursor to top-left
    print!("\x1b[2J\x1b[H");
    stdout.flush().unwrap();

    let terminal_size = terminal::window_size().unwrap();
    let width = terminal_size.columns as usize;
    let height = terminal_size.rows as usize;

    // let mut text_buf: Vec<char> = vec![];
    let mut debug: Vec<String> = vec![];

    let mut state = State {
        text_buf: vec![],
        text_buf_x: 11,
        text_buf_y: 18,
    };
    loop {
        let mut buffer = String::new();
        debug.push("keep clippy happy".to_string());
        // Move cursor to top-left each frame (prevents scrolling)
        buffer.push_str("\x1b[2J\x1b[H");

        let grid = demo(
            Canvas {
                grid: create_grid(width, height),
            },
            &state,
        );
        buf_render(&mut buffer, &grid, &debug);

        debug.clear();

        // buffer.push_str(&format!("\x1b[{};{}f", 12,6)); //where to place text buffer
        // buffer.push_str(&text_buf.iter().collect::<String>());

        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();

        // Write the buffer to stdout all at once
        thread::sleep(Duration::from_millis(10));
        if let Event::Key(event) = read().unwrap() {
            match (event.modifiers, event.code) {
                (_, KeyCode::Right) => {
                    state.text_buf_x += 1;
                }
                (_, KeyCode::Left) => {
                    state.text_buf_x -= 1;
                }
                (_, KeyCode::Up) => {
                    state.text_buf_y -= 1;
                }
                (_, KeyCode::Down) => {
                    state.text_buf_y += 1;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                    control_backspace(&mut state.text_buf);
                }
                (_, KeyCode::Char(c)) => state.text_buf.push(c),

                (_, KeyCode::Backspace) => {
                    state.text_buf.pop();
                }
                _ => {}
            };
            debug.push(format!(
                "Key: mod={:?}, code={:?}",
                event.modifiers, event.code
            ));
        }
    }
    // let grid = create_grid(width, height);
    print!("\x1b[?25h");

    disable_raw_mode().unwrap(); // Restore normal terminal mode
}

fn buf_render(buffer: &mut String, grid: &Canvas, debug: &Vec<String>) {
    for (index, line) in grid.grid.iter().enumerate() {
        for (ch, color_code) in line {
            let color = colorize(*color_code);
            buffer.push_str(&format!("{color}{ch}"))
        }
        buffer.push_str(&format!("\x1b[{};0f", index + 2));
    }
    let height = grid.grid.len();
    for (index, item) in debug.iter().enumerate() {
        buffer.push_str(&format!("\x1b[{};0f", height - index));
        buffer.push_str(&format!("DEBUG: {item}"));
    }
}

fn control_backspace(text_buf: &mut Vec<char>) {
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
fn demo(mut grid: Canvas, state: &State) -> Canvas {
    //~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

    let input =
        "hey guys so this is my text input i think this will work well and im  happy about that :)";
    let input2 = "meowuh meowuh meowuh meowuh meowuh moewuh moewuh en ;)";
    let input3 = "okay so this ones off to the side :) and i think im goated...";
    let text_pos = Right;
    let title = "Tituhl";
    let title_pos = Left;
    let footer = None;
    let footer_pos = Center;
    let width = 15;
    let paragraph_a = boxify(
        input,
        &text_pos,
        Some(title),
        &title_pos,
        footer,
        &footer_pos,
        Some(width),
    );
    let paragraph_b = boxify(
        input2,
        &text_pos,
        Some(title),
        &title_pos,
        footer,
        &footer_pos,
        Some(width),
    );

    let paragraph_c = boxify(
        input3,
        &text_pos,
        Some(title),
        &title_pos,
        footer,
        &footer_pos,
        Some(width),
    );
    let (pa_x1, pa_y1) = (4, 5);
    let (pb_x2, pb_y2) = (25, 3);
    let (pc_x3, pc_y3) = (45, 4);
    grid.place_block(&paragraph_a, pa_x1, pa_y1, 'w');
    grid.place_block(&paragraph_b, pb_x2, pb_y2, 'w');
    grid.place_block(&paragraph_c, pc_x3, pc_y3, 'w');
    let start_edges = find_edge(pa_x1, pa_y1, &paragraph_a);
    let end_edges = find_edge(pb_x2, pb_y2, &paragraph_b);
    let alt_edge = find_nearest_edge(end_edges[3].0, end_edges[3].1, pc_y3, pc_x3, &paragraph_c);

    let mut colors = ['r', 'g', 'b', 'y', 'm', 'c'];
    colors.shuffle(&mut rng());

    grid.place_path(end_edges[3], alt_edge, colors[5]);

    for (index, start) in start_edges.iter().enumerate() {
        let goal = end_edges.choose(&mut rng()).unwrap();
        let color = colors[index];
        grid.place_path(*start, *goal, color);
    }
    grid.place_point(2, 2, 'X', 'r');

    let mut text = state.text_buf.iter().collect::<String>();
    if text.is_empty() {
        text = "-".to_string();
    }

    let text_box = boxify(&text, &Center, Some("emi"), &Left, None, &Left, None);
    let (text_box_x, text_box_y) = (state.text_buf_x, state.text_buf_y);
    grid.place_block(&text_box, text_box_x, text_box_y, 'w');


    let text_box_edge = find_edge(text_box_x, text_box_y, &text_box);
    let bottom_alt_edge = find_edge(pc_x3, pc_y3, &paragraph_c);
    grid.place_path(text_box_edge[2], bottom_alt_edge[3], colors[4]);


    grid.place_border();
    grid
}
