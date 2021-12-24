struct Vars {
    w: i64, x : i64, y : i64, z: i64
}

impl Vars {
    fn get(&self, v: &str) -> i64 {
        if let Ok(d) = v.parse::<i64>() {
            d
        } else {
            match v {
                "w" => self.w,
                "x" => self.x,
                "y" => self.y,
                "z" => self.z,
                _ => panic!("Unknown variable {v}"),
            }
        }
    }

    fn set(&mut self, v: &str, n: i64) {
        match v {
            "w" => self.w = n,
            "x" => self.x = n,
            "y" => self.y = n,
            "z" => self.z = n,
            _ => panic!("Unknown variable {v}"),
        }
    }
}

fn verify(program: &Vec<(&str, &str, &str)>, mut model: i64) -> bool {
    
    let mut vars: Vars = Vars { w: 0, x: 0, y: 0, z: 0 };
    let mut divisor = 10000000000000i64;


    for &(ins,a1,a2) in program.iter() {
        match ins {
            "inp" => {
                let d = model / divisor;
                model %= divisor;
                divisor /= 10;
                vars.set(a1, d);
            }
            "mul" => vars.set(a1, vars.get(a1) * vars.get(a2)),
            "add" => vars.set(a1, vars.get(a1) + vars.get(a2)),        
            "mod" => vars.set(a1, (vars.get(a1) % vars.get(a2)) as i64),
            "div" => vars.set(a1, (vars.get(a1) / vars.get(a2)) as i64),
            "eql" => vars.set(a1, (vars.get(a1) == vars.get(a2)) as i64),
            _=> panic!("Unknown instruction: {ins}")
        }
    }

    vars.z == 0
}

pub fn run() {
    let input = include_str!("../inputs/day24.txt");

    let program: Vec<(&str, &str, &str)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(' ').unwrap();

            if let Some((v, d)) = b.split_once(' ') {
                (a, v, d)
            } else {
                (a, b, "")
            }
            
        })
        .collect();

    let mut a = Vec::with_capacity(14);
    let mut b = Vec::with_capacity(14);
    let mut c = Vec::with_capacity(14);
    for j in 0..14 {
        a.push(program[j*18+4].2.parse::<i64>().unwrap());
        b.push(program[j*18+5].2.parse::<i64>().unwrap());
        c.push(program[j*18+15].2.parse::<i64>().unwrap());
    }


    let mut stack = Vec::new();

    let mut max_model_number = 0;
    let mut min_model_number = 0;

    for (index, divisor) in a.into_iter().enumerate() {
        if divisor == 1 {
            stack.push(index);
        } else {
            let dependency = stack.pop().unwrap();
            let difference = c[dependency] + b[index];

            let (x, y) = if difference < 0 {
                (9 + difference, 9)
            } else {
                (9, 9 - difference)
            };

            let (z, w) = if difference < 0 {
                (1, y-x+1)
            } else {
                (x-y+1, 1)
            };

            max_model_number += 10i64.pow((13 - index) as u32) * x;
            max_model_number += 10i64.pow((13 - dependency) as u32) * y;

            min_model_number += 10i64.pow((13 - index) as u32) * z;
            min_model_number += 10i64.pow((13 - dependency) as u32) * w;
        }
    }


    if verify(&program, max_model_number) {
        println!("Day 24 part 1: {max_model_number}");        
    } else {
        println!("The found maximum was not valid.");
    }
    

    if verify(&program, min_model_number) {
        println!("Day 24 part 2: {min_model_number}");        
    } else {
        println!("The found minimum was not valid.");
    }
}
