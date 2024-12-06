use std::{collections::HashSet, fs::read_to_string};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    N,
    S,
    W,
    E,
}

fn main() {
    let mut obstacles: HashSet<(i32, i32)> = HashSet::new();
    let mut initial_guard_pos: (i32, i32) = (0, 0);
    let mut initial_guard_direction: Direction = Direction::N;

    let input = read_to_string("input.txt").unwrap();
    let mut rows: i32 = 0;
    let mut cols: i32 = 0;

    input.lines().enumerate().for_each(|(i, line)| {
        rows += 1;
        if cols == 0 {
            cols = line.len() as i32
        }
        for (j, char) in line.chars().enumerate() {
            let pos = (i as i32, j as i32);
            match char {
                '#' => {
                    obstacles.insert(pos);
                }
                '^' | '>' | 'v' | '<' => {
                    initial_guard_direction = match char {
                        '^' => Direction::N,
                        '>' => Direction::E,
                        'v' => Direction::S,
                        '<' => Direction::W,
                        _ => unreachable!(),
                    };
                    initial_guard_pos = pos;
                }
                _ => {}
            }
        }
    });

    let mut exited: bool = false;
    let mut guard_pos = initial_guard_pos;
    let mut guard_direction = initial_guard_direction;

    // Part 1

    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    while !exited {
        visited.insert(guard_pos);
        (exited, guard_pos, guard_direction) =
            guard_step(guard_pos, guard_direction, &obstacles, rows, cols);
    }

    println!("{}", visited.len());

    // Part 2

    let mut blockable_pos_count = 0;
    let mut blockable = visited.clone();
    blockable.remove(&initial_guard_pos);

    for point in blockable {
        obstacles.insert(point);

        let mut directed_visited: HashSet<((i32, i32), Direction)> = HashSet::new();

        exited = false;
        guard_pos = initial_guard_pos;
        guard_direction = initial_guard_direction;

        while !exited {
            directed_visited.insert((guard_pos, guard_direction.clone()));

            (exited, guard_pos, guard_direction) =
                guard_step(guard_pos, guard_direction, &obstacles, rows, cols);

            if directed_visited.contains(&(guard_pos, guard_direction)) {
                blockable_pos_count += 1;
                break;
            }
        }

        obstacles.remove(&point);
    }

    println!("{blockable_pos_count}");
}

fn guard_step(
    (x, y): (i32, i32),
    direction: Direction,
    obstacles: &HashSet<(i32, i32)>,
    rows: i32,
    cols: i32,
) -> (bool, (i32, i32), Direction) {
    let (nx, ny) = match direction {
        Direction::N => (x - 1, y),
        Direction::E => (x, y + 1),
        Direction::S => (x + 1, y),
        Direction::W => (x, y - 1),
    };

    if nx < 0 || nx == rows || ny < 0 || ny == cols {
        return (true, (nx, ny), direction);
    }

    if obstacles.contains(&(nx, ny)) {
        let new_direction = match direction {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        };
        return (false, (x, y), new_direction);
    }

    return (false, (nx, ny), direction);
}
