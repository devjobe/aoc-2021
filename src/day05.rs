use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn from(input: &str) -> Self {
        let (x, y) = input.split_once(",").expect("Unrecognized input");
        Self {
            x: x.parse().expect("Expected lhs integer"),
            y: y.parse().expect("Expected rhs integer"),
        }
    }
}

struct Line(Point, Point);

impl Line {
    fn from(input: &str) -> Self {
        let (lhs, rhs) = input.split_once(" -> ").expect("Unrecognized input");
        Line(Point::from(lhs), Point::from(rhs))
    }

    fn is_straight(&self) -> bool {
        self.0.x == self.1.x || self.0.y == self.1.y
    }

    fn direction(&self) -> Point {
        Point {
            x: (self.1.x - self.0.x).signum(),
            y: (self.1.y - self.0.y).signum(),
        }
    }

    fn point_count(&self) -> i32 {
        1 + if self.0.x == self.1.x {
            (self.0.y - self.1.y).abs()
        } else {
            (self.0.x - self.1.x).abs()
        }
    }

    fn points(&self) -> LinePoints {
        LinePoints {
            point: self.0,
            direction: self.direction(),
            count: self.point_count(),
        }
    }
}

struct LinePoints {
    point: Point,
    direction: Point,
    count: i32,
}

impl Iterator for LinePoints {
    type Item = Point;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count > 0 {
            let result = Some(self.point);
            self.point.x += self.direction.x;
            self.point.y += self.direction.y;
            self.count -= 1;
            result
        } else {
            None
        }
    }
}

pub fn run() {
    let input = include_str!("../inputs/day05.txt");

    let lines: Vec<_> = input.lines().map(Line::from).collect();

    let straight_lines = lines.iter().filter(|&x| Line::is_straight(x));

    let mut points: HashMap<Point, i32> = HashMap::new();
    let mut answer = 0;
    for point in straight_lines.map(Line::points).flatten() {
        let counter = points.entry(point).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            answer += 1;
        }
    }

    println!("Day 5 part 1: {answer}");

    let diagonal_lines = lines.iter().filter(|&x| !Line::is_straight(x));
    for point in diagonal_lines.map(Line::points).flatten() {
        let counter = points.entry(point).or_insert(0);
        *counter += 1;

        if *counter == 2 {
            answer += 1;
        }
    }

    println!("Day 5 part 2: {answer}");
}
