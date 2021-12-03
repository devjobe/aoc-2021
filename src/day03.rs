use std::cmp::Ordering;

pub fn run() {
    let input = include_str!("../inputs/day03.txt");

    let list: Vec<u32> = input
        .lines()
        .map(|x| u32::from_str_radix(x, 2).unwrap_or_default())
        .collect();

    let (gamma, epsilon) = (0..12).fold((0, 0), |(gamma, epsilon), n| {
        let bit = 1 << n;
        let balance: i32 = (0..list.len())
            .map(|idx| {
                if list.get(idx).copied().unwrap_or_default() & bit != 0 {
                    1
                } else {
                    -1
                }
            })
            .sum();

        if balance >= 0 {
            (gamma | bit, epsilon)
        } else {
            (gamma, epsilon | bit)
        }
    });

    {
        let answer = gamma * epsilon;
        println!("Day 3 part 1: {answer}");
    }

    let mut oxygen = Vec::new();
    let mut co2 = Vec::new();

    {
        let bit = 1 << 11;
        let (most_common, least_common) = (gamma & bit, epsilon & bit);

        for (idx, value) in list.iter().enumerate() {
            let v = value & bit;
            if v == most_common {
                oxygen.push(idx);
            }
            if v == least_common {
                co2.push(idx);
            }
        }
    }

    fn filter_indices(list: &Vec<u32>, indices: &mut Vec<usize>, ordering: Ordering) {
        for n in (0..11).rev() {
            if indices.len() == 1 {
                break;
            }
            let bit = 1 << n;
            let balance: i32 = indices
                .iter()
                .map(|&idx| {
                    if list.get(idx).copied().unwrap_or_default() & bit != 0 {
                        1
                    } else {
                        -1
                    }
                })
                .sum();

            let value = match balance.cmp(&0) {
                Ordering::Equal => {
                    if ordering == Ordering::Greater {
                        bit
                    } else {
                        0
                    }
                }
                o => {
                    if ordering == o {
                        bit
                    } else {
                        0
                    }
                }
            };

            *indices = indices
                .iter()
                .filter(|&&idx| list.get(idx).copied().unwrap_or_default() & bit == value)
                .copied()
                .collect();
        }
    }

    filter_indices(&list, &mut oxygen, Ordering::Greater);
    filter_indices(&list, &mut co2, Ordering::Less);

    let gamma = list[oxygen.first().copied().unwrap_or_default()];
    let epsilon = list[co2.first().copied().unwrap_or_default()];
    let answer = gamma * epsilon;
    println!("Day 3 part 2: {answer}");
}
