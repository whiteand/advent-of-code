use std::collections::{BTreeMap, HashMap};

pub fn solve_task1(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);
    let mut coords = grid.round_rocks_coords();
    grid.tilt::<North>(&mut coords);
    grid.get_value(&coords)
}

pub fn solve_task2(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);
    let mut coords = grid.round_rocks_coords();
    let mut visited: HashMap<Vec<(usize, usize)>, usize> = HashMap::new();
    let mut results: Vec<usize> = Vec::new();
    let mut first_duplication = 0;
    let mut loop_start = 0;
    const ITERS: usize = 1000000000;
    for i in 0..ITERS {
        grid.tilt::<North>(&mut coords);
        grid.tilt::<West>(&mut coords);
        grid.tilt::<South>(&mut coords);
        grid.tilt::<East>(&mut coords);
        let key = coords.clone();
        if visited.contains_key(&key) {
            first_duplication = i;
            loop_start = visited.get(&key).copied().expect("Key not found");
            break;
        }
        visited.insert(key, i);
        results.push(grid.get_value(&coords));
    }
    let loop_len = first_duplication - loop_start;
    let ind = (ITERS - 1 - loop_start) % loop_len + loop_start;
    results[ind]
}

fn parse_grid(file_content: &str) -> Grid {
    file_content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(col, char)| match char {
                    '#' => Some((row, col, Rock::Square)),
                    'O' => Some((row, col, Rock::Round)),
                    '.' => None,
                    _ => panic!("Invalid char"),
                })
        })
        .collect()
}

struct Grid {
    map: BTreeMap<(usize, usize), Rock>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize, map: BTreeMap<(usize, usize), Rock>) -> Self {
        Self { rows, cols, map }
    }
    fn round_rocks_coords(&self) -> Vec<(usize, usize)> {
        self.keys()
            .copied()
            .filter(|(r, c)| matches!(self.map.get(&(*r, *c)), Some(Rock::Round)))
            .collect::<Vec<_>>()
    }
    fn tilt<D: TiltDirection>(&mut self, coords: &mut [(usize, usize)]) {
        coords.sort_unstable_by(D::cmp);
        for coord in coords {
            let next_position = D::next_pos(self, coord);
            if let Some((r, c)) = next_position {
                self.map.remove(coord);
                self.map.insert((r, c), Rock::Round);
                *coord = (r, c);
            }
        }
    }
    fn get_value(&self, coords: &[(usize, usize)]) -> usize {
        self.rows * coords.len() - coords.iter().map(|(r, _)| r).sum::<usize>()
    }
}

impl FromIterator<(usize, usize, Rock)> for Grid {
    fn from_iter<T: IntoIterator<Item = (usize, usize, Rock)>>(iter: T) -> Self {
        let mut map = BTreeMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (row, col, rock) in iter {
            if row >= rows {
                rows += 1
            }
            if col >= cols {
                cols += 1
            }
            map.insert((row, col), rock);
        }
        Self::new(rows, cols, map)
    }
}

impl std::ops::Deref for Grid {
    type Target = BTreeMap<(usize, usize), Rock>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}

trait TiltDirection {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering;
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)>;
}

struct North;

impl TiltDirection for North {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.cmp(c2)
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        (0..row)
            .rev()
            .map(|r| (r, col))
            .take_while(|coord| !grid.contains_key(coord))
            .last()
    }
}
struct West;

impl TiltDirection for West {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.1.cmp(&c2.1)
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        (0..col)
            .rev()
            .map(|c| (row, c))
            .take_while(|c| !grid.contains_key(c))
            .last()
    }
}
struct South;

impl TiltDirection for South {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.0.cmp(&c2.0).reverse()
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        ((row + 1)..grid.rows)
            .map(|r| (r, col))
            .take_while(|r| !grid.contains_key(r))
            .last()
    }
}
struct East;

impl TiltDirection for East {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.1.cmp(&c2.1).reverse()
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        ((col + 1)..grid.cols)
            .map(|c| (row, c))
            .take_while(|c| !grid.contains_key(c))
            .last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d14/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d14.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "136");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "106997");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "64");
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "99641");
    }
}
