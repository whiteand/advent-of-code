use std::{
    collections::{BTreeMap, HashMap},
    fmt::Write,
};

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
            let next_position =
                std::iter::successors(D::next_pos(self, coord.0, coord.1), |(r, c)| {
                    D::next_pos(self, *r, *c)
                })
                .take_while(|(r, c)| self.get(&(*r, *c)).is_none())
                .last();
            if let Some((r, c)) = next_position {
                self.remove(coord);
                self.insert((r, c), Rock::Round);
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

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.get(&(row, col)).map(|r| char::from(*r)).unwrap_or('.');
                f.write_char(ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid({}x{})", self.rows, self.cols)?;
        writeln!(f, "{}", self)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}
impl From<Rock> for char {
    fn from(rock: Rock) -> Self {
        match rock {
            Rock::Round => 'O',
            Rock::Square => '#',
        }
    }
}
impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}

trait TiltDirection {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering;
    fn next_pos(grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)>;
}

macro_rules! direction {
    ($name:ident, $cmp:expr, $next_pos:expr,) => {
        struct $name;
        impl TiltDirection for $name {
            fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
                #[allow(clippy::redundant_closure_call)]
                $cmp(c1, c2)
            }
            fn next_pos(grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)> {
                #[allow(clippy::redundant_closure_call)]
                $next_pos(grid, row, col)
            }
        }
    };
}

direction! {
    North,
    (|c1: &(usize, usize), c2: &(usize, usize)| c1.cmp(c2)),
    (|_grid: &Grid, row: usize, col: usize| (row > 0).then(|| (row.saturating_sub(1), col))),
}
direction! {
    West,
    (|c1: &(usize, usize), c2: &(usize, usize)| c1.1.cmp(&c2.1)),
    (|_grid: &Grid, row: usize, col: usize| (col > 0).then_some((row, col.saturating_sub(1)))),
}
direction! {
    South,
    (|c1: &(usize, usize), c2: &(usize, usize)| c1.0.cmp(&c2.0).reverse()),
    (|grid: &Grid, row: usize, col: usize| (row < grid.rows - 1).then_some((row + 1, col))),
}
direction! {
    East,
    (|c1: &(usize, usize), c2: &(usize, usize)| c1.1.cmp(&c2.1).reverse()),
    (|grid: &Grid, row: usize, col: usize|
        (col < grid.cols - 1).then_some((row, col + 1))
    ),
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
