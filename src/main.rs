mod xob;
mod controls;

use crate::xob::*;
use crate::controls::*;
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

type Title = String;
type BlockIndex = usize;
type TextBuf = Vec<char>;
struct State {
    blocks: Vec<(TextBuf,Point,ColorCode,Title)>,
    paths: Vec<(BlockIndex,BlockIndex)>,
    current_selection: BlockIndex
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
        current_selection: 0
    };
    let block= (vec![],(1,1),'w',"emitest".to_string());
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
            &xob_state,
        );
        buf_render(&mut buffer, &grid, &debug);

        debug.clear();

        stdout.write_all(buffer.as_bytes()).unwrap();
        stdout.flush().unwrap();

        // Write the buffer to stdout all at once

        thread::sleep(Duration::from_millis(10));
        if let Event::Key(event) = read().unwrap() {
            match (event.modifiers, event.code) {
                (_, KeyCode::Right) => {
                    xob_state.blocks[xob_state.current_selection].1.0 += 1;
                }
                (_, KeyCode::Left) => {
                    xob_state.blocks[xob_state.current_selection].1.0 -= 1;
                }
                (_, KeyCode::Up) => {
                    xob_state.blocks[xob_state.current_selection].1.1 -= 1;
                }
                (_, KeyCode::Down) => {
                    xob_state.blocks[xob_state.current_selection].1.1 += 1;
                }
                (KeyModifiers::CONTROL, KeyCode::Char('q')) => break,
                (KeyModifiers::CONTROL, KeyCode::Char('h')) => {
                    control_backspace(&mut xob_state.blocks[xob_state.current_selection].0);
                }
                (_, KeyCode::Char(c)) => xob_state.blocks[xob_state.current_selection].0.push(c),

                (_, KeyCode::Backspace) => {
                    xob_state.blocks[xob_state.current_selection].0.pop();
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

fn select_edit_move(mut grid: Canvas, xob_state: &State) -> Canvas {
    // Draw all blocks
    let (text_buf,position,color,title) = &xob_state.blocks[xob_state.current_selection]; //currentlyy drawing first textbox.
    
    let mut text_buf = text_buf;
    let sample = vec!['-'];
    if text_buf.is_empty() {
        text_buf = &sample;
    }
    let text_box = boxify(&text_buf.iter().collect::<String>(), &Left, Some(title), &Center, None, &Center, None);
    grid.place_block(&text_box, position.0, position.1, *color);
    // Draw all paths
    grid
}

// fn demo(mut grid: Canvas, state: &State) -> Canvas {

//     let input =
//         "hey guys so this is my text input i think this will work well and im  happy about that :)";
//     let input2 = "meowuh meowuh meowuh meowuh meowuh moewuh moewuh en ;)";
//     let input3 = "okay so this ones off to the side :) and i think im goated...";
//     let text_pos = Right;
//     let title = "Tituhl";
//     let title_pos = Left;
//     let footer = None;
//     let footer_pos = Center;
//     let width = 15;
//     let paragraph_a = boxify(
//         input,
//         &text_pos,
//         Some(title),
//         &title_pos,
//         footer,
//         &footer_pos,
//         Some(width),
//     );
//     let paragraph_b = boxify(
//         input2,
//         &text_pos,
//         Some(title),
//         &title_pos,
//         footer,
//         &footer_pos,
//         Some(width),
//     );

//     let paragraph_c = boxify(
//         input3,
//         &text_pos,
//         Some(title),
//         &title_pos,
//         footer,
//         &footer_pos,
//         Some(width),
//     );
//     let (pa_x1, pa_y1) = (4, 5);
//     let (pb_x2, pb_y2) = (25, 3);
//     let (pc_x3, pc_y3) = (45, 4);
//     grid.place_block(&paragraph_a, pa_x1, pa_y1, 'w');
//     grid.place_block(&paragraph_b, pb_x2, pb_y2, 'w');
//     grid.place_block(&paragraph_c, pc_x3, pc_y3, 'w');
//     let start_edges = find_edge(pa_x1, pa_y1, &paragraph_a);
//     let end_edges = find_edge(pb_x2, pb_y2, &paragraph_b);
//     let alt_edge = find_nearest_edge(end_edges[3].0, end_edges[3].1, pc_y3, pc_x3, &paragraph_c);

//     let mut colors = ['r', 'g', 'b', 'y', 'm', 'c'];
//     colors.shuffle(&mut rng());

//     grid.place_path(end_edges[3], alt_edge, colors[5]);

//     for (index, start) in start_edges.iter().enumerate() {
//         let goal = end_edges.choose(&mut rng()).unwrap();
//         let color = colors[index];
//         grid.place_path(*start, *goal, color);
//     }
//     grid.place_point(2, 2, 'X', 'r');

//     let mut text = state.text_buf.iter().collect::<String>();
//     if text.is_empty() {
//         text = "-".to_string();
//     }

//     let text_box = boxify(&text, &Center, Some("emi"), &Left, None, &Left, None);
//     let (text_box_x, text_box_y) = (state.text_buf_x, state.text_buf_y);
//     grid.place_block(&text_box, text_box_x, text_box_y, 'w');


//     let text_box_edge = find_edge(text_box_x, text_box_y, &text_box);
//     let bottom_alt_edge = find_edge(pc_x3, pc_y3, &paragraph_c);
//     grid.place_path(text_box_edge[2], bottom_alt_edge[3], colors[4]);


//     grid.place_border();
//     grid
// }
