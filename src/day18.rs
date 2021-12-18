#[derive(Clone)]
enum Shell {
    Single(i32, usize),
    Double(i32, i32, usize),
}

fn split(num: &mut Vec<Shell>) -> bool {
    for i in 0..num.len() {
        match &num[i] {
            &Shell::Single(v, d) => {
                if v >= 10 {
                    num[i] = Shell::Double(v / 2, (v + 1) / 2, d + 1);
                    return true;
                }
            }
            &Shell::Double(a, b, d) => {
                if a >= 10 {
                    num[i] = Shell::Double(a / 2, (a + 1) / 2, d + 1);
                    num.insert(i + 1, Shell::Single(b, d));
                    return true;
                } else if b >= 10 {
                    num[i] = Shell::Single(a, d);
                    num.insert(i + 1, Shell::Double(b / 2, (b + 1) / 2, d + 1));
                    return true;
                }
            }
        }
    }
    return false;
}

fn explode(num: &mut Vec<Shell>) -> bool {
    for i in 0..num.len() {
        if let &Shell::Double(a, b, depth) = &num[i] {
            if depth > 4 {
                let mut exploded = Shell::Single(0, depth - 1);
                let mut replace = None;
                if i > 0 {
                    match &mut num[i - 1] {
                        Shell::Single(v, d) => {
                            *v += a;
                            if *d + 1 == depth {
                                exploded = Shell::Double(*v, 0, *d);
                                replace = Some(i - 1);
                            }
                        }
                        Shell::Double(_, v, _) => {
                            *v += a;
                        }
                    }
                }

                if i + 1 < num.len() {
                    match &mut num[i + 1] {
                        Shell::Single(v, d) => {
                            *v += b;
                            if *d + 1 == depth && replace == None {
                                exploded = Shell::Double(0, *v, *d);
                                replace = Some(i + 1);
                            }
                        }
                        Shell::Double(v, _, _) => {
                            *v += b;
                        }
                    }
                }
                if let Some(index) = replace {
                    num[index] = exploded;
                    num.remove(i);
                } else {
                    num[i] = Shell::Single(0, depth - 1);
                }
                return true;
            }
        }
    }
    return false;
}

fn reduce(num: &mut Vec<Shell>) {
    loop {
        if !(explode(num) || split(num)) {
            break;
        }
    }
}

fn parse_shell_number(line: &str) -> Vec<Shell> {
    let mut depth: usize = 0;
    let mut number: Vec<Shell> = Vec::new();
    let mut comma = false;
    let mut single = false;
    for ch in line.chars() {
        match ch {
            '[' => {
                depth += 1;
                single = false;
            }
            ']' => {
                depth -= 1;
                single = false;
            }
            ',' => {}
            '0'..='9' => {
                let v = ch as i32 - '0' as i32;
                if comma && single {
                    if let Some(&Shell::Single(n, d)) = number.last() {
                        if d != depth {
                            panic!("Expected depth to be the same.")
                        }
                        *number.last_mut().unwrap() = Shell::Double(n, v, d);
                    } else {
                        panic!("Expected single number.");
                    }
                } else {
                    number.push(Shell::Single(v, depth));
                    single = true;
                }
            }
            _ => {
                panic!("Unexpected character: {ch}")
            }
        }
        comma = ch == ',';
    }
    number
}

fn sum_shell_numbers(numbers: &Vec<Vec<Shell>>) -> Vec<Shell> {
    numbers.iter().skip(1).fold(
        numbers.first().cloned().unwrap_or_default(),
        |mut result, num| {
            result.extend(num.iter().cloned());
            for n in result.iter_mut() {
                match n {
                    Shell::Single(_, depth) => *depth += 1,
                    Shell::Double(_, _, depth) => *depth += 1,
                }
            }
            reduce(&mut result);
            result
        },
    )
}

fn add_shell_numbers(a: &[Shell], b: &[Shell]) -> Vec<Shell> {
    let mut vec: Vec<Shell> = a
        .iter()
        .chain(b.iter())
        .map(|shell| match shell {
            &Shell::Single(v, d) => Shell::Single(v, d + 1),
            &Shell::Double(a, b, d) => Shell::Double(a, b, d + 1),
        })
        .collect();
    reduce(&mut vec);
    vec
}

fn magnitude(tree: &[Shell], level: usize) -> (i32, &[Shell]) {
    if let Some(shell) = tree.first() {
        match shell {
            &Shell::Single(v, depth) => {
                if depth == level {
                    return (v, &tree[1..]);
                }
            }
            &Shell::Double(a, b, depth) => {
                if depth == level + 1 {
                    return (a * 3 + b * 2, &tree[1..]);
                }
            }
        }
        let (a, right_tree) = magnitude(tree, level + 1);
        let (b, remaining_tree) = magnitude(right_tree, level + 1);
        (a * 3 + b * 2, remaining_tree)
    } else {
        (0, &[])
    }
}

pub fn run() {
    let input = include_str!("../inputs/day18.txt");
    let numbers: Vec<Vec<Shell>> = input.lines().map(parse_shell_number).collect();
    let sum = sum_shell_numbers(&numbers);
    let (answer, _remaining) = magnitude(sum.as_slice(), 0);
    println!("{answer}");

    let mut answer2 = 0;
    let mut tail = numbers.iter();
    for a in numbers.iter() {
        tail.next();
        for b in tail.clone() {
            let (m, _) = magnitude(&add_shell_numbers(a, b), 0);
            answer2 = answer2.max(m);
            let (m, _) = magnitude(&add_shell_numbers(b, a), 0);
            answer2 = answer2.max(m);
        }
    }
    println!("{answer2}");
}
