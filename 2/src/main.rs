use std::fs::read_to_string;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Direction {
    Increase,
    Decrease,
    Same,
}

fn main() {
    let mut safe_count: u32 = 0;

    read_to_string("input.txt").unwrap().lines().for_each(|s| {
        let report: Vec<i32> = s
            .split_whitespace()
            .map(|s| s.parse::<i32>().expect("Not a number"))
            .collect();

        if is_valid_report(&report) {
            safe_count += 1
        }
    });

    println!("{safe_count}")
}

fn is_valid_report(levels: &Vec<i32>) -> bool {
    if check_sequence(levels) {
        return true;
    }

    for idx in 0..levels.len() {
        let mut removed_idx_levels = levels.clone();
        removed_idx_levels.remove(idx);

        if check_sequence(&removed_idx_levels) {
            return true;
        }
    }

    return false;
}

fn check_sequence(levels: &Vec<i32>) -> bool {
    let mut direction: Direction = Direction::Same;
    let mut previous_value: i32 = -1;

    for (idx, &value) in levels.iter().enumerate() {
        let cur_direction = get_direction(value, previous_value);
        let diff_from_previous = value.abs_diff(previous_value);
        previous_value = value;

        if idx == 0 {
            continue;
        }
        if idx == 1 {
            direction = cur_direction;
        }

        let invalid_diff = diff_from_previous == 0 || diff_from_previous > 3;
        let invalid_direction = idx > 1 && direction != cur_direction;

        if invalid_diff || invalid_direction {
            return false;
        }
    }

    return true;
}

fn get_direction(num1: i32, num2: i32) -> Direction {
    if num1 > num2 {
        return Direction::Increase;
    } else if num1 < num2 {
        return Direction::Decrease;
    } else {
        return Direction::Same;
    }
}
