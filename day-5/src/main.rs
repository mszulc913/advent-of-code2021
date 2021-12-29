use std::fs;

const BOARD_SIZE: isize = 1000;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

fn part1() -> isize {
    let mut map = vec![0; (BOARD_SIZE * BOARD_SIZE) as usize];
    let mut n_overlaps = 0;

    fs::read_to_string("day-5/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2, y2) = p2.split_once(',').unwrap();

            (
                x1.parse::<isize>().unwrap(),
                y1.parse::<isize>().unwrap(),
                x2.parse::<isize>().unwrap(),
                y2.parse::<isize>().unwrap(),
            )
        })
        .for_each(|(x1, y1, x2, y2)| {
            let mark = |val: &mut i32| {
                if *val == 1 {
                    n_overlaps += 1;
                }
                *val += 1;
            };
            let (start, end) = get_map_slice_indices(x1, y1, x2, y2);
            let slice = map[start as usize..=end as usize].as_mut();
            if y1 == y2 {
                slice.iter_mut().for_each(mark);
            } else if x1 == x2 {
                slice.iter_mut().step_by(BOARD_SIZE as usize).for_each(mark);
            }
        });
    n_overlaps
}

fn get_map_slice_indices(x1: isize, y1: isize, x2: isize, y2: isize) -> (isize, isize) {
    let (idx1, idx2) = (BOARD_SIZE * y1 + x1, BOARD_SIZE * y2 + x2);
    if idx1 > idx2 {
        (idx2, idx1)
    } else {
        (idx1, idx2)
    }
}

fn part2() -> isize {
    let mut map = vec![0; (BOARD_SIZE * BOARD_SIZE) as usize];
    let mut n_overlaps = 0;

    fs::read_to_string("day-5/input.txt")
        .expect("Unable to read input file.")
        .lines()
        .map(|line| {
            let (p1, p2) = line.split_once(" -> ").unwrap();
            let (x1, y1) = p1.split_once(',').unwrap();
            let (x2, y2) = p2.split_once(',').unwrap();

            (
                x1.parse::<isize>().unwrap(),
                y1.parse::<isize>().unwrap(),
                x2.parse::<isize>().unwrap(),
                y2.parse::<isize>().unwrap(),
            )
        })
        .for_each(|(x1, y1, x2, y2)| {
            let (start, end) = get_map_slice_indices(x1, y1, x2, y2);
            let step = if y1 == y2 {
                1
            } else if x1 == x2 {
                BOARD_SIZE
            } else {
                BOARD_SIZE + (x1 - x2).signum() * (y1 - y2).signum()
            };
            map[start as usize..=end as usize]
                .iter_mut()
                .step_by(step as usize)
                .for_each(|val| {
                    if *val == 1 {
                        n_overlaps += 1;
                    }
                    *val += 1;
                })
        });
    n_overlaps
}
