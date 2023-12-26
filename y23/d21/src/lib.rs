use std::{
    collections::{BTreeMap, VecDeque},
    ops::Range,
};

use itertools::Itertools;

trait Field {
    fn is_empty(&self, coord: (isize, isize)) -> bool;
    fn get_start(&self) -> (isize, isize);
    fn get_original_size(&self) -> (usize, usize);
}

trait MinDistances {
    fn get_min_distance_to(&self, coord: &(isize, isize)) -> Option<usize>;
}

impl MinDistances for BTreeMap<(isize, isize), usize> {
    fn get_min_distance_to(&self, coord: &(isize, isize)) -> Option<usize> {
        self.get(coord).copied()
    }
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

    fn get_original_size(&self) -> (usize, usize) {
        (self.cells.len(), self.cells[0].len())
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

    fn get_original_size(&self) -> (usize, usize) {
        self.0.get_original_size()
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
            println!("{} {}", self.current_distance, self.current_count);
            self.current_count = 1;
            self.current_distance = distance;
            return;
        }
        self.current_count = 1;
        self.current_distance = distance;
    }

    fn into_total(self) -> usize {
        if self.current_distance % 2 == self.max_distance % 2 {
            println!("{} {}", self.current_distance, self.current_count);
            self.total + self.current_count
        } else {
            self.total
        }
    }
}

fn solve<T: Field>(grid: &T, max_d: usize) -> (usize, BTreeMap<(isize, isize), usize>) {
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

    (counter.into_total(), min_distances)
}

fn print_distances<T: Field, D: MinDistances>(
    rows: Range<isize>,
    cols: Range<isize>,
    grid: &T,
    ds: &D,
) {
    let original_size = grid.get_original_size();
    for r in rows {
        let rem = r.rem_euclid(original_size.0 as isize);
        if rem == 0 {
            println!();
        }
        for c in cols.clone() {
            let rem_c = c.rem_euclid(original_size.1 as isize);
            if rem_c == 0 {
                print!("|");
            }
            if let Some(d) = ds.get_min_distance_to(&(r, c)) {
                print!("{:3}", d);
            } else if grid.is_empty((r, c)) {
                print!("  .");
            } else {
                print!("  #");
            }
        }
        println!();
    }
}
fn print_distances_oddity(
    rows: Range<isize>,
    cols: Range<isize>,
    grid: &impl Field,
    remainder: usize,
    ds: &impl MinDistances,
) {
    let original_size = grid.get_original_size();
    for r in rows {
        let rem = r.rem_euclid(original_size.0 as isize);
        if rem == 0 {
            println!();
        }
        for c in cols.clone() {
            let rem_c = c.rem_euclid(original_size.1 as isize);
            if rem_c == 0 {
                print!(" ");
            }
            if let Some(d) = ds.get_min_distance_to(&(r, c)) {
                if d % 2 == remainder {
                    print!("◼︎");
                } else {
                    print!("◻︎");
                }
            } else if grid.is_empty((r, c)) {
                print!(".");
            } else {
                print!("#");
            }
        }
        println!();
    }
}

fn get_minimal_distances(
    grid: &impl Field,
    rows: Range<isize>,
    cols: Range<isize>,
    init: impl Iterator<Item = ((isize, isize), usize)>,
) -> BTreeMap<(isize, isize), usize> {
    let mut min_distances = BTreeMap::new();
    let mut queue = VecDeque::new();
    queue.extend(init);

    while let Some((coord, dist)) = queue.pop_front() {
        if min_distances.contains_key(&coord) {
            continue;
        }
        min_distances.insert(coord, dist);
        let (r, c) = coord;
        if rows.contains(&(r - 1))
            && grid.is_empty((r - 1, c))
            && !min_distances.contains_key(&(r - 1, c))
        {
            queue.push_back(((r - 1, c), dist + 1));
        }
        if rows.contains(&(r + 1))
            && grid.is_empty((r + 1, c))
            && !min_distances.contains_key(&(r + 1, c))
        {
            queue.push_back(((r + 1, c), dist + 1));
        }
        if cols.contains(&(c - 1))
            && grid.is_empty((r, c - 1))
            && !min_distances.contains_key(&(r, c - 1))
        {
            queue.push_back(((r, c - 1), dist + 1));
        }
        if cols.contains(&(c + 1))
            && grid.is_empty((r, c + 1))
            && !min_distances.contains_key(&(r, c + 1))
        {
            queue.push_back(((r, c + 1), dist + 1));
        }
    }

    min_distances
}

struct InfiniteMinDistances {
    size: (usize, usize),
    left_top: BTreeMap<(isize, isize), usize>,
    right_top: BTreeMap<(isize, isize), usize>,
    left_bottom: BTreeMap<(isize, isize), usize>,
    right_bottom: BTreeMap<(isize, isize), usize>,
    top: BTreeMap<(isize, isize), usize>,
    bottom: BTreeMap<(isize, isize), usize>,
    left: BTreeMap<(isize, isize), usize>,
    right: BTreeMap<(isize, isize), usize>,
    center: BTreeMap<(isize, isize), usize>,
}

impl InfiniteMinDistances {
    fn new<F: Field>(grid: &F) -> Self {
        let (r, c) = grid.get_original_size();
        let ri = r as isize;
        let ci = c as isize;
        let center =
            get_minimal_distances(grid, 0..ri, 0..ci, std::iter::once((grid.get_start(), 0)));
        let left_top =
            get_minimal_distances(grid, 0..ri, 0..ci, std::iter::once(((ri - 1, ci - 1), 0)));
        Self {
            size: (r as usize, c as usize),
            left_top,
            right_top: BTreeMap::new(),
            left_bottom: BTreeMap::new(),
            right_bottom: BTreeMap::new(),
            top: BTreeMap::new(),
            bottom: BTreeMap::new(),
            left: BTreeMap::new(),
            right: BTreeMap::new(),
            center,
        }
    }
}

impl MinDistances for InfiniteMinDistances {
    fn get_min_distance_to(&self, coord: &(isize, isize)) -> Option<usize> {
        let (rows, cols) = self.size;
        if coord.0 >= 0 && coord.0 < rows as isize && coord.1 >= 0 && coord.1 < cols as isize {
            return self.center.get_min_distance_to(coord);
        }
        if coord.0 < 0 && coord.1 < 0 {
            let r_rem = coord.0.rem_euclid(rows as isize);
            let c_rem = coord.1.rem_euclid(cols as isize);
            let bottom_right_prev_row =
                coord.0 + (rows as isize - coord.0.rem_euclid(rows as isize));
            let bottom_right_prev_col =
                coord.1 + (cols as isize - coord.1.rem_euclid(cols as isize));
            let bottom_right = (bottom_right_prev_row, bottom_right_prev_col);
            let d = self.get_min_distance_to(&bottom_right)?;
            let additional = self.left_top.get_min_distance_to(&(r_rem, c_rem))?;
            return Some(d + additional + 2);
        }
        None
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let (total, _) = solve(&grid, 64);
    total
}
pub fn solve_part_2(file_content: &str, steps: usize) -> usize {
    let grid = parse_grid(file_content);
    let distances = InfiniteMinDistances::new(&grid);

    print_distances(-40..51, -40..51, &grid, &distances);
    print_distances_oddity(-40..51, -40..51, &grid, steps % 2, &distances);
    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::parse_grid;

    use super::{solve, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let grid = parse_grid(EXAMPLE);
        assert_eq!(format!("{}", solve(&grid, 6).0), "16");
    }

    #[test]
    fn test_part1_actual() {
        let grid = parse_grid(ACTUAL);
        assert_eq!(format!("{}", solve(&grid, 64).0), "3740");
    }

    #[rstest]
    // #[case(6, 16)]
    // #[case(10, 50)]
    #[case(50, 1594)]
    // #[case(100, 6536)]
    // #[case(500, 167004)]
    // #[case(1000, 668697)]
    // #[case(5000, 16733044)]
    fn test_part2(#[case] steps: usize, #[case] expected: usize) {
        assert_eq!(solve_part_2(EXAMPLE, steps), expected);
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL, 26501365)), "0");
    }
}
