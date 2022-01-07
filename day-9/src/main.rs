use std::fs;

const DIRECTIONS: [(isize, isize); 4] = [(0, 1), (1, 0), (-1, 0), (0, -1)];

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

struct Heightmap {
    data: Vec<Vec<u32>>,
}

impl Heightmap {
    pub fn new(data: Vec<Vec<u32>>) -> Heightmap {
        Heightmap { data }
    }

    pub fn get(&self, x: isize, y: isize) -> Option<u32> {
        if x >= self.data[0].len() as isize || y >= self.data.len() as isize || x < 0 || y < 0 {
            None
        } else {
            Some(self.data[y as usize][x as usize])
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.data[0].len(), self.data.len())
    }
}

fn read_heightmap() -> Heightmap {
    let data = fs::read_to_string("day-9/input.txt")
        .expect("Unable to read input file.")
        .split_whitespace()
        .map(|x| x.chars().map(|char| char.to_digit(10).unwrap()).collect())
        .collect();

    Heightmap::new(data)
}

fn part1() -> u32 {
    let heightmap = read_heightmap();
    let mut risk = 0;
    let (n_cols, n_rows) = heightmap.size();
    for x in 0isize..n_cols as isize {
        for y in 0isize..n_rows as isize {
            if is_low_point(x, y, &heightmap) {
                risk += heightmap.get(x, y).unwrap() + 1;
            }
        }
    }
    risk
}

fn is_low_point(x: isize, y: isize, heightmap: &Heightmap) -> bool {
    let value = heightmap.get(x, y).unwrap();
    DIRECTIONS
        .iter()
        .all(|(dx, dy)| value < heightmap.get(x + dx, y + dy).unwrap_or(9))
}

fn part2() -> u32 {
    let heightmap = read_heightmap();
    let mut basins_sizes: Vec<u32> = Vec::new();
    let (n_cols, n_rows) = heightmap.size();
    let mut visited = (0..n_rows)
        .map(|_| (0..n_cols).map(|_| false).collect())
        .collect::<Vec<Vec<_>>>();
    for x in 0isize..n_cols as isize {
        for y in 0isize..n_rows as isize {
            if is_low_point(x, y, &heightmap) {
                basins_sizes.push(get_basin_size(x, y, &heightmap, &mut visited))
            }
        }
    }
    basins_sizes.sort_unstable();
    basins_sizes.iter().rev().take(3).product()
}

fn get_basin_size(x: isize, y: isize, heightmap: &Heightmap, visited: &mut Vec<Vec<bool>>) -> u32 {
    let mut size = 1;
    let value = heightmap.get(x, y).unwrap();
    let mut stack = Vec::new();
    push_neighbors(&mut stack, x, y, value, visited);

    while let Some((neighbor_x, neighbor_y, prev_value)) = stack.pop() {
        if let Some(neighbor_value) = heightmap.get(neighbor_x, neighbor_y) {
            if !visited[neighbor_y as usize][neighbor_x as usize]
                && neighbor_value < 9
                && neighbor_value > prev_value
            {
                visited[neighbor_y as usize][neighbor_x as usize] = true;
                push_neighbors(&mut stack, neighbor_x, neighbor_y, neighbor_value, visited);
                size += 1;
            }
        }
    }
    size
}

fn push_neighbors(
    stack: &mut Vec<(isize, isize, u32)>,
    x: isize,
    y: isize,
    value: u32,
    visited: &[Vec<bool>],
) {
    for (dx, dy) in DIRECTIONS.iter() {
        let (x, y) = (x + dx, y + dy);
        if let Some(is_point_visited) = visited.get(y as usize).and_then(|row| row.get(x as usize))
        {
            if !is_point_visited {
                stack.push((x, y, value));
            }
        }
    }
}
