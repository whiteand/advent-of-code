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

impl MinDistances for () {
    fn get_min_distance_to(&self, _coord: &(isize, isize)) -> Option<usize> {
        None
    }
}

impl MinDistances for BTreeMap<(isize, isize), usize> {
    fn get_min_distance_to(&self, coord: &(isize, isize)) -> Option<usize> {
        self.get(coord).copied()
    }
}

trait AggregatedMinDistances {
    fn get_min_distances_for_row(&self, row: isize) -> Option<usize>;
    fn get_min_distances_for_col(&self, col: isize) -> Option<usize>;
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
impl AggregatedMinDistances for Vec<Vec<usize>> {
    fn get_min_distances_for_row(&self, row: isize) -> Option<usize> {
        let r = usize::try_from(row).ok()?;
        self.get(r)
            .and_then(|r| r.iter().copied().min())
            .and_then(|d| (d != usize::MAX).then_some(d))
    }
    fn get_min_distances_for_col(&self, col: isize) -> Option<usize> {
        let c = usize::try_from(col).ok()?;
        self.iter()
            .flat_map(|r| r.get(c))
            .copied()
            .min()
            .and_then(|d| (d != usize::MAX).then_some(d))
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

#[allow(dead_code)]
fn print_distances<T: Field, D: MinDistances>(
    rows: Range<isize>,
    cols: Range<isize>,
    grid: &T,
    distances: &D,
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
            if let Some(d) = distances.get_min_distance_to(&(r, c)) {
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
#[allow(dead_code)]
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
            && min_distances[r - 1][c] > dist + 1
        {
            queue.push_back(((r - 1, c), dist + 1));
        }
        if r < rows - 1
            && grid.is_empty(((r + 1) as isize, c as isize))
            && min_distances[r + 1][c] > dist + 1
        {
            queue.push_back(((r + 1, c), dist + 1));
        }
        if c > 0
            && grid.is_empty((r as isize, (c - 1) as isize))
            && min_distances[r][c - 1] > dist + 1
        {
            queue.push_back(((r, c - 1), dist + 1));
        }
        if c < cols - 1
            && grid.is_empty((r as isize, (c + 1) as isize))
            && min_distances[r][c + 1] > dist + 1
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
    tops: Vec<DistanceMap>,
    bottoms: Vec<DistanceMap>,
    lefts: Vec<DistanceMap>,
    rights: Vec<DistanceMap>,
    center: Vec<Vec<usize>>,
}

impl InfiniteMinDistances {
    fn new<F: Field>(grid: &F) -> Self {
        let (rows, cols) = grid.get_original_size();
        let center =
            get_minimal_distances(grid, rows, cols, std::iter::once((grid.get_start(), 0)));
        let left_top =
            get_minimal_distances(grid, rows, cols, std::iter::once(((rows - 1, cols - 1), 2)));

        let tops = {
            let mut res = Vec::new();
            let mut current = get_minimal_distances(
                grid,
                rows,
                cols,
                (0..cols).map(|c| {
                    (
                        (rows - 1, c),
                        center
                            .get_min_distance_to(&(0, c as isize))
                            .expect("to be present")
                            + 1,
                    )
                }),
            );
            loop {
                let current_distance = DistanceMap::from(current.as_slice());

                let next = get_minimal_distances(
                    grid,
                    rows,
                    cols,
                    (0..cols).map(|c| {
                        (
                            (rows - 1, c),
                            current
                                .get_min_distance_to(&(0, c as isize))
                                .expect("to be present")
                                + 1,
                        )
                    }),
                );

                let next_distance = DistanceMap::from(next.as_slice());
                if current_distance.relatively_same(&next_distance) {
                    res.push(current_distance);
                    break;
                } else {
                    res.push(current_distance);
                    current = next;
                }
            }
            res
        };
        let right_top =
            get_minimal_distances(grid, rows, cols, std::iter::once(((rows - 1, 0), 2)));
        let lefts = {
            let mut res = Vec::new();
            let mut current = get_minimal_distances(
                grid,
                rows,
                cols,
                (0..rows).map(|r| {
                    (
                        (r, cols - 1),
                        center
                            .get_min_distance_to(&(r as isize, 0))
                            .expect("to be present")
                            + 1,
                    )
                }),
            );
            loop {
                let current_distance = DistanceMap::from(current.as_slice());

                let next = get_minimal_distances(
                    grid,
                    rows,
                    cols,
                    (0..rows).map(|r| {
                        (
                            (r, cols - 1),
                            current
                                .get_min_distance_to(&(r as isize, 0))
                                .expect("to be present")
                                + 1,
                        )
                    }),
                );

                let next_distance = DistanceMap::from(next.as_slice());
                if current_distance.relatively_same(&next_distance) {
                    res.push(current_distance);
                    break;
                } else {
                    res.push(current_distance);
                    current = next;
                }
            }
            res
        };
        let rights = {
            let mut res = Vec::new();
            let mut current = get_minimal_distances(
                grid,
                rows,
                cols,
                (0..rows).map(|r| {
                    (
                        (r, 0),
                        center
                            .get_min_distance_to(&(r as isize, (cols as isize) - 1))
                            .expect("to be present")
                            + 1,
                    )
                }),
            );
            loop {
                let current_distance = DistanceMap::from(current.as_slice());

                let next = get_minimal_distances(
                    grid,
                    rows,
                    cols,
                    (0..rows).map(|r| {
                        (
                            (r, 0),
                            current
                                .get_min_distance_to(&(r as isize, (cols as isize) - 1))
                                .expect("to be present")
                                + 1,
                        )
                    }),
                );

                let next_distance = DistanceMap::from(next.as_slice());
                if current_distance.relatively_same(&next_distance) {
                    res.push(current_distance);
                    break;
                } else {
                    res.push(current_distance);
                    current = next;
                }
            }
            res
        };
        let left_bottom =
            get_minimal_distances(grid, rows, cols, std::iter::once(((0, cols - 1), 2)));
        let bottoms = {
            let mut res = Vec::new();
            let mut current = get_minimal_distances(
                grid,
                rows,
                cols,
                (0..cols).map(|c| {
                    (
                        (0, c),
                        center
                            .get_min_distance_to(&((rows as isize) - 1, c as isize))
                            .expect("to be present")
                            + 1,
                    )
                }),
            );
            loop {
                let current_distance = DistanceMap::from(current.as_slice());

                let next = get_minimal_distances(
                    grid,
                    rows,
                    cols,
                    (0..cols).map(|c| {
                        (
                            (0, c),
                            current
                                .get_min_distance_to(&((rows as isize) - 1, c as isize))
                                .expect("to be present")
                                + 1,
                        )
                    }),
                );

                let next_distance = DistanceMap::from(next.as_slice());
                if current_distance.relatively_same(&next_distance) {
                    res.push(current_distance);
                    break;
                } else {
                    res.push(current_distance);
                    current = next;
                }
            }
            res
        };
        let right_bottom = get_minimal_distances(grid, rows, cols, std::iter::once(((0, 0), 2)));

        Self {
            size: (rows as usize, cols as usize),
            left_top,
            tops,
            right_top,
            left_bottom,
            right_bottom,
            bottoms,
            lefts,
            rights,
            center,
        }
    }
}

struct DistanceMap {
    min_distance: usize,
    difference: Vec<Vec<usize>>,
    input: (usize, usize),
}

impl DistanceMap {
    fn relatively_same(&self, other: &Self) -> bool {
        let it_first = self.difference.iter().flatten();
        let it_second = other.difference.iter().flatten();
        it_first.eq(it_second)
    }
}

impl MinDistances for DistanceMap {
    fn get_min_distance_to(&self, (r, c): &(isize, isize)) -> Option<usize> {
        let r = usize::try_from(*r).ok()?;
        let c = usize::try_from(*c).ok()?;
        let row = self.difference.get(r)?;
        let cell = row.get(c)?;
        if *cell == usize::MAX {
            return None;
        }
        Some(*cell + self.min_distance)
    }
}

impl From<&[Vec<usize>]> for DistanceMap {
    fn from(map: &[Vec<usize>]) -> Self {
        let (row, col, min_distance) = map
            .iter()
            .enumerate()
            .flat_map(|(row, r)| r.iter().enumerate().map(move |(col, it)| (row, col, *it)))
            .min_by_key(|(_, _, it)| *it)
            .expect("empty map");

        let difference = map
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|d| {
                        if *d == usize::MAX {
                            *d
                        } else {
                            (*d) - min_distance
                        }
                    })
                    .collect_vec()
            })
            .collect_vec();

        Self {
            min_distance,
            difference,
            input: (row, col),
        }
    }
}

impl MinDistances for InfiniteMinDistances {
    fn get_min_distance_to(&self, coord: &(isize, isize)) -> Option<usize> {
        let (rows, cols) = self.size;
        let (mut row, mut col) = coord.clone();
        let rows_i = rows as isize;
        let cols_i = cols as isize;
        let mut total = 0usize;

        loop {
            if row >= 0 && row < rows_i && col >= 0 && col < cols_i {
                total += self.center.get_min_distance_to(&(row, col))?;
                return Some(total);
            }
            let c_rem = col.rem_euclid(cols_i);
            let r_rem = row.rem_euclid(rows_i);

            if row < 0 && col < 0 {
                if r_rem == 0 && c_rem == 0 {
                    let row_grid_index = row.abs() as usize / rows;
                    let col_grid_index = col.abs() as usize / cols;
                    let steps_to_vertical_or_horizontal = row_grid_index.min(col_grid_index);
                    let next_row_grid_index = row_grid_index - steps_to_vertical_or_horizontal;
                    let next_col_grid_index = col_grid_index - steps_to_vertical_or_horizontal;
                    total += self.left_top.get_min_distance_to(&(r_rem, c_rem))?
                        * steps_to_vertical_or_horizontal;
                    row = -(next_row_grid_index as isize * rows_i);
                    col = -(next_col_grid_index as isize * cols_i);
                    continue;
                }
                row += rows_i - r_rem;
                col += cols_i - c_rem;
                total += self.left_top.get_min_distance_to(&(r_rem, c_rem))?;
                continue;
            }

            if row < 0 && col >= 0 && col < cols_i {
                let top_index = (row + 1).abs() as usize / rows;
                if top_index < self.tops.len() {
                    total += self.tops[top_index].get_min_distance_to(&(r_rem, c_rem))?;
                    return Some(total);
                }
                let distance_to_known = top_index - self.tops.len();
                let last_top = self.tops.last().expect("to be present");
                let difference_per_grid = last_top.difference[0][last_top.input.1] + 1;
                total += last_top.difference.get_min_distance_to(&(r_rem, c_rem))?
                    + difference_per_grid * distance_to_known
                    + last_top.difference[0][last_top.input.1]
                    + 1
                    + last_top.min_distance;
                return Some(total);
            }

            if row < 0 && col >= cols_i {
                if r_rem == 0 && c_rem == cols_i - 1 {
                    let row_grid_index = row.abs() as usize / rows;
                    let col_grid_index = (col.abs() as usize + 1 - cols) / cols;
                    let steps_to_horizontal_or_vertical = row_grid_index.min(col_grid_index);
                    let next_row_grid_index = row_grid_index - steps_to_horizontal_or_vertical;
                    let next_col_grid_index = col_grid_index - steps_to_horizontal_or_vertical;
                    total += self.right_top.get_min_distance_to(&(r_rem, c_rem))?
                        * steps_to_horizontal_or_vertical;
                    row = -(next_row_grid_index as isize * rows_i);
                    col = (next_col_grid_index as isize * cols_i) + cols_i - 1;
                    continue;
                }
                row += rows_i - r_rem;
                col -= c_rem + 1;
                total += self.right_top.get_min_distance_to(&(r_rem, c_rem))?;
                continue;
            }

            if row >= 0 && row < rows_i && col < 0 {
                let left_index = (col + 1).abs() as usize / cols;
                if left_index < self.lefts.len() {
                    total += self.lefts[left_index].get_min_distance_to(&(r_rem, c_rem))?;
                    return Some(total);
                }
                let distance_to_known = left_index - self.lefts.len();
                let last_left = self.lefts.last().expect("to be present");
                let difference_per_grid = last_left.difference[last_left.input.0][0] + 1;
                total += last_left.difference.get_min_distance_to(&(r_rem, c_rem))?
                    + difference_per_grid * distance_to_known
                    + last_left.difference[last_left.input.0][0]
                    + 1
                    + last_left.min_distance;
                return Some(total);
            }

            if row >= 0 && row < rows_i && col >= cols_i {
                let right_ind = (col as usize - cols) / cols;
                if right_ind < self.rights.len() {
                    total += self.rights[right_ind].get_min_distance_to(&(r_rem, c_rem))?;
                    return Some(total);
                }

                let distance_to_known = right_ind - self.rights.len();
                let last_right = self.rights.last().expect("to be present");
                let difference_per_grid = last_right.difference[last_right.input.0][cols - 1] + 1;
                total += last_right.difference.get_min_distance_to(&(r_rem, c_rem))?
                    + difference_per_grid * distance_to_known
                    + last_right.difference[last_right.input.0][cols - 1]
                    + 1
                    + last_right.min_distance;
                return Some(total);
            }

            if row >= rows_i && col < 0 {
                row -= r_rem + 1;
                col += cols_i - c_rem;
                total += self.left_bottom.get_min_distance_to(&(r_rem, c_rem))?;
                continue;
            }

            if row >= rows_i && col >= cols_i {
                row -= r_rem + 1;
                col -= c_rem + 1;
                total += self.right_bottom.get_min_distance_to(&(r_rem, c_rem))?;
                continue;
            }

            if row >= rows_i && col >= 0 && col < cols_i {
                let bottom_ind = (row as usize - rows) / rows;
                if bottom_ind < self.bottoms.len() {
                    total += self.bottoms[bottom_ind].get_min_distance_to(&(r_rem, c_rem))?;
                    return Some(total);
                }

                let distance_to_known = bottom_ind - self.bottoms.len();
                let last_bottom = self.bottoms.last().expect("to be present");
                let difference_per_grid = last_bottom.difference[rows - 1][last_bottom.input.1] + 1;
                total += last_bottom
                    .difference
                    .get_min_distance_to(&(r_rem, c_rem))?
                    + difference_per_grid * distance_to_known
                    + last_bottom.difference[rows - 1][last_bottom.input.1]
                    + 1
                    + last_bottom.min_distance;
                return Some(total);
            }

            unreachable!();
        }
    }
}
impl AggregatedMinDistances for InfiniteMinDistances {
    fn get_min_distances_for_row(&self, row: isize) -> Option<usize> {
        let (_, cols) = self.size;
        (0..cols)
            .flat_map(|c| self.get_min_distance_to(&(row, c as isize)))
            .min()
    }

    fn get_min_distances_for_col(&self, col: isize) -> Option<usize> {
        let (rows, _) = self.size;
        (0..rows)
            .flat_map(|r| self.get_min_distance_to(&(r as isize, col)))
            .min()
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
    let start = grid.get_start();
    let size = grid.get_original_size();

    // print_distances(-44..22, -11..22, &grid, &distances);

    get_odd_count_less_then(start, size, &distances, steps)
}

fn iter_spiral() -> impl Iterator<Item = (isize, isize)> {
    let mut dir = (0, 1);
    let mut pos = (0, 0);
    let mut steps = 2;
    let mut remaining_steps = steps / 2;
    std::iter::from_fn(move || {
        if remaining_steps == 0 {
            dir = match dir {
                (0, 1) => (1, 0),
                (1, 0) => (0, -1),
                (0, -1) => (-1, 0),
                (-1, 0) => (0, 1),
                _ => unreachable!(),
            };
            steps += 1;
            remaining_steps = steps / 2;
        }
        let prev_pos = pos;
        remaining_steps -= 1;
        pos = (pos.0 + dir.0, pos.1 + dir.1);
        Some(prev_pos)
    })
}

fn get_odd_count_less_then<T: MinDistances + AggregatedMinDistances>(
    (start_row, start_col): (usize, usize),
    _: (usize, usize),
    distances: &T,
    steps: usize,
) -> usize {
    let remainder = steps % 2;

    let mut total = 0;
    for (dr, dc) in iter_spiral() {
        let r = (start_row as isize) + dr;
        let c = (start_col as isize) + dc;
        if dc == 0 && dr.abs() > steps as isize {
            break;
        }
        if dr == 0 && dc.abs() > steps as isize {
            break;
        }
        if dr.abs() + dc.abs() > steps as isize {
            continue;
        }

        match distances.get_min_distance_to(&(r, c)) {
            Some(d) => {
                if d <= steps && d % 2 == remainder {
                    total += 1;
                }
            }
            _ => {}
        }
    }

    total
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
    #[case(6, 16)]
    #[case(10, 50)]
    #[case(50, 1594)]
    #[case(100, 6536)]
    #[case(500, 167004)]
    #[case(1000, 668697)]
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
