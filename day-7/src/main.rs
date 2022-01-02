use std::fs;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> i32 {
    let mut positions = fs::read_to_string("day-7/input.txt")
        .expect("Unable to read input file.")
        .split(',')
        .map(|c| c.parse().unwrap())
        .collect::<Vec<i32>>();
    let mid = positions.len() / 2;
    let (_, &mut final_pos, _) = positions.select_nth_unstable(mid);

    positions.iter().map(|&pos| (pos - final_pos).abs()).sum()
}

fn part2() -> i32 {
    let mut max_position = 0;
    let positions = fs::read_to_string("day-7/input.txt")
        .expect("Unable to read input file.")
        .split(',')
        .map(|c| c.parse().unwrap())
        .inspect(|&pos| {
            if pos > max_position {
                max_position = pos
            }
        })
        .collect::<Vec<i32>>();

    let costs = (0..=max_position)
        .scan(0, |acc, pos| {
            *acc += pos;
            Some(*acc)
        })
        .collect::<Vec<i32>>();

    (0..positions.len())
        .map(|i| {
            positions
                .iter()
                .map(|&pos| costs[(pos - i as i32).abs() as usize])
                .sum()
        })
        .min()
        .unwrap()
}
