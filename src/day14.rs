use std::collections::HashMap;

fn get_answer(pair_counts: &HashMap<(char, char), i64>, last_ch: char) -> i64 {
    let mut counts = HashMap::new();
    counts.insert(last_ch, 1);

    for (&(a, _b), num) in pair_counts.iter() {
        *counts.entry(a).or_insert(0i64) += num;
    }

    let (least, most) = counts
        .values()
        .fold((i64::MAX, 0i64), |(l, m), &v| (l.min(v), m.max(v)));
    most - least
}

pub fn run() {
    let input = include_str!("../inputs/day14.txt");
    let (template, input_rules) = input.split_once("\n\n").unwrap();
    let rules: HashMap<(char, char), char> = input_rules
        .lines()
        .filter_map(|line| line.split_once(" -> "))
        .map(|(key, value)| {
            let mut chars = key.chars();
            (
                (
                    chars.next().unwrap_or_default(),
                    chars.next().unwrap_or_default(),
                ),
                value.chars().next().unwrap_or_default(),
            )
        })
        .collect();
    let mut pair_counts = HashMap::new();

    for (a, b) in template.chars().zip(template.chars().skip(1)) {
        *pair_counts.entry((a, b)).or_insert(0i64) += 1;
    }

    for i in 0..40 {
        let mut counts = HashMap::new();
        for (&(a, b), value) in pair_counts.iter() {
            if let Some(&ch) = rules.get(&(a, b)) {
                *counts.entry((a, ch)).or_insert(0i64) += value;
                *counts.entry((ch, b)).or_insert(0i64) += value;
            }
        }
        pair_counts = counts;
        if i == 9 {
            let answer = get_answer(&pair_counts, template.chars().last().unwrap());
            println!("Day 14 part 1: {answer}");
        }
    }

    let answer = get_answer(&pair_counts, template.chars().last().unwrap());
    println!("Day 14 part 2: {answer}");
}
