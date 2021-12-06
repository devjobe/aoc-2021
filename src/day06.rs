pub fn run() {
    let input = include_str!("../inputs/day06.txt");
    let fishes: Vec<_> = input.split(',').map(str::parse::<i64>).collect::<Result<Vec<i64>, _>>().expect("Numbers");

    let mut frequencies = vec![0;9];

    for n in fishes {
        assert!((0..7).contains(&n));
        *frequencies.get_mut(n as usize).expect("Number in range.") += 1i64;
    }

    let g = frequencies.len();
    for n in 0..80 {
        frequencies[(n+7)%g] += frequencies[n % g];
    }

    let sum : i64 = frequencies.iter().sum();
    println!("Day 6 part 1: {sum}");

    for n in 80..256 {
        frequencies[(n+7)%g] += frequencies[n % g];
    }

    let sum : i64 = frequencies.iter().sum();
    println!("Day 6 part 2: {sum}");
}
