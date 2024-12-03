use regex::Regex;
use std::fs::read_to_string;

fn main() {
    let regex = Regex::new(r"don't\(\)|do\(\)|mul\((?<num1>\d{1,3}),(?<num2>\d{1,3})\)").unwrap();
    let input = read_to_string("input.txt").unwrap();

    let mut result: u32 = 0;
    let mut enabled: bool = true;

    for c in regex.captures_iter(&input) {
        match &c[0] {
            "don't()" => enabled = false,
            "do()" => enabled = true,
            _ => {
                if enabled {
                    let num1 = c["num1"].parse::<u32>().unwrap();
                    let num2 = c["num2"].parse::<u32>().unwrap();

                    result += num1 * num2
                }
            }
        }
    }

    println!("{result}");
}
