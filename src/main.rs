mod xob;

use crate::xob::*;
use TextPosition::*;
use rand::{
    rng,
    seq::{IndexedRandom, SliceRandom},
};

fn main() {
    let (w, h) = term_size::dimensions().unwrap();
    let grid = create_grid(w, h - 5);
    // let mut stdout = stdout();

    // Enable raw mode and hide cursor
    let mut grid = demo(Canvas { grid });
    let mut buf = String::new();
    grid.render(&mut buf);
    // println!("{:?}",grid.grid);
    for line in grid.grid {
        for (ch, color_code) in line {
            let color = colorize(color_code);
            print!("{color}{ch}")
        }
        println!()
    }
    // Cleanup: restore terminal state
}
fn demo(mut grid: Canvas) -> Canvas {
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
        grid.place_point(start.1, start.0, '+', color);
        grid.place_point(goal.1, goal.0, '+', color);
    }
    grid.place_border();
    grid
}
