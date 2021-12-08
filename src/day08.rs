use std::collections::HashMap;

fn decode_line(line: &str) -> i64 {
    let (numbers, decode) = line.split_once(" | ").expect("expected delimited line");

    let mut lookup = HashMap::new();

    let mut acf: String = String::new();
    let mut bcdf: String = String::new();

    for d in numbers.split(' ').map(|d| {
        let mut s = d.chars().collect::<Vec<_>>();
        s.sort();
        s.iter().collect::<String>()
    }) {
        let v = match d.len() {
            2 => 1,
            3 => {
                acf = d.clone();
                7
            }
            4 => {
                bcdf = d.clone();
                4
            }
            7 => 8,
            _ => 0,
        };
        lookup.entry(d).or_insert(v);
    }

    let mut c = 'x';
    let mut e = 'z';

    for (key, value) in lookup.iter_mut().filter(|(k, _v)| k.len() == 6) {
        let u1 = {
            let mut it = "abcdefg".chars().filter(|&c| !key.contains(c));
            it.next().expect("Expected unfilled")
        };

        if acf.contains(u1) {
            *value = 6;
            c = u1;
        } else if bcdf.contains(u1) {
            *value = 0; // u1 = d
        } else {
            *value = 9;
            e = u1;
        }
    }

    for (key, value) in lookup.iter_mut().filter(|(k, _v)| k.len() == 5) {
        let (u1, u2) = {
            let mut it = "abcdefg".chars().filter(|&c| !key.contains(c));
            (
                it.next().expect("Expected unfilled"),
                it.next().expect("Expected unfilled"),
            )
        };

        if u1 == e || u2 == e {
            if u1 == c || u2 == c {
                *value = 5;
            } else {
                *value = 3;
            }
        } else {
            *value = 2;
        }
    }

    decode
        .split(' ')
        .map(|d| {
            let mut s = d.chars().collect::<Vec<_>>();
            s.sort();
            s.iter().collect::<String>()
        })
        .map(|x| lookup.get(&x).expect("Unexpected value decoded."))
        .fold(0i64, |n, k| n * 10 + k)
}

pub fn run() {
    let input = include_str!("../inputs/day08.txt");

    {
        let answer: i32 = input
            .lines()
            .map(|x| {
                x.split_once(" | ")
                    .expect("expected delimited line")
                    .1
                    .split(' ')
            })
            .flatten()
            .map(|x| match x.len() {
                2 | 3 | 4 | 7 => 1,
                _ => 0,
            })
            .sum();
        println!("Day 1 part 1 {answer}");
    }

    {
        let answer: i64 = input.lines().map(decode_line).sum();
        println!("Day 1 part 2 {answer}");
    }
}
