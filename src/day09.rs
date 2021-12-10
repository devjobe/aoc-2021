use std::collections::HashSet;

fn get_basin_size(depths: &Vec<Vec<i32>>, x: usize, y: usize) -> usize {
    let mut frontier = Vec::new();
    frontier.push((x, y));

    let mut in_basin = HashSet::new();
    in_basin.insert((x, y));

    while let Some(coords) = frontier.pop() {
        let row = &depths[coords.1];
        let current = row[coords.0];
        if coords.1 > 0 {
            let depth = depths[coords.1 - 1][coords.0];
            if depth != 9 && depth > current {
                let new_coords = (coords.0, coords.1 - 1);
                if in_basin.insert(new_coords) {
                    frontier.push(new_coords);
                }
            }
        }

        if coords.1 + 1 < depths.len() {
            let depth = depths[coords.1 + 1][coords.0];
            if depth != 9 && depth > current {
                let new_coords = (coords.0, coords.1 + 1);
                if in_basin.insert(new_coords) {
                    frontier.push(new_coords);
                }
            }
        }

        if coords.0 > 0 {
            let depth = row[coords.0 - 1];
            if depth != 9 && depth > current {
                let new_coords = (coords.0 - 1, coords.1);
                if in_basin.insert(new_coords) {
                    frontier.push(new_coords);
                }
            }
        }

        if coords.0 + 1 < row.len() {
            let depth = row[coords.0 + 1];
            if depth != 9 && depth > current {
                let new_coords = (coords.0 + 1, coords.1);
                if in_basin.insert(new_coords) {
                    frontier.push(new_coords);
                }
            }
        }
    }

    in_basin.len()
}

pub fn run() {
    let input = include_str!("../inputs/day09.txt");

    let nums: Vec<_> = input
        .lines()
        .map(|x| x.chars().map(|c| c as i32 - '0' as i32).collect::<Vec<_>>())
        .collect();

    let mut basin_sizes = Vec::new();
    let mut answer = 0;
    for (y, row) in nums.iter().enumerate() {
        for (x, &depth) in row.iter().enumerate() {
            if (x == 0 || row[(x - 1)] > depth)
                && row.get(x + 1).copied().unwrap_or(i32::MAX) > depth
            {
                if (y == 0 || nums[y - 1][x] > depth)
                    && nums.get(y + 1).map(|below| below[x]).unwrap_or(i32::MAX) > depth
                {
                    answer += depth + 1;
                    basin_sizes.push(get_basin_size(&nums, x, y))
                }
            }
        }
    }

    println!("Day 9 part 1 {answer}");

    basin_sizes.sort();
    let answer2: usize = basin_sizes.iter().rev().take(3).product();
    println!("Day 9 part 2 {answer2}");
}
