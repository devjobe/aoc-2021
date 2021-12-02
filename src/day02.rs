pub fn run() {
    let input = include_str!("../inputs/day02.txt");

    let list: Vec<(i32, i32)> = input
        .lines()
        .map(|x| {
            let mut it = x.split_whitespace();
            (it.next().unwrap_or_default(), it.next().unwrap_or_default())
        })
        .map(|(instruction, amount)| {
            let n: i32 = amount.parse().unwrap_or_default();
            match instruction {
                "forward" => (n, 0),
                "up" => (0, -n),
                "down" => (0, n),
                _ => {
                    println!("Unknown instruction: {instruction}");
                    (0, 0)
                }
            }
        })
        .collect();

    {
        let (x, y) = list
            .iter()
            .fold((0, 0), |(x0, y0), (x1, y1)| (x0 + x1, y0 + y1));
        let answer = x * y;
        println!("Day 2 part 1: {answer}");
    }

    {
        let (x, y, _) = list.iter().fold((0, 0, 0), |(x0, y0, aim), (x1, y1)| {
            (x0 + x1, y0 + x1 * aim, aim + y1)
        });
        let answer = x * y;
        println!("Day 2 part 2: {answer}");
    }
}
