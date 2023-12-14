use std::{
    collections::{btree_map, BTreeMap},
    fmt::Write,
};

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
struct Grid {
    map: BTreeMap<(usize, usize), Rock>,
    rows: usize,
    cols: usize,
}
trait TiltDirection {
    fn get_coords(grid: &Grid) -> Vec<(usize, usize)>;
    fn next_pos(grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)>;
}

struct North;

impl TiltDirection for North {
    fn get_coords(grid: &Grid) -> Vec<(usize, usize)> {
        grid.keys()
            .filter(|(r, c)| *r > 0 && matches!(grid.get(&(*r, *c)), Some(Rock::Round)))
            .copied()
            .collect::<Vec<_>>()
    }
    fn next_pos(_grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)> {
        (row > 0).then(|| (row.saturating_sub(1), col))
    }
}

struct West;

impl TiltDirection for West {
    fn get_coords(grid: &Grid) -> Vec<(usize, usize)> {
        let mut res = grid
            .keys()
            .filter(|(r, c)| *c > 0 && matches!(grid.get(&(*r, *c)), Some(Rock::Round)))
            .copied()
            .collect::<Vec<_>>();
        res.sort_by(|(_, c1), (_, c2)| c1.cmp(c2));
        res
    }
    fn next_pos(_grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)> {
        (col > 0).then_some((row, col.saturating_sub(1)))
    }
}

struct South;

impl TiltDirection for South {
    fn get_coords(grid: &Grid) -> Vec<(usize, usize)> {
        let mut res = grid
            .keys()
            .filter(|(r, c)| *r < grid.rows - 1 && matches!(grid.get(&(*r, *c)), Some(Rock::Round)))
            .copied()
            .collect::<Vec<_>>();
        res.sort_by(|(r1, _), (r2, _)| r1.cmp(r2).reverse());
        res
    }
    fn next_pos(grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)> {
        (row < grid.rows - 1).then_some((row + 1, col))
    }
}

struct East;

impl TiltDirection for East {
    fn get_coords(grid: &Grid) -> Vec<(usize, usize)> {
        let mut res = grid
            .keys()
            .filter(|(r, c)| *c < grid.cols - 1 && matches!(grid.get(&(*r, *c)), Some(Rock::Round)))
            .copied()
            .collect::<Vec<_>>();
        res.sort_by(|(_, c1), (_, c2)| c1.cmp(c2).reverse());
        res
    }
    fn next_pos(grid: &Grid, row: usize, col: usize) -> Option<(usize, usize)> {
        (col < grid.cols - 1).then_some((row, col + 1))
    }
}

impl Grid {
    fn new(rows: usize, cols: usize, map: BTreeMap<(usize, usize), Rock>) -> Self {
        Self { rows, cols, map }
    }
    fn tilt<D: TiltDirection>(&mut self) {
        let coord = D::get_coords(self);
        for (row, col) in coord {
            let rock = match self.get(&(row, col)).copied().expect("Rock not found") {
                Rock::Round => Rock::Round,
                Rock::Square => {
                    continue;
                }
            };
            let next_position = std::iter::successors(D::next_pos(self, row, col), |(row, col)| {
                D::next_pos(self, *row, *col)
            })
            .take_while(|(r, c)| self.get(&(*r, *c)).is_none())
            .last();
            if let Some((r, c)) = next_position {
                self.remove(&(row, col));
                self.insert((r, c), rock);
            }
        }
    }
    fn get_value(&self) -> usize {
        self.map
            .iter()
            .map(|((row, _), rock)| match rock {
                Rock::Round => self.rows - *row,
                Rock::Square => 0,
            })
            .sum()
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

pub fn solve_task1(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);
    grid.tilt::<North>();
    grid.get_value()
}
pub fn solve_task2(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);

    let mut memory: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    let mut first_duplication = 0;
    let mut loop_start = 0;
    const ITERS: usize = 1000000000;
    for i in 0..ITERS {
        grid.tilt::<North>();
        grid.tilt::<West>();
        grid.tilt::<South>();
        grid.tilt::<East>();
        let key = format!("{grid}");
        match memory.entry(key) {
            btree_map::Entry::Vacant(e) => {
                e.insert((i, grid.get_value()));
            }
            btree_map::Entry::Occupied(e) => {
                loop_start = e.get().0;
                first_duplication = i;
                break;
            }
        }
    }
    let loop_len = first_duplication - loop_start;
    let mut loop_map = memory
        .into_values()
        .filter(|(i, _)| *i >= loop_start)
        .map(|(i, v)| (i % loop_len, v))
        .collect::<BTreeMap<_, _>>();

    loop_map
        .remove(&((ITERS - 1) % loop_len))
        .expect("Not found")
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
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "99641");
    }
}
