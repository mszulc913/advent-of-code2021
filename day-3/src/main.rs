use std::{cmp::Ordering, fs};

const CODE_LENGTH: usize = 12;
const NUM_CODES: usize = 1000;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let mut counts = [0; CODE_LENGTH];
    for line in fs::read_to_string("day-3/input.txt")
        .expect("Unable to read input file.")
        .lines()
    {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                counts[i] += 1;
            }
        }
    }
    let threshold = NUM_CODES / 2;
    let (gamma, epsilon) = counts
        .iter()
        .fold((0, 0), |(gamma_acc, epsilon_acc), &count| {
            if count > threshold {
                ((gamma_acc << 1) + 1, epsilon_acc << 1)
            } else {
                (gamma_acc << 1, (epsilon_acc << 1) + 1)
            }
        });

    gamma * epsilon
}

#[derive(Clone, Copy)]
enum Criteria {
    Oxygen,
    O2Scrubber,
}

fn part2() -> u32 {
    let input_bits = read_numbers();
    let input_bits_as_refs: &Vec<&Vec<u32>> = &(input_bits.iter().collect());
    let oxygen_generator_rating = read_rating(input_bits_as_refs, Criteria::Oxygen);
    let o2_scrubber_rating = read_rating(input_bits_as_refs, Criteria::O2Scrubber);

    oxygen_generator_rating * o2_scrubber_rating
}

fn read_numbers() -> Vec<Vec<u32>> {
    let mut numbers = vec![vec![0; CODE_LENGTH]; NUM_CODES];
    fs::read_to_string("day-3/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .enumerate()
        .for_each(|(i, line)| {
            line.chars().enumerate().for_each(|(j, c)| {
                if c == '1' {
                    numbers[i][j] = 1;
                }
            })
        });
    numbers
}

fn read_rating(numbers: &[&Vec<u32>], criteria: Criteria) -> u32 {
    let mut numbers_left = filter_numbers(numbers, 0, criteria);
    let mut i = 1;
    while numbers_left.len() != 1 {
        numbers_left = filter_numbers(&numbers_left, i, criteria);
        i += 1;
    }
    numbers_left[0].iter().fold(0, |acc, b| acc * 2 + b)
}

fn filter_numbers<'a>(
    numbers: &[&'a Vec<u32>],
    idx: usize,
    criteria: Criteria,
) -> Vec<&'a Vec<u32>> {
    let mut ones = Vec::new();
    let mut zeros = Vec::new();
    numbers.iter().for_each(|&number| {
        if number[idx] == 1 {
            ones.push(number);
        } else {
            zeros.push(number);
        }
    });

    match ones.len().cmp(&zeros.len()) {
        Ordering::Greater => match criteria {
            Criteria::Oxygen => ones,
            Criteria::O2Scrubber => zeros,
        },
        Ordering::Less => match criteria {
            Criteria::Oxygen => zeros,
            Criteria::O2Scrubber => ones,
        },
        Ordering::Equal => match criteria {
            Criteria::Oxygen => ones,
            Criteria::O2Scrubber => zeros,
        },
    }
}
