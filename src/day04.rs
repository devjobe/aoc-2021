enum Outcome {
    None,
    Bingo,
}

struct Board {
    numbers: [[i32; 5]; 5],
    marked: [[bool; 5]; 5],
    bingo: bool,
}

impl Board {
    fn from(input: &str) -> Self {
        let mut board = Self {
            numbers: [[0; 5]; 5],
            marked: [[false; 5]; 5],
            bingo: false,
        };

        let iter = input.lines().map(|line| {
            line.split_whitespace()
                .map(|x| i32::from_str_radix(x, 10).expect("A number"))
        });

        for (input_line, board_line) in iter.zip(board.numbers.iter_mut()) {
            for (input_value, value) in input_line.zip(board_line.iter_mut()) {
                *value = input_value;
            }
        }

        board
    }

    fn find_mut(&mut self, number: i32) -> Option<(usize, usize)> {
        for row_index in 0..self.numbers.len() {
            let row = &self.numbers[row_index];
            for column_index in 0..row.len() {
                if row[column_index] == number {
                    return Some((row_index, column_index));
                }
            }
        }
        None
    }

    fn mark(&mut self, number: i32) -> Outcome {
        if let Some((row, column)) = self.find_mut(number) {
            self.marked[row][column] = true;

            if self.marked[row].iter().all(|&x| x)
                || self.marked.iter().map(|m| m[column]).all(|x| x)
            {
                self.bingo = true;
                return Outcome::Bingo;
            }
        }

        Outcome::None
    }

    fn sum_nonmarked_values(&self) -> i32 {
        self.numbers
            .iter()
            .zip(self.marked.iter())
            .map(|(numbers, marked)| {
                numbers
                    .iter()
                    .zip(marked.iter())
                    .fold(0, |count, (&num, &marked)| {
                        count + if !marked { num } else { 0 }
                    })
            })
            .sum()
    }
}

pub fn run() {
    let input = include_str!("../inputs/day04.txt");

    let mut paragraphs = input.split("\r\n\r\n");

    let nums: Vec<_> = paragraphs
        .next()
        .expect("Expected first line.")
        .split(',')
        .map(|x| i32::from_str_radix(x, 10).expect("Expected number."))
        .collect();

    let mut boards: Vec<Board> = paragraphs.map(Board::from).collect();

    let mut last_score = None;

    for number in nums {
        for board in boards.iter_mut().filter(|board| !board.bingo) {
            if let Outcome::Bingo = board.mark(number) {
                let answer = board.sum_nonmarked_values() * number;
                if last_score == None {
                    println!("Day04 part 1: {answer}");
                }
                last_score = Some(answer);
            }
        }
    }

    if let Some(answer) = last_score {
        println!("Day04 part 2: {answer}");
    }
}
