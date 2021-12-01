fn part1(nums: &[i32]) {
    let increases: i32 = nums
        .iter()
        .zip(nums.iter().skip(1))
        .map(|(x, y)| if y > x { 1 } else { 0 })
        .sum();
    println!("Day 1 part 1: {increases}");
}

fn part2(nums: &[i32]) {
    let sums: Vec<_> = nums.windows(3).map(|w| w.iter().sum::<i32>()).collect();
    let increases: i32 = sums
        .iter()
        .zip(sums.iter().skip(1))
        .map(|(x, y)| if y > x { 1 } else { 0 })
        .sum();
    println!("Day 1 part 2: {increases}")
}

pub fn run() {
    let input = include_str!("../inputs/day01.txt");

    let nums: Vec<_> = input
        .lines()
        .map(str::parse::<i32>)
        .map(|f| f.unwrap())
        .collect();

    part1(&nums);
    part2(&nums);
}
