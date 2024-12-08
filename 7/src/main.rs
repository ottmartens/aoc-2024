use std::fs::read_to_string;

fn main() {
    let (mut sum_basic_ops, mut sum_with_concat): (u64, u64) = (0, 0);

    let concat = |a, b| format!("{a}{b}").parse().unwrap();

    read_to_string("input.txt")
        .unwrap()
        .lines()
        .for_each(|line| {
            let mut nums = line.split(" ");
            let correct_answer = nums.next().unwrap().trim_end_matches(":").parse().unwrap();

            let components: Vec<u64> = nums.map(|num| num.parse().unwrap()).collect();
            let (a, b) = (components[0], components[1]);

            let mut basic_results = vec![a * b, a + b];
            let mut concat_results = vec![a * b, a + b, concat(a, b)];

            for &c in &components[2..] {
                basic_results = basic_results
                    .iter()
                    .flat_map(|&res| [res * c, res + c])
                    .collect();

                concat_results = concat_results
                    .iter()
                    .flat_map(|&res| [res * c, res + c, concat(res, c)])
                    .collect();
            }

            if basic_results.contains(&correct_answer) {
                sum_basic_ops += correct_answer
            }
            if concat_results.contains(&correct_answer) {
                sum_with_concat += correct_answer
            }
        });

    println!("{}", sum_basic_ops);
    println!("{}", sum_with_concat);
}
