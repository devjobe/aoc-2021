use std::collections::HashMap;

pub fn run() {
    let input = include_str!("../inputs/day10.txt");
    let scores = HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);
    let mut syntax_error_score = 0;
    let completion_scores = HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);
    let mut incomplete_scores = Vec::new();
    for line in input.lines() {
        let mut state = Vec::new();
        let mut corrupt = false;
        for ch in line.chars() {
            match ch {
                '(' => state.push(')'),
                '[' => state.push(']'),
                '{' => state.push('}'),
                '<' => state.push('>'),
                ch if state.last() == Some(&ch) => {
                    state.pop();
                }
                ch => {
                    if let Some(value) = scores.get(&ch) {
                        syntax_error_score += value;
                        corrupt = true;
                        break;
                    } else {
                        panic!("Unrecognized char!")
                    }
                }
            }
        }
        if !corrupt {
            let score = state
                .iter()
                .rev()
                .map(|ch| completion_scores.get(ch).copied().unwrap_or_default())
                .fold(0i64, |total_score, value| total_score * 5 + value);
            incomplete_scores.push(score);
        }
    }
    incomplete_scores.sort_unstable();
    let middle_score = incomplete_scores
        .get(incomplete_scores.len() / 2)
        .copied()
        .unwrap_or_default();
    println!("Day 10 part 1: {syntax_error_score}");
    println!("Day 10 part 2: {middle_score}");
}
