use std::fs;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn count_fish_after_days(n_days: usize) -> usize {
    let mut num_fishes_per_timer = fs::read_to_string("day-6/input.txt")
        .expect("Unable to read input file.")
        .split(',')
        .fold([0; 9], |mut acc, x| {
            acc[x.parse::<usize>().unwrap()] += 1;
            acc
        });
    (0..n_days).for_each(|i| num_fishes_per_timer[(i + 7) % 9] += num_fishes_per_timer[i % 9]);

    num_fishes_per_timer.iter().sum()
}

fn part1() -> usize {
    count_fish_after_days(80)
}

fn part2() -> usize {
    count_fish_after_days(256)
}
