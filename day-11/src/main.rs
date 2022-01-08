use std::fs;

const ADJACENT_DIRS: [(isize, isize); 8] = [
    (0, 1),
    (1, 0),
    (0, -1),
    (-1, 0),
    (1, 1),
    (-1, -1),
    (1, -1),
    (-1, 1),
];

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> u32 {
    let mut flashes = 0;
    let mut map = read_map();
    for _ in 0..100 {
        flashes += run_iteration(&mut map);
    }
    flashes
}

fn run_iteration(map: &mut Vec<Vec<u32>>) -> u32 {
    let mut flashes = 0;
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            map[y][x] += 1;
        }
    }
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            if map[y][x] > 9 {
                flashes += flash(x, y, map);
            }
        }
    }
    flashes
}

fn read_map() -> Vec<Vec<u32>> {
    let map = fs::read_to_string("day-11/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect::<Vec<Vec<_>>>();
    map
}

fn flash(x: usize, y: usize, map: &mut Vec<Vec<u32>>) -> u32 {
    map[y][x] = 0;
    ADJACENT_DIRS.iter().fold(1, |acc, (dx, dy)| {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;
        if new_x >= 0
            && new_y >= 0
            && (new_x as usize) < map[0].len()
            && (new_y as usize) < map.len()
        {
            let value = &mut map[new_y as usize][new_x as usize];
            if *value > 8 {
                acc + flash(new_x as usize, new_y as usize, map)
            } else if *value > 0 {
                *value += 1;
                acc
            } else {
                acc
            }
        } else {
            acc
        }
    })
}

fn part2() -> u32 {
    let mut map = read_map();
    let mut flashes = 0;
    let map_size = map.len() * map[0].len();
    let mut i = 0;
    while flashes != map_size as u32 {
        flashes = run_iteration(&mut map);
        i += 1;
    }
    i
}
