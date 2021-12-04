use std::fs;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 1: {}", part2());
}

fn part1() -> usize {
    let measurements = load_measurements();

    count_increasing(&measurements, 1)
}

fn part2() -> usize {
    let measurements = load_measurements();

    count_increasing(&measurements, 3)
}

fn load_measurements() -> Vec<i32> {
    let measurements: Vec<i32> = fs::read_to_string("input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect();
    measurements
}

fn count_increasing(data: &[i32], window: usize) -> usize {
    data.iter()
        .zip(data.iter().skip(window))
        .filter(|(prev, actual)| actual > prev)
        .count()
}
