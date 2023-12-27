use std::{
    collections::{BTreeMap, VecDeque},
    ops::Range,
};

use itertools::Itertools;

trait Field {
    fn is_empty(&self, coord: (isize, isize)) -> bool;
    fn get_start(&self) -> (usize, usize);
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
impl MinDistances for Vec<Vec<usize>> {
    fn get_min_distance_to(&self, coord: &(isize, isize)) -> Option<usize> {
        let (r, c) = coord;

        let r = usize::try_from(*r).ok()?;
        let c = usize::try_from(*c).ok()?;

        self.get(r).and_then(|r| {
            r.get(c)
                .copied()
                .and_then(|d| (d != usize::MAX).then_some(d))
        })
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

    fn get_start(&self) -> (usize, usize) {
        (self.start.0, self.start.1)
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
    fn get_start(&self) -> (usize, usize) {
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
    let start = grid.get_start();
    let start = (start.0 as isize, start.1 as isize);
    queue.push_back((start, 0));

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
    rows: usize,
    cols: usize,
    init: impl Iterator<Item = ((usize, usize), usize)>,
) -> Vec<Vec<usize>> {
    let mut min_distances = (0..rows)
        .map(|_| (0..cols).map(|_| usize::MAX).collect_vec())
        .collect_vec();
    let mut queue = VecDeque::new();
    queue.extend(init);

    while let Some((coord, dist)) = queue.pop_front() {
        let prev_distance = min_distances[coord.0][coord.1];
        if dist >= prev_distance {
            continue;
        }
        min_distances[coord.0][coord.1] = dist;
        let (r, c) = coord;
        if r > 0
            && grid.is_empty(((r - 1) as isize, c as isize))
            && min_distances[r - 1][c] == usize::MAX
        {
            queue.push_back(((r - 1, c), dist + 1));
        }
        if r < rows - 1
            && grid.is_empty(((r + 1) as isize, c as isize))
            && min_distances[r + 1][c] == usize::MAX
        {
            queue.push_back(((r + 1, c), dist + 1));
        }
        if c > 0
            && grid.is_empty((r as isize, (c - 1) as isize))
            && min_distances[r][c - 1] == usize::MAX
        {
            queue.push_back(((r, c - 1), dist + 1));
        }
        if c < cols - 1
            && grid.is_empty((r as isize, (c + 1) as isize))
            && min_distances[r][c + 1] == usize::MAX
        {
            queue.push_back(((r, c + 1), dist + 1));
        }
    }

    min_distances
}

struct InfiniteMinDistances {
    size: (usize, usize),
    left_top: Vec<Vec<usize>>,
    right_top: Vec<Vec<usize>>,
    left_bottom: Vec<Vec<usize>>,
    right_bottom: Vec<Vec<usize>>,
    tops: Vec<Vec<Vec<usize>>>,
    bottom: Vec<Vec<usize>>,
    lefts: Vec<Vec<Vec<usize>>>,
    rights: Vec<Vec<Vec<usize>>>,
    center: Vec<Vec<usize>>,
}

impl InfiniteMinDistances {
    fn new<F: Field>(grid: &F) -> Self {
        let (rows, cols) = grid.get_original_size();
        let center =
            get_minimal_distances(grid, rows, cols, std::iter::once((grid.get_start(), 0)));
        let left_top =
            get_minimal_distances(grid, rows, cols, std::iter::once(((rows - 1, cols - 1), 0)));
        let tops = (0..cols)
            .map(|c| get_minimal_distances(grid, rows, cols, std::iter::once(((rows - 1, c), 1))))
            .collect_vec();
        let right_top =
            get_minimal_distances(grid, rows, cols, std::iter::once(((rows - 1, 0), 2)));
        let lefts = (0..rows)
            .map(|r| get_minimal_distances(grid, rows, cols, std::iter::once(((r, cols - 1), 1))))
            .collect_vec();
        let rights = (0..rows)
            .map(|r| get_minimal_distances(grid, rows, cols, std::iter::once(((r, 0), 1))))
            .collect_vec();
        Self {
            size: (rows as usize, cols as usize),
            left_top,
            tops,
            right_top,
            left_bottom: Vec::new(),
            right_bottom: Vec::new(),
            bottom: Vec::new(),
            lefts,
            rights,
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
        if coord.0 < 0 && coord.1 >= 0 && coord.1 < cols as isize {
            let next_row = coord.0 + (rows as isize) - coord.0.rem_euclid(rows as isize);
            let r_rem = coord.0.rem_euclid(rows as isize);
            return (0..cols)
                .flat_map(|c| {
                    self.get_min_distance_to(&(next_row, c as isize))
                        .and_then(|d| {
                            let additional = self.tops[c].get_min_distance_to(&(r_rem, coord.1))?;
                            Some(d + additional)
                        })
                })
                .min();
        }
        if coord.0 < 0 && coord.1 >= cols as isize {
            let next_row = coord.0 + (rows as isize - coord.0.rem_euclid(rows as isize));
            let next_col = coord.1 - coord.1.rem_euclid(cols as isize) - 1;
            let r_rem = coord.0.rem_euclid(rows as isize);
            let c_rem = coord.1.rem_euclid(cols as isize);
            let d = self.get_min_distance_to(&(next_row, next_col))?;
            let additional = self.right_top.get_min_distance_to(&(r_rem, c_rem))?;
            return Some(d + additional);
        }
        if coord.0 >= 0 && coord.0 < rows as isize && coord.1 < 0 {
            let r_rem = coord.0.rem_euclid(rows as isize);
            let c_rem = coord.1.rem_euclid(cols as isize);
            let next_col = coord.1 + (cols as isize - coord.1.rem_euclid(cols as isize));
            return (0..rows)
                .flat_map(|r| {
                    let d = self.get_min_distance_to(&(r as isize, next_col))?;
                    let additional = self.lefts[r].get_min_distance_to(&(r_rem, c_rem))?;
                    Some(d + additional)
                })
                .min();
        }
        if coord.0 >= 0 && coord.0 < rows as isize && coord.1 >= cols as isize {
            let r_rem = coord.0.rem_euclid(rows as isize);
            let c_rem = coord.1.rem_euclid(cols as isize);
            let next_col = coord.1 - coord.1.rem_euclid(cols as isize) - 1;
            return (0..rows)
                .flat_map(|r| {
                    let d = self.get_min_distance_to(&(r as isize, next_col))?;
                    let additional = self.rights[r].get_min_distance_to(&(r_rem, c_rem))?;
                    Some(d + additional)
                })
                .min();
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
