use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Amphipod {
    A,
    B,
    C,
    D,
}

impl Amphipod {
    fn movement_cost(&self) -> usize {
        match self {
            Amphipod::A => 1,
            Amphipod::B => 10,
            Amphipod::C => 100,
            Amphipod::D => 1000,
        }
    }

    fn destination(&self) -> usize {
        match self {
            Amphipod::A => 2,
            Amphipod::B => 4,
            Amphipod::C => 6,
            Amphipod::D => 8,
        }
    }

    fn from_char(ch: char) -> Option<Amphipod> {
        match ch {
            'A' => Some(Amphipod::A),
            'B' => Some(Amphipod::B),
            'C' => Some(Amphipod::C),
            'D' => Some(Amphipod::D),
            _ => None,
        }
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Burrow {
    locations: [[Option<Amphipod>; 11]; 5],
}

impl Burrow {
    fn from_input(input: &str) -> Self {
        let mut burrow = Burrow {
            locations: Default::default(),
        };

        let mut lines = input.lines().skip(2);

        let line0 = lines.next().expect("input");
        let line1 = lines.next().expect("input");

        let mut burrow0 = line0.chars().skip(3).step_by(2).map(Amphipod::from_char);

        let (a0, a1, a2, a3) = (
            burrow0.next().expect("amphipod"),
            burrow0.next().expect("amphipod"),
            burrow0.next().expect("amphipod"),
            burrow0.next().expect("amphipod"),
        );

        burrow.locations[1][2] = a0;
        burrow.locations[1][4] = a1;
        burrow.locations[1][6] = a2;
        burrow.locations[1][8] = a3;

        let mut burrow3 = line1.chars().skip(3).step_by(2).map(Amphipod::from_char);

        let (d0, d1, d2, d3) = (
            burrow3.next().expect("amphipod"),
            burrow3.next().expect("amphipod"),
            burrow3.next().expect("amphipod"),
            burrow3.next().expect("amphipod"),
        );

        burrow.locations[2][2] = d0;
        burrow.locations[2][4] = d1;
        burrow.locations[2][6] = d2;
        burrow.locations[2][8] = d3;

        burrow
    }

    fn part2(&mut self) {
        self.locations[4][2] = self.locations[2][2];
        self.locations[4][4] = self.locations[2][4];
        self.locations[4][6] = self.locations[2][6];
        self.locations[4][8] = self.locations[2][8];

        let (b0, b1, b2, b3) = (Amphipod::D, Amphipod::C, Amphipod::B, Amphipod::A);

        self.locations[2][2] = Some(b0);
        self.locations[2][4] = Some(b1);
        self.locations[2][6] = Some(b2);
        self.locations[2][8] = Some(b3);

        let (c0, c1, c2, c3) = (Amphipod::D, Amphipod::B, Amphipod::A, Amphipod::C);
        self.locations[3][2] = Some(c0);
        self.locations[3][4] = Some(c1);
        self.locations[3][6] = Some(c2);
        self.locations[3][8] = Some(c3);
    }

    fn should_move_to_hallway(&self, pos: &(usize, usize), row_count: usize) -> bool {
        let a = if let Some(a) = self.locations[pos.0][pos.1] {
            a
        } else {
            return false;
        };

        let destination = a.destination();
        let should_move = if pos.1 == destination {
            if pos.0 == 2 {
                return false;
            }

            for n in pos.0 + 1..=row_count {
                if self.locations[n][pos.1] != Some(a) {
                    return true;
                }
            }
            false
        } else {
            true
        };

        if should_move {
            for n in 1..pos.0 {
                if self.locations[n][pos.1] != None {
                    return false;
                }
            }
            true
        } else {
            false
        }
    }

    fn try_move_to_room(&self, x: usize, row_count: usize) -> Option<((usize, usize), usize)> {
        let a = self.locations[0][x]?;

        let destination = a.destination();
        if destination < x {
            for n in destination + 1..x {
                if self.locations[0][n] != None {
                    return None;
                }
            }
        } else {
            for n in x + 1..destination {
                if self.locations[0][n] != None {
                    return None;
                }
            }
        }

        let mut t = 0;
        for n in 1..=row_count {
            match self.locations[n][destination] {
                Some(b) if b != a => return None,
                None => t = n,
                _ => {}
            }
        }

        let new_pos = (t, destination);
        let cost = movement_cost(&(0, x), &new_pos, a);
        Some((new_pos, cost))
    }

    fn try_move_to_hallway(
        &self,
        pos: &(usize, usize),
        to_x: usize,
    ) -> Option<((usize, usize), usize)> {
        let a = self.locations[pos.0][pos.1]?;

        let s = pos.1.min(to_x);
        let e = pos.1.max(to_x);

        for n in s..=e {
            if self.locations[0][n] != None {
                return None;
            }
        }
        let new_pos = (0, to_x);
        let cost = movement_cost(pos, &new_pos, a);
        Some((new_pos, cost))
    }

    fn is_solved(&self, row_count: usize) -> bool {
        for x in (2..=8).step_by(2) {
            for y in 1..=row_count {
                if let Some(a) = self.locations[y][x] {
                    if a.destination() != x {
                        return false;
                    }
                } else {
                    return false;
                }
            }
        }
        true
    }

    fn step(
        &mut self,
        memo: &mut HashMap<Burrow, usize>,
        row_count: usize,
        from: &(usize, usize),
        to: &(usize, usize),
    ) -> usize {
        let a = self.locations[from.0][from.1];

        self.locations[to.0][to.1] = a;
        self.locations[from.0][from.1] = None;

        let result = self.solve(memo, row_count);

        self.locations[to.0][to.1] = None;
        self.locations[from.0][from.1] = a;

        result
    }

    fn solve(&mut self, memo: &mut HashMap<Burrow, usize>, row_count: usize) -> usize {
        if let Some(&cost) = memo.get(self) {
            return cost;
        }

        if self.is_solved(row_count) {
            memo.insert(self.clone(), 0);
            return 0;
        }

        let mut current_best = usize::MAX;
        for pos in (1..=row_count).flat_map(|y| [2, 4, 6, 8].into_iter().map(move |x| (y, x))) {
            if !self.should_move_to_hallway(&pos, row_count) {
                continue;
            }

            for to_x in (0..11).filter(|n| ![2, 4, 6, 8].contains(n)) {
                if let Some((new_pos, cost)) = self.try_move_to_hallway(&pos, to_x) {
                    let result = self.step(memo, row_count, &pos, &new_pos);
                    if result != usize::MAX {
                        current_best = current_best.min(result + cost);
                    }
                }
            }
        }

        for x in 0..11 {
            if let Some((new_pos, cost)) = self.try_move_to_room(x, row_count) {
                let pos = (0, x);
                let result = self.step(memo, row_count, &pos, &new_pos);
                if result != usize::MAX {
                    current_best = current_best.min(result + cost);
                }
            }
        }

        memo.insert(self.clone(), current_best);

        current_best
    }
}

fn movement_cost(from: &(usize, usize), to: &(usize, usize), a: Amphipod) -> usize {
    let steps = (from.0.max(to.0) - from.0.min(to.0)) + (from.1.max(to.1) - from.1.min(to.1));
    steps * a.movement_cost()
}

pub fn run() {
    let mut burrow = Burrow::from_input(include_str!("../inputs/day23.txt"));
    let mut memo = HashMap::new();
    let answer = burrow.solve(&mut memo, 2);
    println!("Day 23.1: {answer}");

    memo = HashMap::with_capacity(64 * 1024);

    burrow.part2();
    let answer = burrow.solve(&mut memo, 4);
    println!("Day 23.2: {answer}");
}
