use std::{collections::BTreeSet, fs};

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> usize {
    fs::read_to_string("day-8/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map::<usize, _>(|line| {
            line.split_once("| ")
                .unwrap()
                .1
                .split_whitespace()
                .map(|digit| {
                    let len = digit.len();
                    if len == 2 || len == 3 || len == 4 || len == 7 {
                        1
                    } else {
                        0
                    }
                })
                .sum()
        })
        .sum()
}

fn part2() -> usize {
    fs::read_to_string("day-8/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map::<usize, _>(|line| {
            let (patterns_str, output_str) = line.split_once("| ").unwrap();
            let patterns = extract_codes(patterns_str, true);
            let output_values = extract_codes(output_str, false);
            decode_line(patterns.as_ref(), output_values.as_ref())
        })
        .sum()
}

fn extract_codes(values_str: &str, sort: bool) -> Vec<BTreeSet<char>> {
    let mut patterns: Vec<BTreeSet<char>> = values_str
        .split_whitespace()
        .map(|value| value.chars().collect())
        .collect();
    if sort {
        patterns.sort_unstable_by_key(|a| a.len());
    }
    patterns
}

fn decode_line(patterns: &[BTreeSet<char>], output_values: &[BTreeSet<char>]) -> usize {
    // patterns are sorted by len
    let one = &patterns[0];
    let four = &patterns[2];
    let seven = &patterns[1];
    let b_and_d: BTreeSet<char> = four.difference(seven).copied().collect();
    output_values
        .iter()
        .map(|value| {
            match value.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                5 => {
                    // only 2, 3 and 5 are possible now
                    if value.is_superset(&b_and_d) {
                        5
                    } else if value.is_superset(one) {
                        3
                    } else {
                        2
                    }
                }
                _ => {
                    // only 0, 6 and 9 are possible now
                    if value.is_superset(four) {
                        9
                    } else if value.is_superset(&b_and_d) {
                        6
                    } else {
                        0
                    }
                }
            }
        })
        .fold(0, |acc, code| 10 * acc + code)
}
