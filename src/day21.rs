use std::collections::HashMap;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone)]
struct Player {
    position: usize,
    score: usize,
}

impl Player {
    fn from(line: &str) -> Player {
        let position = line
            .trim()
            .chars()
            .last()
            .map(|ch| ch as i32 - '1' as i32)
            .expect("Digit at end of line") as usize;
        Player { position, score: 0 }
    }

    fn step(&self, k: usize) -> Player {
        let position = (self.position + k) % 10;
        let score = self.score + position + 1;
        Player { position, score }
    }
}

pub fn run() {
    let (p1, p2) = {
        let input = include_str!("../inputs/day21.txt");
        input
            .split_once('\n')
            .map(|(a, b)| (Player::from(a), Player::from(b)))
            .expect("Two lines")
    };
    part1(p1, p2);
    part2(p1, p2);
}

fn part1(mut p1: Player, mut p2: Player) {
    let mut d = 0;
    let mut roll = || {
        let k = d % 10 + (d + 1) % 10 + (d + 2) % 10 + 3;
        d += 3;
        k
    };

    let answer = loop {
        p1 = p1.step(roll());
        if p1.score >= 1000 {
            break p2.score * d;
        }

        p2 = p2.step(roll());
        if p2.score >= 1000 {
            break p1.score * d;
        }
    };

    println!("Day 21 part 1: {answer}");
}

fn universes(
    input: &(Player, Player),
    results: &mut HashMap<(Player, Player), (usize, usize)>,
) -> (usize, usize) {
    if let Some(&res) = results.get(input) {
        return res;
    }

    let mut wins = (0, 0);
    const OCCURENCES: [(usize, usize); 7] =
        [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    for &(r1, n1) in OCCURENCES.iter() {
        let a = input.0.step(r1);
        if a.score >= 21 {
            wins.0 += n1;
            continue;
        }

        for &(r2, n2) in OCCURENCES.iter() {
            let b = input.1.step(r2);
            let k = n1 * n2;
            if b.score >= 21 {
                wins.1 += k;
                continue;
            }

            let result = universes(&(a, b), results);
            wins.0 += result.0 * k;
            wins.1 += result.1 * k;
        }
    }
    results.insert(*input, wins);
    wins
}

fn part2(p1: Player, p2: Player) {
    let mut results = HashMap::new();
    let (u1, u2) = universes(&(p1, p2), &mut results);
    let answer = u1.max(u2);
    println!("Day 21 part 2: {answer}");
}
