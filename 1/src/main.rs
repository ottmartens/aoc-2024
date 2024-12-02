use std::fs::read_to_string;

fn main() {
    let (mut col_1, mut col_2): (Vec<u32>, Vec<u32>) = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|s| {
            let mut nums = s
                .split_whitespace()
                .map(|num| num.parse::<u32>().expect("Not a number"));
            (nums.next().unwrap(), nums.next().unwrap())
        })
        .unzip();

    col_1.sort();
    col_2.sort();

    let mut element_wise_diff: u32 = 0;
    let mut similarity_scores_sum: u32 = 0;

    let mut first_idx_2 = 0;
    let col_2_slice = col_2.as_slice();

    for (idx, elem_1) in col_1.iter().enumerate() {
        element_wise_diff += elem_1.abs_diff(col_2[idx]);

        let mut elem_1_count = 0;

        for idx_2 in first_idx_2..col_2_slice.len() {
            let elem_2 = col_2_slice[idx_2];

            if elem_2 == *elem_1 {
                elem_1_count += 1
            } else if elem_2.gt(elem_1) {
                first_idx_2 = idx_2;
                break;
            };
        }

        similarity_scores_sum += elem_1 * elem_1_count;

    }
    println!("{element_wise_diff}");
    println!("{similarity_scores_sum}");
}
