#[derive(PartialEq, Eq, Copy, Clone)]
enum Tile {
    EastBound,
    SouthBound,
    Empty,
}

pub fn run() {
    let input = include_str!("../inputs/day25.txt");
    let mut seafloor: Vec<Vec<Tile>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '>' => Tile::EastBound,
                    'v' => Tile::SouthBound,
                    '.' => Tile::Empty,
                    _ => panic!("Unrecognized input {ch}"),
                })
                .collect()
        })
        .collect();

    for n in 1.. {
        let mut changed = 0;

        let state = seafloor.clone();
        for y in 0..seafloor.len() {
            let xlen = seafloor[y].len();
            for x in 0..seafloor[y].len() {
                let x1 = (x + 1) % xlen;
                if state[y][x] == Tile::EastBound && state[y][x1] == Tile::Empty {
                    seafloor[y][x1] = Tile::EastBound;
                    seafloor[y][x] = Tile::Empty;
                    changed += 1;
                }
            }
        }

        let state = seafloor.clone();
        for y in 0..seafloor.len() {
            for x in 0..seafloor[y].len() {
                let y1 = (y + 1) % seafloor.len();
                if state[y][x] == Tile::SouthBound && state[y1][x] == Tile::Empty {
                    seafloor[y1][x] = Tile::SouthBound;
                    seafloor[y][x] = Tile::Empty;
                    changed += 1;
                }
            }
        }

        if changed == 0 {
            println!("Day 25 part 1: {n}");
            break;
        }
    }
}
