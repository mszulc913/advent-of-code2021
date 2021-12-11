use std::fs;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let (horizontal_final, depth_final) = fs::read_to_string("input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold((0, 0), |(horizontal, depth), (command, value_str)| {
            let value = value_str.parse::<i32>().unwrap();
            match command {
                "forward" => (horizontal + value, depth),
                "up" => (horizontal, depth - value),
                "down" => (horizontal, depth + value),
                _ => (horizontal, depth),
            }
        });
    horizontal_final * depth_final
}

fn part2() -> i32 {
    let (horizontal_final, depth_final, _) = fs::read_to_string("input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .fold(
            (0, 0, 0),
            |(horizontal, depth, aim), (command, value_str)| {
                let value = value_str.parse::<i32>().unwrap();
                match command {
                    "forward" => (horizontal + value, depth + value * aim, aim),
                    "up" => (horizontal, depth, aim - value),
                    "down" => (horizontal, depth, aim + value),
                    _ => (horizontal, depth, aim),
                }
            },
        );
    horizontal_final * depth_final
}
