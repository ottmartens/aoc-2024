use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

fn main() {
    let mut antennas: HashMap<char, HashSet<(i32, i32)>> = HashMap::new();

    let mut rows = 0;
    let mut cols = 0;

    read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            if i == 0 {
                cols = line.len() as i32;
            }
            rows += 1;

            line.char_indices()
                .filter(|&(_, c)| c != '.')
                .for_each(|(j, c)| {
                    antennas
                        .entry(c)
                        .or_insert_with(HashSet::new)
                        .insert((i as i32, j as i32));
                });
        });

    part_1(&antennas, rows, cols);
    part_2(&antennas, rows, cols);
}

fn part_1(antennas: &HashMap<char, HashSet<(i32, i32)>>, rows: i32, cols: i32) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, coords) in antennas {
        for (i, (x1, y1)) in coords.iter().enumerate() {
            for (j, (x2, y2)) in coords.iter().enumerate() {
                if i >= j {
                    continue;
                }
                let (dx, dy) = (x1 - x2, y1 - y2);

                let n1 = (x1 + dx, y1 + dy);
                let n2 = (x2 - dx, y2 - dy);

                if in_bounds(n1, (rows, cols)) {
                    antinodes.insert(n1);
                }
                if in_bounds(n2, (rows, cols)) {
                    antinodes.insert(n2);
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn part_2(antennas: &HashMap<char, HashSet<(i32, i32)>>, rows: i32, cols: i32) {
    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, coords) in antennas {
        for (i, (x1, y1)) in coords.iter().enumerate() {
            for (j, (x2, y2)) in coords.iter().enumerate() {
                if i >= j {
                    continue;
                }

                let (dx, dy) = (x1 - x2, y1 - y2);

                let (mut nx, mut ny) = (*x1, *y1);
                loop {
                    antinodes.insert((nx, ny));

                    (nx, ny) = (nx + dx, ny + dy);

                    if !in_bounds((nx, ny), (rows, cols)) {
                        break;
                    }
                }

                (nx, ny) = (x1 - dx, y1 - dy);
                loop {
                    antinodes.insert((nx, ny));

                    (nx, ny) = (nx - dx, ny - dy);

                    if !in_bounds((nx, ny), (rows, cols)) {
                        break;
                    }
                }
            }
        }
    }

    println!("{}", antinodes.len());
}

fn in_bounds((x, y): (i32, i32), (bx, by): (i32, i32)) -> bool {
    x >= 0 && x < bx && y >= 0 && y < by
}
