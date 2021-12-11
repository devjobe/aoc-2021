fn advance(nums: &mut Vec<Vec<i32>>) -> i32 {
    let coords = (0..10).flat_map(|y| (0..10).map(move |x| (x, y)));
    let mut flashes = 0;
    let mut queue: Vec<(i32, i32)> = coords
        .filter(|&(x, y)| {
            let v = &mut nums[y as usize][x as usize];
            *v += 1;
            *v > 9
        })
        .collect();

    while let Some((x, y)) = queue.pop() {
        flashes += 1;
        for j in (y - 1).max(0)..=(y + 1) {
            for i in (x - 1).max(0)..=(x + 1) {
                if let Some(v) = nums
                    .get_mut(j as usize)
                    .and_then(|row| row.get_mut(i as usize))
                {
                    *v += 1;
                    if *v == 10 {
                        queue.push((i, j));
                    }
                }
            }
        }
    }
    nums.iter_mut().flatten().filter(|v| **v > 9).for_each(|v| {
        *v = 0;
    });
    flashes
}

pub fn run() {
    let input = include_str!("../inputs/day11.txt");
    let mut nums: Vec<Vec<i32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|ch| ch as i32 - '0' as i32)
                .collect::<Vec<_>>()
        })
        .collect();
    let mut flashes = 0;
    let mut synced_at = 0;
    for step in 0.. {
        if step >= 100 && synced_at != 0 {
            break;
        }
        let count = advance(&mut nums);
        if step < 100 {
            flashes += count;
        }
        if synced_at == 0 && count == 100 {
            synced_at = step + 1;
        }
    }
    println!("Day 11 part 1: {flashes}");
    println!("Day 11 part 2: {synced_at}");
}
