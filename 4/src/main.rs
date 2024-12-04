use std::fs::read_to_string;

static PATTERN: [char; 4] = ['X', 'M', 'A', 'S'];
static X_PATTERN: (char, char, char) = ('M', 'A', 'S');

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let lines: Vec<Vec<char>> = input.lines().map(|s| s.chars().collect()).collect();

    let rows = lines.len();
    let cols = lines.get(0).unwrap().len();

    let mut found_count = 0;
    let mut found_x_count = 0;

    let mut forward_found = 0;
    let mut backward_found = 0;

    for row in &lines {
        for char in row {
            handle_char(
                char,
                &mut forward_found,
                &mut backward_found,
                &mut found_count,
            )
        }
        (forward_found, backward_found) = (0, 0);
    }

    // columns
    for j in 0..cols {
        for i in 0..rows {
            let char = lines.get(i).unwrap().get(j).unwrap();
            handle_char(
                char,
                &mut forward_found,
                &mut backward_found,
                &mut found_count,
            )
        }
        (forward_found, backward_found) = (0, 0);
    }

    // forwards lower diagonals
    for start_row in 0..rows {
        let mut i = start_row;
        let mut j = 0;
        while i < rows && j < cols {
            let char = lines.get(i).unwrap().get(j).unwrap();
            handle_char(
                char,
                &mut forward_found,
                &mut backward_found,
                &mut found_count,
            );
            i += 1;
            j += 1;
        }
        (forward_found, backward_found) = (0, 0);
    }

    // forwards upper diagonals
    for start_col in 1..cols {
        let mut i = 0;
        let mut j = start_col;
        while i < rows && j < cols {
            let char = lines.get(i).unwrap().get(j).unwrap();
            handle_char(
                char,
                &mut forward_found,
                &mut backward_found,
                &mut found_count,
            );
            i += 1;
            j += 1;
        }
        (forward_found, backward_found) = (0, 0);
    }

    // backwards lower diagonals
    for start_row in 0..rows {
        let mut i = start_row;
        let mut j = (cols as i32) - 1;
        while i < rows && j >= 0 {
            let char = lines.get(i).unwrap().get(j as usize).unwrap();
            handle_char(
                char,
                &mut forward_found,
                &mut backward_found,
                &mut found_count,
            );
            i += 1;
            j -= 1;
        }
        (forward_found, backward_found) = (0, 0);
    }

    // backwards upper diagonals
    for start_col in 0..cols - 1 {
        let mut i = 0;
        let mut j = start_col as i32;
        while i < rows && j >= 0 {
            let char = lines.get(i).unwrap().get(j as usize).unwrap();
            handle_char(
                char,
                &mut forward_found,
                &mut backward_found,
                &mut found_count,
            );
            i += 1;
            j -= 1;
        }
        (forward_found, backward_found) = (0, 0);
    }

    for i in 1..rows - 1 {
        for j in 1..cols - 1 {
            if check_x_pattern(&lines, i, j) {
                found_x_count += 1;
            }
        }
    }

    println!("{found_count}");
    println!("{found_x_count}");
}

fn handle_char(
    c: &char,
    forward_found: &mut usize,
    backward_found: &mut usize,
    found_count: &mut usize,
) {
    if *c == PATTERN[*forward_found] {
        *forward_found += 1;

        if *forward_found == PATTERN.len() {
            *found_count += 1;
            *forward_found = 0;
        }
    } else {
        *forward_found = if *c == PATTERN[0] { 1 } else { 0 };
    }

    if *c == PATTERN[PATTERN.len() - 1 - *backward_found] {
        *backward_found += 1;

        if *backward_found == PATTERN.len() {
            *found_count += 1;
            *backward_found = 0;
        }
    } else {
        *backward_found = if *c == PATTERN[PATTERN.len() - 1] {
            1
        } else {
            0
        };
    }
}

fn check_x_pattern(grid: &Vec<Vec<char>>, i: usize, j: usize) -> bool {
    let top_left = (grid[i - 1][j - 1], grid[i][j], grid[i + 1][j + 1]);
    let bottom_left = (grid[i + 1][j - 1], grid[i][j], grid[i - 1][j + 1]);

    let forward_pattern = X_PATTERN;
    let backward_pattern = (X_PATTERN.2, X_PATTERN.1, X_PATTERN.0);

    return (top_left == forward_pattern || top_left == backward_pattern)
        && (bottom_left == forward_pattern || bottom_left == backward_pattern);
}
