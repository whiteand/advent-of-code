use std::collections::{BTreeMap, VecDeque};

trait Field {
    fn is_empty(&self, coord: (isize, isize)) -> bool;
    fn get_start(&self) -> (isize, isize);
}

struct Grid {
    cells: Vec<Vec<bool>>,
    start: (usize, usize),
}

impl Field for Grid {
    fn is_empty(&self, coord: (isize, isize)) -> bool {
        if coord.0 < 0 || coord.1 < 0 {
            return false;
        }
        let row = coord.0 as usize;
        let col = coord.1 as usize;
        if row >= self.cells.len() {
            return false;
        }
        if col >= self.cells[0].len() {
            return false;
        }
        !self.cells[row][col]
    }

    fn get_start(&self) -> (isize, isize) {
        (self.start.0 as isize, self.start.1 as isize)
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start_row, start_col) = self.start;
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i == start_row && j == start_col {
                    write!(f, "S")?;
                } else if *cell {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Infinite(Grid);

impl Infinite {
    fn get_row(&self, row: isize) -> usize {
        let rows = self.0.cells.len();
        row.rem_euclid(rows as isize) as usize
    }
    fn get_col(&self, col: isize) -> usize {
        let cols = self.0.cells[0].len();
        col.rem_euclid(cols as isize) as usize
    }
    fn get_coords(&self, coord: (isize, isize)) -> (usize, usize) {
        (self.get_row(coord.0), self.get_col(coord.1))
    }
}

impl Field for Infinite {
    fn is_empty(&self, coord: (isize, isize)) -> bool {
        let (r, c) = self.get_coords(coord);
        self.0.is_empty((r as isize, c as isize))
    }
    fn get_start(&self) -> (isize, isize) {
        self.0.get_start()
    }
}

impl std::fmt::Debug for Infinite {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_r = -(self.0.cells.len() as isize);
        let max_r = (self.0.cells.len() * 2) as isize;
        let min_c = -(self.0.cells[0].len() as isize);
        let max_c = (self.0.cells[0].len() * 2) as isize;

        for i in min_r..=max_r {
            for j in min_c..=max_c {
                if self.is_empty((i, j)) {
                    write!(f, ".")?;
                } else {
                    write!(f, "#")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    for (row, line) in input.lines().enumerate() {
        let mut cells = Vec::with_capacity(line.len());
        for (col, cell) in line.chars().enumerate() {
            match cell {
                '.' => cells.push(false),
                '#' => cells.push(true),
                'S' => {
                    cells.push(false);
                    start = (row, col);
                }
                _ => panic!("invalid input"),
            }
        }
        grid.push(cells);
    }
    Grid { cells: grid, start }
}

struct Counter {
    total: usize,
    current_distance: usize,
    current_count: usize,
    max_distance: usize,
}

impl Counter {
    fn new(max_distance: usize) -> Self {
        Self {
            current_distance: 0,
            total: 0,
            current_count: 0,
            max_distance,
        }
    }
    fn push(&mut self, distance: usize) {
        if self.current_distance == distance {
            self.current_count += 1;
            return;
        }

        if self.current_distance % 2 == self.max_distance % 2 {
            self.total += self.current_count;
            self.current_count = 1;
            self.current_distance = distance;
            return;
        }
        self.current_count = 1;
        self.current_distance = distance;
    }

    fn into_total(self) -> usize {
        if self.current_distance % 2 == self.max_distance % 2 {
            self.total + self.current_count
        } else {
            self.total
        }
    }
}

fn solve<T: Field>(grid: &T, max_d: usize) -> usize {
    let mut counter = Counter::new(max_d);
    let mut min_distances = BTreeMap::new();

    let mut queue = VecDeque::new();
    queue.push_back((grid.get_start(), 0));

    while let Some((coord, dist)) = queue.pop_front() {
        if min_distances.contains_key(&coord) {
            continue;
        }
        if dist > max_d {
            break;
        }
        counter.push(dist);
        min_distances.insert(coord, dist);
        let (r, c) = coord;
        if grid.is_empty((r - 1, c)) && !min_distances.contains_key(&(r - 1, c)) {
            queue.push_back(((r - 1, c), dist + 1));
        }
        if grid.is_empty((r + 1, c)) && !min_distances.contains_key(&(r + 1, c)) {
            queue.push_back(((r + 1, c), dist + 1));
        }
        if grid.is_empty((r, c - 1)) && !min_distances.contains_key(&(r, c - 1)) {
            queue.push_back(((r, c - 1), dist + 1));
        }
        if grid.is_empty((r, c + 1)) && !min_distances.contains_key(&(r, c + 1)) {
            queue.push_back(((r, c + 1), dist + 1));
        }
    }

    counter.into_total()
}

fn print_distances(ds: &[Vec<u16>]) {
    for row in ds {
        for d in row {
            if *d == u16::MAX {
                print!("  .");
            } else {
                print!("{:3}", d);
            }
        }
        println!();
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    solve(&grid, 64)
}
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let grid = Infinite(grid);
    solve(&grid, 26501365)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::{parse_grid, Infinite};

    use super::{solve, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(format!("{}", solve(&grid, 6)), "16");
    }

    #[test]
    fn test_part1_actual() {
        let grid = parse_grid(ACTUAL);
        assert_eq!(format!("{}", solve(&grid, 64)), "3740");
    }

    #[rstest]
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
    #[case(5000, 16733044)]
    fn test_part2(#[case] steps: usize, #[case] expected: usize) {
        let grid = parse_grid(EXAMPLE);
        let grid = Infinite(grid);
        assert_eq!(solve(&grid, steps), expected);
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
