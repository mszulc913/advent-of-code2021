use std::fs;

const BOARD_SIZE: usize = 5;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

struct Board {
    rows: Vec<Vec<i32>>,
    sum: i32,
}

impl Board {
    pub fn new(rows: Vec<Vec<i32>>) -> Board {
        let sum = rows.iter().map::<i32, _>(|row| row.iter().sum()).sum();
        Board { rows, sum }
    }

    pub fn mark(&mut self, draw: i32) -> Option<i32> {
        for row in self.rows.iter_mut() {
            for value in row.iter_mut() {
                if *value == draw {
                    *value = -1;
                    self.sum -= draw;
                }
            }
        }
        self.calculate_score(draw)
    }

    fn calculate_score(&mut self, draw: i32) -> Option<i32> {
        let mut marked: usize;
        let board_size = self.rows.len();
        for row in self.rows.iter() {
            marked = 0;
            for &val in row.iter() {
                if val == -1 {
                    marked += 1;
                }
            }
            if marked == board_size {
                return Some(self.sum * draw);
            }
        }

        for col_idx in 0..board_size {
            marked = 0;
            for row in self.rows.iter() {
                if row[col_idx] == -1 {
                    marked += 1;
                }
            }
            if marked == board_size {
                return Some(self.sum * draw);
            }
        }

        None
    }
}

fn part1() -> i32 {
    let (draws, mut boards) = read_input();

    for &draw in draws.iter() {
        for board in boards.iter_mut() {
            if let Some(score) = board.mark(draw) {
                return score;
            }
        }
    }
    panic!("No solution found!")
}

fn part2() -> i32 {
    let (draws, mut boards) = read_input();
    let mut completed_boards: Vec<usize> = Vec::new();
    let boards_count = boards.len();
    for &draw in draws.iter() {
        for (i, board) in boards.iter_mut().enumerate() {
            if completed_boards.contains(&i) {
                continue;
            }
            if let Some(score) = board.mark(draw) {
                if completed_boards.len() == boards_count - 1 {
                    return score;
                }
                completed_boards.push(i);
            }
        }
    }
    panic!("No solution found!")
}

fn read_input() -> (Vec<i32>, Vec<Board>) {
    let input_lines = fs::read_to_string("day-4/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    let draws: Vec<i32> = input_lines[0]
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    let boards: Vec<Board> = input_lines[2..]
        .chunks(BOARD_SIZE + 1)
        .map(|board_lines| {
            Board::new(
                board_lines
                    .iter()
                    .filter(|&line| !line.is_empty())
                    .map(|line| {
                        line.split_whitespace()
                            .map(|number| number.parse().unwrap())
                            .collect()
                    })
                    .collect(),
            )
        })
        .collect();

    (draws, boards)
}
