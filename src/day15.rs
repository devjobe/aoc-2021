use std::collections::BinaryHeap;

#[derive(PartialEq, Eq)]
struct State {
    x: usize,
    y: usize,
    cost: i32,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

pub fn run() {
    let input = include_str!("../inputs/day15.txt");
    let grid: Vec<Vec<i32>> = input
        .lines()
        .map(|line| line.chars().map(|x| (x as i32) - ('0' as i32)).collect())
        .collect();
    let first_end = grid.len() - 1;
    let second_end = grid.len() * 5 - 1;

    let mut new_grid = vec![vec![0i32; grid.len() * 5]; grid.len() * 5];
    for (y, row) in new_grid.iter_mut().enumerate() {
        let grid_row = &grid[y % grid.len()];
        for (x, value) in row.iter_mut().enumerate() {
            let grid_value = grid_row[x % grid_row.len()];
            let new_value = (y / grid.len() + x / grid_row.len()) as i32 + grid_value;
            if new_value > 9 {
                *value = new_value - 9;
            } else {
                *value = new_value;
            }
        }
    }
    let grid = new_grid;

    let mut pq = BinaryHeap::new();
    pq.push(State {
        x: 0,
        y: 0,
        cost: 0,
    });
    let size = grid.len();
    let mut costs: Vec<Vec<i32>> = vec![vec![i32::MAX; size]; size];
    costs[0][0] = 0;
    while let Some(state) = pq.pop() {
        let State { x, y, cost } = state;

        if costs[y][x] < cost {
            continue;
        }

        let mut add = |x1: usize, y1: usize| {
            if let Some(step_cost) = grid.get(y1).and_then(|row: &Vec<i32>| row.get(x1).copied()) {
                let current = &mut costs[y1][x1];
                let updated = cost + step_cost;
                if *current > updated {
                    *current = updated;
                    pq.push(State {
                        x: x1,
                        y: y1,
                        cost: updated,
                    });
                }
            }
        };
        add(x + 1, y);
        add(x, y + 1);
        if x > 0 {
            add(x - 1, y);
        }
        if y > 0 {
            add(x, y - 1);
        }
    }
    let answer1 = costs[first_end][first_end];
    let answer2 = costs[second_end][second_end];
    println!("Day 15 part 1: {answer1}");
    println!("Day 15 part 2: {answer2}");
}
