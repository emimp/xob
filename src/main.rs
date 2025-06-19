mod xob;
use crate::xob::*;
use TextPosition::*;
use rand::{rng, seq::IndexedRandom};
fn main() {
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
    let (w, h) = term_size::dimensions().unwrap(); // println!("w,h {w}, {h}");
    let mut grid = vec![vec![(' ', 'w'); w]; h - 5];
    let (pa_x1, pa_y1) = (4, 10);
    let (pb_x2, pb_y2) = (width + 4 + 5, 3);
    let (pc_x3, pc_y3) = (15,30);
    place_block(&mut grid, &paragraph_a, pa_y1, pa_x1, 'w');
    place_block(&mut grid, &paragraph_b, pb_y2, pb_x2, 'w');
    place_block(&mut grid, &paragraph_c, pc_x3, pc_y3, 'w');
    let start_edges = find_edge(pa_x1, pa_y1, &paragraph_a);
    let end_edges = find_edge(pb_x2, pb_y2, &paragraph_b);
    let alt_edge = find_nearest_edge(end_edges[3].0, end_edges[3].1, pc_x3, pc_y3, &paragraph_c);

    let colors = ['r', 'g', 'b', 'y', 'm', 'c'];
    
    place_path(&mut grid, end_edges[3], alt_edge, 'm');
    
    for (index, start) in start_edges.iter().enumerate() {
        let goal = end_edges.choose(&mut rng()).unwrap();
        let color = colors[index];
        place_path(&mut grid, *start, *goal, color);
        grid[start.1][start.0] = ('+', color);
        grid[goal.1][goal.0] = ('+', color);
    }

    place_border(&mut grid);


    for row in grid {
        for (char, color_code) in row {
            let color = colorize(color_code);
            print!("{color}{char}")
        }
        println!();
    }
}
