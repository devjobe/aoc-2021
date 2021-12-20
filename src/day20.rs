fn enhance(algorithm: &Vec<bool>, input: &Vec<Vec<bool>>, def: bool) -> Vec<Vec<bool>> {
    let width = input[0].len() as isize;
    let height = input.len() as isize;
    let pixel = |x, y| {
        if x < 0 || x >= width || y < 0 || y >= height {
            def
        } else {
            input[y as usize][x as usize]
        }
    };

    let decode = |x, y| {
        let mut index = 0;
        for j in (y - 1)..=(y + 1) {
            for i in (x - 1)..=(x + 1) {
                index = index << 1 | pixel(i, j) as usize;
            }
        }
        algorithm
            .get(index)
            .copied()
            .expect("Decoded index in range.")
    };

    let w = (width + 2) as usize;
    let h = (height + 2) as usize;
    let mut output = vec![vec![false; w]; h];
    for (y, line) in output.iter_mut().enumerate() {
        for (x, pixel) in line.iter_mut().enumerate() {
            *pixel = decode(x as isize - 1, y as isize - 1);
        }
    }
    output
}

pub fn run() {
    let input = include_str!("../inputs/day20.txt");
    let (algorithm, image) = input
        .split_once("\n\n")
        .map(|(algorithm, image)| {
            (
                algorithm
                    .trim()
                    .chars()
                    .map(|ch| if ch == '#' { true } else { false })
                    .collect::<Vec<bool>>(),
                image
                    .trim()
                    .lines()
                    .map(|line| {
                        line.chars()
                            .map(|ch| if ch == '#' { true } else { false })
                            .collect::<Vec<bool>>()
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .expect("Paragraph");

    let mut result = enhance(&algorithm, &image, false);
    let toggle0 = algorithm[0];
    let toggle1 = if toggle0 { algorithm[511] } else { false };
    let toggle = [toggle0, toggle1];
    result = enhance(&algorithm, &result, toggle[0]);
    let answer = result.iter().flatten().map(|&x| x as usize).sum::<usize>();
    println!("Day 20 part 1: {answer}");

    for step in 1..=48 {
        result = enhance(&algorithm, &result, toggle[step & 1]);
    }
    let answer2 = result.iter().flatten().map(|&x| x as usize).sum::<usize>();
    println!("Day 20 part 2: {answer2}");
}
