use std::collections::HashSet;

fn rotate(mut coords: (i32, i32, i32), facing: i32, up: i32) -> (i32, i32, i32) {
    // const Z_POS_UP: i32 = 0;
    const Z_NEG_UP: i32 = 1;
    const X_POS_UP: i32 = 2;
    const X_NEG_UP: i32 = 3;
    match up {
        Z_NEG_UP => {
            coords = (-coords.0, coords.1, -coords.2);
        }
        X_POS_UP => {
            coords = (coords.2, coords.1, -coords.0);
        }
        X_NEG_UP => {
            coords = (-coords.2, coords.1, coords.0);
        }
        _ => { // Z_POS
        }
    }
    // const Y_POS: i32 = 0;
    const Y_NEG: i32 = 1;
    const X_POS: i32 = 2;
    const X_NEG: i32 = 3;
    const Z_POS: i32 = 4;
    const Z_NEG: i32 = 5;
    match facing {
        Y_NEG => {
            coords = (-coords.0, -coords.1, coords.2);
        }
        X_POS => {
            coords = (coords.1, -coords.0, coords.2);
        }
        X_NEG => {
            coords = (-coords.1, coords.0, coords.2);
        }
        Z_POS => {
            coords = (coords.0, -coords.2, coords.1);
        }
        Z_NEG => {
            coords = (coords.0, coords.2, -coords.1);
        }
        _ => { // Y_POS
        }
    }
    coords
}

fn rotate_scans(
    scans: &HashSet<(i32, i32, i32)>,
    facing: i32,
    up: i32,
) -> HashSet<(i32, i32, i32)> {
    scans
        .iter()
        .cloned()
        .map(|coords| rotate(coords, facing, up))
        .collect()
}

fn overlaps(
    world: &HashSet<(i32, i32, i32)>,
    local: &HashSet<(i32, i32, i32)>,
) -> Option<((i32, i32, i32), HashSet<(i32, i32, i32)>)> {
    for origin in world.iter() {
        for local_origin in local.iter() {
            let scanner_origin = (
                origin.0 - local_origin.0,
                origin.1 - local_origin.1,
                origin.2 - local_origin.2,
            );
            let local_to_world = local.iter().map(|coords| {
                let world_coords = (
                    coords.0 + scanner_origin.0,
                    coords.1 + scanner_origin.1,
                    coords.2 + scanner_origin.2,
                );
                world_coords
            });

            if local_to_world
                .clone()
                .filter(|coords| world.contains(coords))
                .count()
                >= 12
            {
                return Some((scanner_origin, local_to_world.collect()));
            }
        }
    }
    None
}

pub fn run() {
    let input = include_str!("../inputs/day19.txt");
    let mut scanners = input
        .split("\n\n")
        .map(|paragraph| {
            paragraph
                .lines()
                .skip(1)
                .map(|line| {
                    let mut n = line.split(',').map(str::parse::<i32>);
                    let a = n.next().expect("Three numbers").expect("Integer");
                    let b = n.next().expect("Three numbers").expect("Integer");
                    let c = n.next().expect("Three numbers").expect("Integer");
                    (a, b, c)
                })
                .collect()
        })
        .enumerate()
        .collect::<Vec<(usize, HashSet<(i32, i32, i32)>)>>();

    println!("Parsed {} scanners", scanners.len());
    let rotations = (0..=3).flat_map(|up| (0..=5).map(move |facing| (facing, up)));

    let mut locations = Vec::with_capacity(scanners.len());
    locations.push((0i32, 0i32, 0i32));
    let mut unknown = scanners.drain(1..).collect::<Vec<_>>();

    while unknown.len() > 0 {
        for rotation in rotations.clone() {
            let mut rotated = unknown
                .iter()
                .map(|scans| (scans.0, rotate_scans(&scans.1, rotation.0, rotation.1)))
                .collect::<Vec<_>>();

            let mut scanner_index = 0;
            while scanner_index < scanners.len() {
                for index in (0..rotated.len()).rev() {
                    if let Some((origin, world)) =
                        overlaps(&scanners[scanner_index].1, &rotated[index].1)
                    {
                        unknown.remove(index);
                        let id = rotated.remove(index).0;
                        scanners.push((id, world));
                        locations.push(origin);
                        println!(
                            "{} matched with {} at {origin:?}",
                            id, scanners[scanner_index].0
                        );
                    }
                }
                scanner_index += 1;
            }
        }
    }

    let beacons = scanners
        .iter()
        .flat_map(|x| x.1.iter().copied())
        .collect::<HashSet<(i32, i32, i32)>>();
    let answer = beacons.len();
    println!("Day 19 part 1: {answer}");

    let mut answer2 = 0;
    let mut tail = locations.iter();
    for &(x, y, z) in locations.iter() {
        answer2 = answer2.max(x.abs() + y.abs() + z.abs());
        tail.next();
        for &(a, b, c) in tail.clone() {
            answer2 = answer2.max((x - a).abs() + (y - b).abs() + (z - c).abs());
        }
    }
    println!("Day 19 part 2: {answer2}");
}
