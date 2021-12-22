use std::ops::Range;

#[derive(Clone)]
struct Cuboid {
    x: Range<i32>,
    y: Range<i32>,
    z: Range<i32>,
    state: bool,
}

impl Cuboid {
    fn from(line: &str) -> Cuboid {
        line.split_once(' ')
            .map(|(a, b)| {
                let state = a == "on";
                let mut ranges = b.split(',').map(|coords| {
                    coords[2..]
                        .split_once("..")
                        .map(|(start, end)| {
                            let s = start.parse::<i32>().expect("starting range");
                            let e = end.parse::<i32>().expect("ending range");
                            let r = if s > e { e..(s + 1) } else { s..(e + 1) };
                            r
                        })
                        .expect("range")
                });

                let x = ranges.next().expect("x");
                let y = ranges.next().expect("y");
                let z = ranges.next().expect("z");
                Cuboid { x, y, z, state }
            })
            .expect("cuboid")
    }

    fn all_within_50(&self) -> Cuboid {
        fn subrange(r: &Range<i32>) -> Range<i32> {
            let s = (r.start).max(-50);
            let e = (r.end).min(51);
            if e < s {
                0..0
            } else {
                s..e
            }
        }

        let x = subrange(&self.x);
        let y = subrange(&self.y);
        let z = subrange(&self.z);
        Cuboid {
            x,
            y,
            z,
            state: self.state,
        }
    }

    fn intersection(&self, other: &Self) -> Cuboid {
        fn inter(a: &Range<i32>, b: &Range<i32>) -> Range<i32> {
            let min = a.start.max(b.start);
            let max = a.end.min(b.end);

            if min >= max {
                0..0
            } else {
                min..max
            }
        }
        let x = inter(&self.x, &other.x);
        let y = inter(&self.y, &other.y);
        let z = inter(&self.z, &other.z);
        Cuboid {
            x,
            y,
            z,
            state: other.state,
        }
    }

    fn count(&self) -> usize {
        self.x.len() * self.y.len() * self.z.len()
    }
}

fn exclude(active: &mut Vec<Cuboid>, cuboid: &Cuboid) {
    for index in (0..active.len()).rev() {
        let intersection = active[index].intersection(cuboid);
        let count = intersection.count();
        if count == 0 {
            continue;
        }
        let mut current = active.swap_remove(index);
        if count == current.count() {
            continue;
        }

        if intersection.x.start > current.x.start {
            let mut left = current.clone();
            left.x.end = intersection.x.start;
            current.x.start = intersection.x.start;
            active.push(left);
        }

        if intersection.x.end < current.x.end {
            let mut right = current.clone();
            right.x.start = intersection.x.end;
            current.x.end = intersection.x.end;
            active.push(right);
        }

        if intersection.y.start > current.y.start {
            let mut front = current.clone();
            front.y.end = intersection.y.start;
            current.y.start = intersection.y.start;
            active.push(front);
        }

        if intersection.y.end < current.y.end {
            let mut back = current.clone();
            back.y.start = intersection.y.end;
            current.y.end = intersection.y.end;
            active.push(back);
        }
        if intersection.z.start > current.z.start {
            let mut bottom = current.clone();
            bottom.z.end = intersection.z.start;
            current.z.start = intersection.z.start;
            active.push(bottom);
        }

        if intersection.z.end < current.z.end {
            let mut top = current.clone();
            top.z.start = intersection.z.end;
            current.z.end = intersection.z.end;
            active.push(top);
        }
    }
}

fn count_enabled<'a, I: Iterator<Item = Cuboid>>(cuboids: I) -> usize {
    let mut active = Vec::new();
    for cuboid in cuboids {
        exclude(&mut active, &cuboid);
        if cuboid.state {
            active.push(cuboid)
        }
    }
    active.iter().map(|x| x.count()).sum::<usize>()
}

pub fn run() {
    let input = include_str!("../inputs/day22.txt");
    let cuboids: Vec<_> = input.lines().map(Cuboid::from).collect();
    let answer = count_enabled(cuboids.iter().map(Cuboid::all_within_50));
    println!("Day 22 part 1: {answer}");
    let answer = count_enabled(cuboids.into_iter());
    println!("Day 22 part 2: {answer}");
}
