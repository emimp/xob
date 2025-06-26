mod xob;

use crate::xob::*;
use ::xob::{control_backspace, debug_init, debug_read};
use crossterm::{
    event::{Event, KeyCode, KeyModifiers, read},
    terminal::{self, disable_raw_mode, enable_raw_mode},
};
use rand::rng;
use rand::seq::IndexedRandom;
use rand::seq::SliceRandom;
use std::{io::{self, Write}};
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
        debug.push("keep clippy happy".to_string());

        for line in debug_read() {
            debug.push(line);
        }

        let mut buffer = String::new();
        
        
        // Move cursor to top-left each frame (prevents scrolling)
        buffer.push_str("\x1b[2J\x1b[H");

        let grid = demo(
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

fn demo(mut grid: Canvas, state: &State) -> Canvas {
    // Configuration constants
    const WIDTH: Option<usize> = Some(15);
    const TEXT_POS: TextPosition = TextPosition::Right;
    const TITLE: Option<&str> = Some("Tituhl");
    const TITLE_POS: TextPosition = TextPosition::Left;
    const FOOTER: Option<&str> = None;
    const FOOTER_POS: TextPosition = TextPosition::Center;

    // Text inputs
    let inputs = [
        "hey guys so this is my text input i think this will work well and im  happy about that :)",
        "meowuh meowuh meowuh meowuh meowuh moewuh moewuh en ;)",
        "okay so this ones off to the side :) and i think im goated...",
    ];

    // Position coordinates
    let positions = [(4, 5), (25, 3), (45, 4)];

    // Create text boxes
    let paragraphs: Vec<_> = inputs
        .iter()
        .map(|&input| {
            boxify(
                input,
                &TEXT_POS,
                TITLE,
                &TITLE_POS,
                FOOTER,
                &FOOTER_POS,
                WIDTH,
            )
        })
        .collect();

    // Place paragraphs on grid
    for (paragraph, &(x, y)) in paragraphs.iter().zip(&positions) {
        grid.place_block(paragraph, x, y, 'w');
    }

    // Get edges for connections
    let paragraph_edges: Vec<_> = paragraphs
        .iter()
        .zip(&positions)
        .map(|(paragraph, &(x, y))| find_edge(x, y, paragraph))
        .collect();

    // Set up colors
    let mut colors = ['r', 'g', 'b', 'y', 'm', 'c'];
    colors.shuffle(&mut rng());

    // Create specific connections
    let alt_edge = find_nearest_edge(
        paragraph_edges[1][3].0,
        paragraph_edges[1][3].1,
        positions[2].1,
        positions[2].0,
        &paragraphs[2],
    );
    grid.place_path(paragraph_edges[1][3], alt_edge, colors[5]);

    // Create random connections from first to second paragraph
    for (index, &start) in paragraph_edges[0].iter().enumerate() {
        let goal = paragraph_edges[1].choose(&mut rng()).unwrap();
        grid.place_path(start, *goal, colors[index]);
    }

    // Place marker point
    grid.place_point(2, 2, 'X', 'r');

    // Handle state-based text box
    let (text_buf, (position_x, position_y), _, _) = &state.blocks[state.current_selection];
    let text = if text_buf.is_empty() {
        "-".to_string()
    } else {
        text_buf.iter().collect::<String>()
    };

    let text_box = boxify(
        &text,
        &TextPosition::Center,
        Some("emi"),
        &TextPosition::Left,
        None,
        &TextPosition::Left,
        None,
    );

    grid.place_block(&text_box, *position_x, *position_y, 'w');

    // Connect text box to third paragraph
    let text_box_edge = find_edge(*position_x, *position_y, &text_box);
    let bottom_alt_edge = find_edge(positions[2].0, positions[2].1, &paragraphs[2]);
    grid.place_path(text_box_edge[2], bottom_alt_edge[3], colors[4]);

    grid.place_border();
    grid
}
