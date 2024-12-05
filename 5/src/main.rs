use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

type Rules = HashMap<String, HashSet<String>>;
fn main() {
    let mut correct_sum = 0;
    let mut fixed_sum = 0;

    let mut rules_section = true;
    let mut rules: Rules = HashMap::new();

    for line in read_to_string("input.txt").unwrap().lines() {
        if line.is_empty() {
            rules_section = false;
            continue;
        }

        if rules_section {
            let [num1, num2] = line
                .split("|")
                .take(2)
                .map(str::to_string)
                .collect::<Vec<String>>()
                .try_into()
                .unwrap();

            rules.entry(num2).or_default().insert(num1);
        } else {
            let (sum, correct) = check_pages(line, &rules);
            if correct {
                correct_sum += sum;
            } else {
                fixed_sum += sum;
            }
        }
    }

    println!("{correct_sum}");
    println!("{fixed_sum}");
}

fn check_pages(line: &str, rules: &Rules) -> (u32, bool) {
    let pages: Vec<String> = line.split(",").map(str::to_string).collect();
    let page_count = pages.len();

    let alphabet: HashSet<String> = pages.iter().cloned().collect();
    let mut encountered_pages: HashSet<String> = HashSet::new();

    for page in &pages {
        for page_req in rules.get(page).unwrap() {
            if alphabet.contains(page_req) && !encountered_pages.contains(page_req) {
                let ordered = correct_ordering(rules, &alphabet);

                return (ordered[ordered.len() / 2].parse().unwrap(), false);
            }
        }

        encountered_pages.insert(page.to_string());
    }

    return (pages[page_count / 2].parse().unwrap(), true);
}

fn correct_ordering(rules: &Rules, alphabet: &HashSet<String>) -> Vec<String> {
    let mut needed_rules = rules.clone();

    for char in rules.keys() {
        if !alphabet.contains(char) {
            needed_rules.remove(char);
            continue;
        }

        let reqs = needed_rules.get_mut(char).unwrap();
        reqs.retain(|char_req| alphabet.contains(char_req));
    }

    let mut res: Vec<String> = alphabet.iter().cloned().collect();
    res.sort_by_key(|char| needed_rules.get(char).map_or(0, HashSet::len));

    return res;
}
