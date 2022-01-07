use std::{collections::HashMap, fs};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u64 {
    let opening_to_closing: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('<', '>'), ('[', ']')]);
    let closing_to_score: HashMap<char, u64> =
        HashMap::from([(')', 3), (']', 57), ('}', 1197), ('>', 25137)]);

    fs::read_to_string("day-10/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                if opening_to_closing.contains_key(&c) {
                    stack.push(c);
                } else if closing_to_score.contains_key(&c) {
                    match stack.pop() {
                        Some(value) => {
                            if c != opening_to_closing[&value] {
                                return closing_to_score[&c];
                            }
                        }
                        None => return closing_to_score[&c],
                    }
                }
            }
            0
        })
        .sum()
}

fn part2() -> u64 {
    let opening_to_closing: HashMap<char, char> =
        HashMap::from([('(', ')'), ('{', '}'), ('<', '>'), ('[', ']')]);
    let closing_to_score: HashMap<char, u64> =
        HashMap::from([(')', 1), (']', 2), ('}', 3), ('>', 4)]);

    let mut scores = fs::read_to_string("day-10/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| {
            let mut stack = Vec::new();
            for c in line.chars() {
                if opening_to_closing.contains_key(&c) {
                    stack.push(c);
                } else if closing_to_score.contains_key(&c) {
                    if let Some(value) = stack.pop() {
                        if c != opening_to_closing[&value] {
                            return 0;
                        }
                    }
                }
            }
            stack.iter().rev().fold(0, |acc, c| {
                acc * 5 + closing_to_score[&opening_to_closing[c]]
            })
        })
        .filter(|score| *score > 0)
        .collect::<Vec<_>>();

    scores.sort_unstable();
    scores[scores.len() / 2]
}
