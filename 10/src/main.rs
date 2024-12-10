use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let mut map: Vec<Vec<usize>> = Vec::new();
    let mut trail_heads: HashSet<(usize, usize)> = HashSet::new();

    read_to_string("input.txt")
        .unwrap()
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            let mut line_vec: Vec<usize> = Vec::with_capacity(line.len());

            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .enumerate()
                .for_each(|(j, c)| {
                    line_vec.push(c);
                    if c == 0 {
                        trail_heads.insert((i, j));
                    }
                });

            map.push(line_vec);
        });

    let bounds = (map.len(), map.get(0).unwrap().len());
    let mut destinations_count = 0;
    let mut trail_ratings_sum = 0;
    for head in trail_heads {
        let mut destinations: HashSet<(usize, usize)> = HashSet::new();
        let mut trail_head_raing = 0;

        visit(
            head,
            0,
            &map,
            bounds,
            &mut destinations,
            &mut trail_head_raing,
        );

        destinations_count += destinations.len();
        trail_ratings_sum += trail_head_raing;
    }

    println!("{destinations_count}");
    println!("{trail_ratings_sum}");
}

fn visit(
    pos: (usize, usize),
    elevation: usize,
    map: &Vec<Vec<usize>>,
    bounds: (usize, usize),
    destinations: &mut HashSet<(usize, usize)>,
    trail_head_raing: &mut usize,
) {
    if elevation == 9 {
        destinations.insert(pos);
        *trail_head_raing += 1;
    }

    for (x, y) in adjacent(pos, bounds) {
        let &height = map.get(x).unwrap().get(y).unwrap();
        if height == elevation + 1 {
            visit((x, y), height, map, bounds, destinations, trail_head_raing);
        }
    }
}

fn adjacent((x, y): (usize, usize), bounds: (usize, usize)) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = Vec::with_capacity(4);

    if x > 0 {
        res.push((x - 1, y));
    }
    if x < bounds.0 - 1 {
        res.push((x + 1, y));
    }
    if y > 0 {
        res.push((x, y - 1));
    }
    if y < bounds.1 - 1 {
        res.push((x, y + 1));
    }

    res
}
