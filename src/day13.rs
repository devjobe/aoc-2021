use std::{collections::HashSet, iter::once};

enum Fold {
    X(i32),
    Y(i32),
}

pub fn run() {
    let input = include_str!("../inputs/day13.txt");
    let (mut coordinates, folds) = if let Some((coordinates, folds)) = input.split_once("\n\n") {
        let folds: Vec<_> = folds
            .lines()
            .filter_map(|line| {
                let (axis, value) = line[11..].split_once('=')?;
                let v = value.parse::<i32>().ok()?;
                if axis == "y" {
                    Some(Fold::Y(v))
                } else if axis == "x" {
                    Some(Fold::X(v))
                } else {
                    println!("Unrecognized fold axis: {axis}");
                    None
                }
            })
            .collect();
        let coordinates: HashSet<_> = coordinates
            .lines()
            .filter_map(|line| {
                let (x, y) = line.split_once(',')?;
                Some((x.parse::<i32>().ok()?, y.parse::<i32>().ok()?))
            })
            .collect();
        (coordinates, folds)
    } else {
        println!("Unrecognized input");
        return;
    };

    let mut answer1 = None;
    for fold in folds {
        coordinates = match fold {
            Fold::Y(at) => coordinates
                .iter()
                .map(|&(x, y)| {
                    let y = if y >= at { at - (y - at) } else { y };
                    (x, y)
                })
                .collect(),
            Fold::X(at) => coordinates
                .iter()
                .map(|&(x, y)| {
                    let x = if x >= at { at - (x - at) } else { x };
                    (x, y)
                })
                .collect(),
        };
        if answer1 == None {
            answer1 = Some(coordinates.len());
        }
    }

    let answer1 = answer1.unwrap_or_default();
    println!("Day 13 part 1: {answer1}");

    let mut c = coordinates.iter().copied().collect::<Vec<(i32, i32)>>();
    c.sort();
    let columns = c.iter().map(|x| x.0).max().unwrap_or_default() as usize + 1;
    let lines = c.iter().map(|x| x.1).max().unwrap_or_default() as usize + 1;
    let mut paper = vec![vec!['.'; columns]; lines];
    for (x, y) in c {
        paper[y as usize][x as usize] = '#';
    }

    let answer2 = paper
        .iter()
        .flat_map(|x| x.iter().chain(once(&'\n')))
        .collect::<String>();
    println!("Day 13 part 2:\n{answer2}");
}
