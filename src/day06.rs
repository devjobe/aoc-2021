use std::collections::HashMap;

fn spawned(days: i64, memo: &mut HashMap<i64, i64>) -> i64 {
    if days <= 0 {
        return 0;
    }

    if let Some(&n) = memo.get(&days) {
        return n;
    }

    let k = (days+6) / 7;
    let mut sum = k;
    for n in 0..k {
        sum += spawned(days-n*7-9, memo);
    }
    memo.insert(days, sum);
    sum
}

pub fn run() {
    let input = include_str!("../inputs/day06.txt");
    let fishes: Vec<_> = input.split(',').map(str::parse::<i64>).collect::<Result<Vec<i64>, _>>().expect("Numbers");

    let mut memo = HashMap::new();

    {
        let total_days = 80;
        let sum : i64 = fishes.iter().map(|&t| { spawned(total_days-t, &mut memo)+1 }).sum();
        println!("Day 06 part 1: {sum}");
    }

    {
        let total_days = 256;
        let sum : i64= fishes.iter().map(|&t| { spawned(total_days-t, &mut memo)+1 }).sum();
        println!("Day 06 part 2: {sum}");
    }
}