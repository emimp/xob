mod controls;
mod xob;

use crate::controls::*;
use crate::xob::*;
use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

type Title = String;
type BlockIndex = usize;
type TextBuf = Vec<char>;

#[allow(dead_code)]
struct State {
    blocks: Vec<(TextBuf, Point, ColorCode, Title)>,
    paths: Vec<(BlockIndex, BlockIndex)>,
    current_selection: BlockIndex,
    block_width: usize,
    block_height: usize,
    grid_width: usize,
    grid_height: usize,
}
impl State {
    fn new_block(&mut self, text: &str, color: ColorCode, title: &str) {
        let text_buf = text.chars().collect();
        self.blocks
            .push((text_buf, (0, 0), color, title.to_string()))
    }
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

    let mut xob_state = State {
        blocks: vec![],
        paths: vec![],
        current_selection: 0,
        block_width: 0,
        block_height: 0,
        grid_height: height,
        grid_width: width,
    };
    let block = (vec![], (1, 1), 'w', "emitest".to_string());
    xob_state.blocks.push(block);
    loop {
        let mut buffer = String::new();
        debug.push("keep clippy happy".to_string());
        // Move cursor to top-left each frame (prevents scrolling)
        buffer.push_str("\x1b[2J\x1b[H");

        let grid = select_edit_move(
            Canvas {
                grid: create_grid(width, height),
            },
            &mut xob_state,
        );
        buf_render(&mut buffer, &grid, &debug);

        debug.clear();

        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();

        // Write the buffer to stdout all at once

        thread::sleep(Duration::from_millis(10));
        if let Event::Key(event) = read().unwrap() {
            let block = &mut xob_state.blocks[xob_state.current_selection];
            let pos = &mut block.1;

            match (event.modifiers, event.code) {
                (KeyModifiers::CONTROL, KeyCode::Right) => {
                    pos.0 = (pos.0 + 5).min(width - xob_state.block_width - 2);
                }
                (KeyModifiers::CONTROL, KeyCode::Left) => {
                    pos.0 = pos.0.saturating_sub(5);
                }
                (KeyModifiers::CONTROL, KeyCode::Up) => {
                    pos.1 = pos.1.saturating_sub(5);
                }
                (KeyModifiers::CONTROL, KeyCode::Down) => {
                    pos.1 = (pos.1 + 5).min(height - xob_state.block_height);
                }
                (_, KeyCode::Right) if pos.0 + 1 < width - xob_state.block_width - 1 => {
                    pos.0 += 1;
                }
                (_, KeyCode::Left) if pos.0 > 0 => {
                    pos.0 -= 1;
                }
                (_, KeyCode::Up) if pos.1 > 0 => {
                    pos.1 -= 1;
                }
                (_, KeyCode::Down) if pos.1 + 1 < height - xob_state.block_height + 1 => {
                    pos.1 += 1;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                    control_backspace(&mut block.0);
                }
                (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                    xob_state.new_block("Hey guys whats up!\nSO essentially this is a popup window menu :P and im goated..\n anyways bruhs DIE", 'H', "New window!");
                    xob_state.current_selection = xob_state.blocks.len() - 1
                }
                (_, KeyCode::Char(c)) => block.0.push(c),
                (_, KeyCode::Backspace) => {
                    block.0.pop();
                }
                _ => {}
            }

            debug.push(format!(
                "Key: mod={:?}, code={:?}",
                event.modifiers, event.code
            ));
        }
    }
    // let grid = create_grid(width, height);

    disable_raw_mode().unwrap(); // Restore normal terminal mode
}

fn buf_render(buffer: &mut String, grid: &Canvas, debug: &[String]) {
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

fn select_edit_move(mut grid: Canvas, xob_state: &mut State) -> Canvas {
    // Draw all blocks
    for block in xob_state.blocks.clone() {
        let (text_buf, position, color, title) = block; //currentlyy drawing first textbox.

        let mut text_buf = text_buf;
        let sample = vec!['-'];
        if text_buf.is_empty() {
            text_buf = sample;
        }
        let (text_box, box_width, box_height) = boxify(
            &text_buf.iter().collect::<String>(),
            &TextPosition::Left,
            Some(&title),
            &TextPosition::Right,
            None,
            &TextPosition::Center,
            None,
        );
        xob_state.block_width = box_width;
        xob_state.block_height = box_height;
        grid.place_block(&text_box, position.0, position.1, color);
    }
    grid
}
