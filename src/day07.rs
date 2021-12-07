pub fn run() {
    let input = include_str!("../inputs/day07.txt");
    let mut crabs: Vec<_> = input
        .split(',')
        .map(str::parse::<i64>)
        .collect::<Result<Vec<i64>, _>>()
        .expect("Numbers");

    crabs.sort();

    {
        let median = crabs.get(crabs.len() / 2).copied().unwrap_or_default();
        let answer: i64 = crabs.iter().map(|&x| (x - median).abs()).sum();
        println!("Day 6 part 1: {answer}");
    }

    {
        fn fuel_cost(n: i64) -> i64 {
            n * (n + 1) / 2
        }
        let min = crabs.first().copied().unwrap_or_default();
        let max = crabs.last().copied().unwrap_or_default();
        let answer : i64 = (min..=max)
            .map(|k| crabs.iter().map(|&x| fuel_cost((x - k).abs())).sum())
            .min()
            .unwrap();
        println!("Day 6 part 2: {answer}");
    }
}
