use std::{collections::VecDeque, fs::read_to_string};

fn main() {
    let mut initial_disk_state: Vec<Option<usize>> = Vec::new();
    let mut disk_state_chunks: Vec<(usize, usize, usize)> = Vec::new();
    let mut initial_empty_slots: VecDeque<(usize, usize)> = VecDeque::new();

    read_to_string("input.txt")
        .unwrap()
        .trim()
        .chars()
        .map(|c| c.to_string().parse::<usize>().unwrap())
        .enumerate()
        .for_each(|(i, count)| {
            let empty_space = i % 2 == 1;

            let elem = if empty_space { None } else { Some(i / 2) };

            if empty_space {
                if count == 0 {
                    return;
                }
                initial_empty_slots.push_back((initial_disk_state.len(), count));
            } else {
                disk_state_chunks.push((initial_disk_state.len(), elem.unwrap(), count));
            }
            initial_disk_state.extend(std::iter::repeat_n(elem, count));
        });

    // Part 1

    let mut empty_slots = initial_empty_slots.clone();
    let mut cur_empty_slot: Option<(usize, usize)> = None;
    let mut next_empty_idx = || -> Option<usize> {
        if cur_empty_slot.is_none() {
            cur_empty_slot = empty_slots.pop_front();
            if cur_empty_slot.is_none() {
                return None;
            }
        }

        let (idx, count) = cur_empty_slot.unwrap();

        cur_empty_slot = if count > 1 {
            Some((idx + 1, count - 1))
        } else {
            None
        };

        return Some(idx);
    };

    let mut disk_state = initial_disk_state.clone();
    for idx in (0..disk_state.len()).rev() {
        let Some(elem) = disk_state[idx] else {
            continue;
        };
        let Some(empty_idx) = next_empty_idx() else {
            break;
        };
        if empty_idx >= idx {
            break;
        };

        disk_state[empty_idx] = Some(elem);
        disk_state[idx] = None;
    }

    println!("{}", checksum(disk_state));

    // Part 2

    disk_state = initial_disk_state.clone();

    let mut first_fitting_empty_slot = |size: usize| -> Option<(usize, usize)> {
        for (idx, slot) in initial_empty_slots.iter_mut().enumerate() {
            if slot.1 >= size {
                let res = Some(*slot);

                if slot.1 == size {
                    initial_empty_slots.remove(idx);
                } else {
                    slot.0 += size;
                    slot.1 -= size;
                }
                return res;
            }
        }

        return None;
    };

    for &(idx, id, size) in disk_state_chunks.iter().rev() {
        if let Some(empty_slot) = first_fitting_empty_slot(size) {
            if empty_slot.0 >= idx {
                continue;
            }

            for i in 0..size {
                disk_state[empty_slot.0 + i] = Some(id);
                disk_state[idx + i] = None;
            }
        } else {
            continue;
        }
    }

    println!("{}", checksum(disk_state));
}

fn checksum(disk_state: Vec<Option<usize>>) -> usize {
    return disk_state
        .iter()
        .enumerate()
        .filter_map(|(i, &elem)| elem.map(|x| x * i))
        .sum();
}
