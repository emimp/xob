mod xob;

use crate::xob::*;
use ::xob::{
    control_backspace, debug_init, debug_read, debug_write, setup_panic_hook, subtract_wrap,
};
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

fn main() {
    setup_panic_hook();
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
    debug_init();
    let mut xob_state = State {
        blocks: vec![(vec![], (1, 1), 'w', "".to_string())],
        paths: vec![],
        current_selection: 0,
        block_width: 0,
        block_height: 0,
        grid_height: height,
        grid_width: width,
    };
    loop {
        debug_write("keep clippy happy");

        let mut buffer = String::new();

        // Move cursor to top-left each frame (prevents scrolling)
        buffer.push_str("\x1b[2J\x1b[H");

        let grid = render_xob(
            Canvas {
                grid: create_grid(width, height),
            },
            &mut xob_state,
        );

        buf_render(&mut buffer, &grid);

        debug.clear();

        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();

        // Write the buffer to stdout all at once
        thread::sleep(Duration::from_millis(10));
        if let Event::Key(event) = read().unwrap() {
            // let blocks = &xob_state.blocks;
            let block = &mut xob_state.blocks[xob_state.current_selection];
            debug_write(block.2);
            // let current_selection_index = xob_state.current_selection.clone();
            let pos = &mut block.1;

            match (event.modifiers, event.code) {
                //Selection
                (KeyModifiers::ALT, KeyCode::Right) | (KeyModifiers::ALT, KeyCode::Down) => {
                    xob_state.current_selection =
                        (xob_state.current_selection + 1) % xob_state.blocks.len();
                }
                (KeyModifiers::ALT, KeyCode::Left) | (KeyModifiers::ALT, KeyCode::Up) => {
                    xob_state.current_selection =
                        subtract_wrap(xob_state.current_selection, xob_state.blocks.len());
                }
                (KeyModifiers::SHIFT, KeyCode::Right) => {}
                (KeyModifiers::SHIFT, KeyCode::Left) => {}
                (KeyModifiers::SHIFT, KeyCode::Up) => {}
                (KeyModifiers::SHIFT, KeyCode::Down) => {}
                //  MOVEMENT
                (KeyModifiers::CONTROL, KeyCode::Right) => {
                    pos.0 = (pos.0 + 5).min(width - xob_state.block_width - 3);
                }
                (KeyModifiers::CONTROL, KeyCode::Left) => {
                    pos.0 = pos.0.saturating_sub(5);
                }
                (KeyModifiers::CONTROL, KeyCode::Up) => {
                    pos.1 = pos.1.saturating_sub(5);
                }
                (KeyModifiers::CONTROL, KeyCode::Down) => {
                    pos.1 = (pos.1 + 5).min(height - xob_state.block_height - 3);
                }
                (_, KeyCode::Right) if pos.0 + 1 < width - xob_state.block_width - 2 => {
                    pos.0 += 1;
                }
                (_, KeyCode::Left) if pos.0 > 0 => {
                    pos.0 -= 1;
                }
                (_, KeyCode::Up) if pos.1 > 0 => {
                    pos.1 -= 1;
                }
                (_, KeyCode::Down) if pos.1 + 1 < height - xob_state.block_height - 2 => {
                    pos.1 += 1;
                }
                // Other controls
                (KeyModifiers::CONTROL, KeyCode::Char('n')) => {
                    block.2 = 'w'; //set old box to white
                    xob_state
                        .blocks
                        .push(("X".chars().collect(), (0, 0), 'w', "".to_string()));
                    xob_state.current_selection = xob_state.blocks.len() - 1
                }
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (KeyModifiers::CONTROL, KeyCode::Char('p')) => panic!("panick test"),
                (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                    control_backspace(&mut block.0);
                }
                (_, KeyCode::Char(c)) => block.0.push(c),
                (_, KeyCode::Enter) => {
                    block.0.push('\n');
                    block.0.push(' ')
                }
                (_, KeyCode::Backspace) => {
                    block.0.pop();
                }
                _ => {}
            }

            debug_write(format!(
                "Key: mod={:?}, code={:?}",
                event.modifiers, event.code
            ));
        }
    }
    // let grid = create_grid(width, height);
    println!("\x1b[?25h");
    disable_raw_mode().unwrap(); // Restore normal terminal mode
}

fn buf_render(buffer: &mut String, grid: &Canvas) {
    for (index, line) in grid.grid.iter().enumerate() {
        for (ch, color_code) in line {
            let color = colorize(*color_code);
            buffer.push_str(&format!("{color}{ch}"))
        }
        buffer.push_str(&format!("\x1b[{};0f", index + 2));
    }
}

fn render_xob(mut grid: Canvas, state: &State) -> Canvas {
    grid.place_border();

    let current_selection = state.current_selection;
    let mut index = 0;
    for (block, (x, y), color_code, title) in &state.blocks {
        debug_write(((x, y), color_code, title));
        let mut input = block.iter().collect::<String>();
        if input.is_empty() {
            input = "X".to_string()
        }
        let text_box = boxify(&input, None, Some(title), None, None, None, None);
        let mut color_code = color_code;
        if current_selection == index {
            color_code = &'g'
        }
        grid.place_block(&text_box, *x, *y, *color_code);
        index += 1;
    }
    for (index, item) in debug_read().iter().enumerate() {
        grid.place_block(item, 0, grid.grid.len() - index, 'r');
    }
    grid
}
