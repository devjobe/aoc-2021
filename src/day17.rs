use std::ops::RangeInclusive;

trait TriangleNumber {
    fn tri_sum(self) -> Self;
}

impl TriangleNumber for i32 {
    fn tri_sum(self) -> Self {
        (self * (self + 1)) / 2
    }
}

pub fn run() {
    let (_text, input) = include_str!("../inputs/day17.txt").trim().split_at(15);
    let (input_x, input_y) = input
        .split_once(", y=")
        .expect("Input correctly formatted.");
    fn parse_range(s: &str) -> Option<RangeInclusive<i32>> {
        let (min, max) = s.split_once("..")?;
        Some(min.parse().ok()?..=max.parse().ok()?)
    }
    let area_x = parse_range(input_x).expect("Input correctly formatted.");
    let area_y = parse_range(input_y).expect("Input correctly formatted.");

    fn solve(target: i32) -> i32 {
        (((target * 8 + 1) as f64).sqrt() as i32 + 1) / 2
    }
    let min_vel_x = solve(*area_x.start());
    let mut answer = 0;
    let mut answer2 = 0;
    for initial_x in min_vel_x..=*area_x.end() {
        let mut max_y = 0;
        for initial_y in *area_y.start()..=(0 - area_y.start()) {
            let mut vel_x = initial_x;
            let mut vel_y = initial_y;
            let mut x = 0;
            let mut y = 0;
            let success = loop {
                x += vel_x;
                y += vel_y;
                vel_x -= vel_x.signum();
                vel_y -= 1;
                if area_y.contains(&y) && area_x.contains(&x) {
                    break true;
                }
                if y < *area_y.start() {
                    break false;
                }
            };
            if success {
                if initial_y > 0 {
                    max_y = max_y.max(initial_y.tri_sum());
                }
                answer2 += 1;
            }
        }
        answer = answer.max(max_y);
    }
    println!("Day 17 part 1: {answer}");
    println!("Day 17 part 2: {answer2}");
}
