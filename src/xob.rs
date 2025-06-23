use pathfinding::prelude::astar;
pub type ColorCode = char;
pub type Grid = Vec<Vec<(char, ColorCode)>>;
pub type Point = (usize, usize);
pub enum TextPosition {
    Left,
    Right,
    Center,
}

pub fn boxify(
    input: &str,
    text_pos: &TextPosition,
    title: Option<&str>,
    title_pos: &TextPosition,
    footer: Option<&str>,
    footer_pos: &TextPosition,
    width: Option<usize>,
) -> String {
    let lines: Vec<String> = input.lines().map(|l| l.to_string()).collect();

    let mut max_width = width.unwrap_or(lines.iter().map(|l| l.len()).max().unwrap_or(0));

    let title_len = title.unwrap_or("").len();
    if title_len > max_width {
        max_width = title_len
    }
    let mut wrapped: Vec<String> = Vec::new();

    for line in lines {
        let mut current = String::new();

        for word in line.split_whitespace() {
            if current.is_empty() {
                current.push_str(word);
            } else if current.len() + 1 + word.len() <= max_width {
                current.push(' ');
                current.push_str(word);
            } else {
                wrapped.push(current);
                current = word.to_string();
            }
        }

        if !current.is_empty() {
            wrapped.push(current);
        }
    }
    let lines = wrapped;

    let format_line = |text: &str, pos: &TextPosition| -> String {
        match pos {
            TextPosition::Left => {
                format!("{text}{}", "─".repeat(max_width.saturating_sub(text.len())))
            }
            TextPosition::Right => format!(
                "{}{}",
                "─".repeat(max_width.saturating_sub(text.len())),
                text
            ),
            TextPosition::Center => {
                let padding = max_width.saturating_sub(text.len());
                let left = padding / 2;
                let right = padding - left;
                format!("{}{}{}", "─".repeat(left), text, "─".repeat(right))
            }
        }
    };

    let format_row = |line: &str| -> String {
        match text_pos {
            TextPosition::Left => format!("│{}{}│", line, " ".repeat(max_width - line.len())),
            TextPosition::Right => format!("│{}{}│", " ".repeat(max_width - line.len()), line),
            TextPosition::Center => {
                let padding = max_width.saturating_sub(line.len());
                let left = padding / 2;
                let right = padding - left;
                format!("│{}{}{}│", " ".repeat(left), line, " ".repeat(right))
            }
        }
    };

    let content = lines
        .iter()
        .map(|line| format_row(line))
        .collect::<Vec<_>>()
        .join("\n");
    let title = title.unwrap_or("");
    let footer = footer.unwrap_or("");
    format!(
        "┌{}┐\n{}\n└{}┘",
        format_line(title, title_pos),
        content,
        format_line(footer, footer_pos)
    )
}

pub struct Canvas {
    pub grid: Grid,
}

pub fn create_grid(width: usize, height: usize) -> Grid {
    vec![vec![(' ', 'w'); width]; height]
}

//TODO Make it so every place function can stack by returning canvas and testing it
//TODO I also need to make it so the demo mode is run with the tests
impl Canvas {
    // pub fn place_connected_blocks(&mut self, blocks: Vec<(&str,Point,ColorCode)>) {

    // }
    pub fn place_block(&mut self, block: &str, start_x: usize, start_y: usize, color: char) {
        let mut y = start_y;

        for line in block.split('\n') {
            if y >= self.grid.len() {
                break;
            }

            for (i, c) in line.chars().enumerate() {
                if start_x + i >= self.grid[y].len() {
                    break;
                }
                if c == ' ' {
                    // Used for paths to avoid space within block.
                    self.grid[y][start_x + i] = (c, 'A');
                } else {
                    self.grid[y][start_x + i] = (c, color);
                }
            }

            y += 1;
        }
    }
    pub fn place_path(&mut self, start: Point, goal: Point, color: char) {
        let result = astar(
            &start,
            |p| neighbors(*p, &self.grid),
            |p| {
                (goal.0 as isize - p.0 as isize).unsigned_abs()
                    + (goal.1 as isize - p.1 as isize).unsigned_abs()
            },
            |p| *p == goal,
        );
        if let Some((path, _cost)) = result {
            for &(x, y) in &path {
                if (x, y) != start && (x, y) != goal {
                    self.grid[y][x] = ('*', 'w');
                }
            }
            self.draw_path(&path, color);
        } //else no path found

        // Check neighbors char to determine what connector to use
        // let (start_x, start_y) = (start_x as isize, start_y as isize);
    }

    pub fn place_border(&mut self) {
        let height = self.grid.len();
        let width = if height > 0 {
            self.grid[0].len()
        } else {
            return;
        };

        // Draw top border (row 0)
        for x in 0..width {
            self.grid[0][x] = (char::from_digit(x as u32 % 10, 10).unwrap(), 'G');
        }

        // Draw left border (column 0)
        for (y, item) in self.grid.iter_mut().enumerate().take(height) {
            item[0] = (char::from_digit(y as u32 % 10, 10).unwrap(), 'G');
        }
    }
    pub fn place_point(&mut self, x: usize, y: usize, ch: char, color: char) {
        self.grid[x][y] = (ch, color);
    }
    fn draw_path(&mut self, path: &[Point], color: char) {
        for position in path.windows(3) {
            let prev = position[0];
            let curr = position[1];
            let next = position[2];

            let prevdiff = (
                ((curr.0 as i32) - (prev.0 as i32)),
                ((curr.1 as i32) - (prev.1 as i32)),
            );
            let nextdiff = (
                ((next.0 as i32) - (curr.0 as i32)),
                ((next.1 as i32) - (curr.1 as i32)),
            );

            let symbol = match (prevdiff, nextdiff) {
                ((0, 1), (0, 1)) | ((0, -1), (0, -1)) => '│',
                ((1, 0), (1, 0)) | ((-1, 0), (-1, 0)) => '─',
                ((0, 1), (1, 0)) | ((-1, 0), (0, -1)) => '└',
                ((1, 0), (0, -1)) | ((0, 1), (-1, 0)) => '┘',
                ((0, -1), (1, 0)) | ((-1, 0), (0, 1)) => '┌',
                ((0, -1), (-1, 0)) | ((1, 0), (0, 1)) => '┐',
                _ => 'X',
            };
            // if symbol == 'X' {
            //     println!("-------\nprev {prev:?}\ncurr {curr:?}\nnext {next:?}");
            //     println!("prevdiff {prevdiff:?}");
            //     println!("nextdiff {nextdiff:?}");
            //     println!("sym {symbol}\n-------");
            // }
            self.grid[curr.1][curr.0] = (symbol, color)
        }

        // println!("PATH: {path:?}");
        // println!("~~~~~~~~~~");

        let (prev_sym_x, prev_sym_y) = path[path.len() - 2];
        let prev_sym = self.grid[prev_sym_y][prev_sym_x].0;
        // println!("PREV: {prev_sym:?} xy {prev_sym_x},{prev_sym_y}");

        let (last_sym_x, last_sym_y) = *path.last().unwrap();
        let last_sym = self.grid[last_sym_y][last_sym_x].0;
        // println!("LAST: {last_sym:?} xy {last_sym_x},{last_sym_y}");

        let direction = (
            last_sym_x as isize - prev_sym_x as isize,
            last_sym_y as isize - prev_sym_y as isize,
        );

        // println!("direction: {direction:?}");
        // println!("({prev_sym:?},{last_sym:?})");

        let connector = match (prev_sym, last_sym, direction) {
            ('└', '─', _) => '─',
            ('─', '│', _) | (_, '┤', _) => '┤',
            ('┐', '│', _) | ('└', '│', _) | ('└', '├', _) | (_, '├', _) | ('┘', '│', (-1, 0)) => {
                '├'
            }
            ('│', '─', (0, -1)) | (_, '┬', (_, _)) | ('┘', '─', (0, -1)) => '┬',
            ('│', '─', (0, 1)) | (_, '┴', _) => '┴',
            ('┘', '│', _) | ('│', '│', _) => '│',
            (_, '─', _) => '─',
            _ => 'X',
        };
        self.grid[last_sym_y][last_sym_x] = (connector, color);
        // println!("CONNECTOR: {connector:?}");
        // println!("~~~~~~~~~~");
    }
    pub fn _buf_string(&mut self, buf: &mut String) {
        for row in &self.grid {
            for (char, color_code) in row {
                let color = colorize(*color_code);
                let content = format!("{color}{char}");
                for ch in content.chars() {
                    buf.push(ch);
                }
            }
        }
    }
    // pub fn clear(&mut self, w: usize, h: usize) {
    //     self.grid = vec![vec![(' ', 'w'); w]; h]
    // }
}

fn neighbors(pos: Point, grid: &Grid) -> Vec<(Point, usize)> {
    let mut result = Vec::new();
    let (x, y) = pos;
    let height = grid.len();
    let width = grid[0].len();

    let deltas = [(0isize, 1), (1, 0), (0, -1), (-1, 0)];
    for (dx, dy) in deltas {
        let nx = x.wrapping_add(dx as usize);
        let ny = y.wrapping_add(dy as usize);
        if nx < width
            && ny < height
            && [' ', '│', '─', '+', '┴', '┬', '├', '┤'].contains(&grid[ny][nx].0)
            && (grid[ny][nx].1 != 'A')
        {
            result.push(((nx, ny), 1));
        }
    }
    result
}

pub fn find_edge(block_x: usize, block_y: usize, block: &str) -> [Point; 4] {
    let block_height = block.lines().count();
    let block_width = block.lines().next().expect("block empty").chars().count();

    let left = (block_x, block_y + block_height / 2);
    let right = (block_x + block_width - 1, block_y + block_height / 2);
    let top = (block_x + block_width / 2, block_y);
    let bottom = (block_x + block_width / 2, block_y + block_height - 1);

    [left, right, top, bottom]
}

pub fn find_nearest_edge(
    ref_x: usize,
    ref_y: usize,
    block_x: usize,
    block_y: usize,
    block: &str,
) -> Point {
    let candidates = find_edge(block_y, block_x, block);

    // Find the closest edge midpoint using squared distance
    *candidates
        .iter()
        .min_by_key(|&&(x, y)| {
            let dx = ref_x as i32 - x as i32;
            let dy = ref_y as i32 - y as i32;
            dx * dx + dy * dy
        })
        .expect("no candidates")
}

pub fn colorize(color_code: char) -> String {
    let color_str = match color_code {
        'B' => "\x1b[30m", //Black
        'r' => "\x1b[31m", //Red
        'g' => "\x1b[32m", //Green
        'y' => "\x1b[33m", //Yellow
        'b' => "\x1b[34m", //Blue
        'm' => "\x1b[35m", //Magenta
        'c' => "\x1b[36m", //Cyan
        'G' => "\x1b[37m", //Gray
        'w' => "\x1b[39m", //White
        'R' => "\x1b[0m",  //Reset
        'A' => "",         //not a color used for avoid.
        _ => "",
    };
    color_str.to_string()
}
