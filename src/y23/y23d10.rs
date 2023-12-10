use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Horizontal,
    Vertical,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None,
    Start,
}

impl Cell {
    fn is_pipe(&self) -> bool {
        !self.is_none()
    }
    fn is_none(&self) -> bool {
        matches!(self, Cell::None)
    }
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Horizontal => write!(f, "_"),
            Vertical => write!(f, "|"),
            TopRight => write!(f, "L"),
            TopLeft => write!(f, "J"),
            BottomRight => write!(f, "F"),
            BottomLeft => write!(f, "7"),
            None => write!(f, "."),
            Start => write!(f, "S"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Cell::*;
        match value {
            '-' => Ok(Horizontal),
            '|' => Ok(Vertical),
            'L' => Ok(TopRight),
            'J' => Ok(TopLeft),
            'F' => Ok(BottomRight),
            '7' => Ok(BottomLeft),
            '.' => Ok(None),
            'S' => Ok(Start),
            _ => Err(()),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }
    fn rows(&self) -> usize {
        self.cells.len()
    }
    fn cols(&self) -> usize {
        self.cells[0].len()
    }
    fn is_connected(&self, row1: usize, col1: usize, row2: usize, col2: usize) -> bool {
        // checking that we are requesting only vertical and horizontal neighbours
        debug_assert!(
            (row1 == row2 && (col1 == col2 + 1) || (col1 + 1 == col2))
                || (col1 == col2 && (row1 + 1 == row2 || row1 == row2 + 1))
        );
        if row1 == row2 {
            if col1 == col2 + 1 {
                let right = self.get(row1, col1);
                let left = self.get(row2, col2);
                matches!(
                    left,
                    Cell::Horizontal | Cell::TopRight | Cell::BottomRight | Cell::Start
                ) && matches!(
                    right,
                    Cell::Horizontal | Cell::TopLeft | Cell::BottomLeft | Cell::Start
                )
            } else {
                self.is_connected(row2, col2, row1, col1)
            }
        } else if row1 == row2 + 1 {
            let top = self.get(row2, col2);
            let bottom = self.get(row1, col1);
            matches!(
                top,
                Cell::Vertical | Cell::BottomRight | Cell::BottomLeft | Cell::Start
            ) && matches!(
                bottom,
                Cell::Vertical | Cell::TopRight | Cell::TopLeft | Cell::Start
            )
        } else {
            self.is_connected(row2, col2, row1, col1)
        }
    }
    fn start(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .find_map(|(col, cell)| matches!(cell, Cell::Start).then_some((row, col)))
            })
            .unwrap()
    }
    fn iter(&self, from_row: usize, from_col: usize, row: usize, col: usize) -> GridIter<'_> {
        GridIter {
            grid: self,
            start_row: from_row,
            start_col: from_col,
            row,
            col,
            previous_row: from_row,
            previous_col: from_col,
        }
    }
}
impl std::ops::Deref for Grid {
    type Target = Vec<Vec<Cell>>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}

impl FromIterator<Vec<Cell>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<Cell>>>(iter: T) -> Self {
        let mut cells = iter.into_iter().collect::<Vec<_>>();
        for line in &mut cells {
            line.insert(0, Cell::None);
            line.push(Cell::None);
        }
        cells.insert(0, vec![Cell::None; cells[0].len()]);
        cells.push(vec![Cell::None; cells[0].len()]);
        Grid { cells }
    }
}

struct GridIter<'a> {
    grid: &'a Grid,
    start_row: usize,
    start_col: usize,
    row: usize,
    col: usize,
    previous_row: usize,
    previous_col: usize,
}

impl<'t> Iterator for GridIter<'t> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        // When we are at the start of the loop
        // and we already returned it as an item
        // we set previous row and col to the current row and col
        // without updating current row and col
        // So we can return None. Because loop was already fully iterated.
        if self.previous_col == self.col && self.previous_row == self.row {
            return None;
        }

        // If we at the start position and start position
        // We are finishing loop
        // The last value is the start position value is the start is a pipe
        // or None if the start is not a pipe
        if self.row == self.start_row && self.col == self.start_col {
            self.previous_col = self.col;
            self.previous_row = self.row;
            return self.grid[self.row][self.col]
                .is_pipe()
                .then_some((self.row, self.col));
        }

        let previous_row = self.previous_row;
        let previous_col = self.previous_col;
        self.previous_row = self.row;
        self.previous_col = self.col;

        // Searching for next position
        for (r, c) in [
            (self.row - 1, self.col),
            (self.row, self.col + 1),
            (self.row + 1, self.col),
            (self.row, self.col - 1),
        ] {
            if r == previous_row && c == previous_col {
                continue;
            }
            if !self.grid.is_connected(self.row, self.col, r, c) {
                continue;
            }
            self.row = r;
            self.col = c;
            break;
        }
        Some((self.previous_row, self.previous_col))
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let (start_row, start_col) = grid.start();
    [
        (start_row - 1, start_col),
        (start_row + 1, start_col),
        (start_row, start_col + 1),
        (start_row, start_col - 1),
    ]
    .into_iter()
    .filter(|(r, c)| grid.is_connected(start_row, start_col, *r, *c))
    .find_map(|(r, c)| {
        grid.iter(start_row, start_col, r, c)
            .enumerate()
            .find(|(_, (r, c))| *r == start_row && *c == start_col)
            .map(|(ind, _)| (ind + 1) / 2)
    })
    .expect("at least one loop should start from start")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Left,
    Right,
    On,
}

struct ColorGrid {
    colors: Vec<Vec<Option<Color>>>,
}
impl ColorGrid {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            colors: vec![vec![None; cols]; rows],
        }
    }
    /// Returns true if the cell is not already colored
    fn set(&mut self, row: usize, col: usize, value: Color) -> bool {
        if value == Color::Left && self.colors[row][col] == Some(Color::Right) {
            unreachable!("Trying to set color both to left and right")
        }
        if value == Color::Right && self.colors[row][col] == Some(Color::Left) {
            unreachable!("Trying to set color both to left and right")
        }
        match self.colors[row][col] {
            Some(_) => false,
            None => {
                self.colors[row][col] = Some(value);
                true
            }
        }
    }
}
#[derive(Debug)]
struct ColorTask {
    row: usize,
    col: usize,
    color: Color,
}
impl ColorTask {
    fn new(row: usize, col: usize, color: Color) -> Self {
        Self { row, col, color }
    }
}

pub fn solve_task2(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);
    let (start_row, start_col) = grid.start();

    let (loop_row, loop_col, loop_end_row, loop_end_col) = [
        (start_row - 1, start_col),
        (start_row + 1, start_col),
        (start_row, start_col + 1),
        (start_row, start_col - 1),
    ]
    .into_iter()
    .filter(|(r, c)| grid.is_connected(start_row, start_col, *r, *c))
    .find_map(|(loop_start_row, loop_start_col)| {
        let mut previous = (loop_start_row, loop_start_col);
        for (r, c) in grid.iter(start_row, start_col, loop_start_row, loop_start_col) {
            if r != start_row || c != start_col {
                previous = (r, c);
                continue;
            }
            return Some((loop_start_row, loop_start_col, previous.0, previous.1));
        }
        None
    })
    .expect("at least one loop should start from the start point");

    // Settings start position to close the loop in appropriate way
    grid.cells[start_row][start_col] = get_connection_pipe(
        loop_end_row,
        loop_end_col,
        start_row,
        start_col,
        loop_row,
        loop_col,
    );

    let mut colors = ColorGrid::new(grid.rows(), grid.cols());
    for (r, c) in grid.iter(start_row, start_col, loop_row, loop_col) {
        colors.set(r, c, Color::On);
    }
    let mut tasks = Vec::new();
    for ((pr, pc), (r, c)) in std::iter::once((loop_end_row, loop_end_col))
        .chain(grid.iter(start_row, start_col, loop_row, loop_col))
        .chain(std::iter::once((loop_row, loop_col)))
        .tuple_windows()
    {
        let value = grid.get(r, c);
        match value {
            Cell::Horizontal if pc < c => {
                if colors.set(r - 1, c, Color::Left) {
                    tasks.push(ColorTask::new(r - 1, c, Color::Left))
                }
                if colors.set(r + 1, c, Color::Right) {
                    tasks.push(ColorTask::new(r + 1, c, Color::Right))
                }
            }
            Cell::Horizontal if pc > c => {
                if colors.set(r - 1, c, Color::Right) {
                    tasks.push(ColorTask::new(r - 1, c, Color::Right))
                }
                if colors.set(r + 1, c, Color::Left) {
                    tasks.push(ColorTask::new(r + 1, c, Color::Left))
                }
            }
            Cell::Vertical if pr < r => {
                if colors.set(r, c + 1, Color::Left) {
                    tasks.push(ColorTask::new(r, c + 1, Color::Left))
                }
                if colors.set(r, c - 1, Color::Right) {
                    tasks.push(ColorTask::new(r, c - 1, Color::Right))
                }
            }
            Cell::Vertical if pr > r => {
                if colors.set(r, c + 1, Color::Right) {
                    tasks.push(ColorTask::new(r, c + 1, Color::Right))
                }
                if colors.set(r, c - 1, Color::Left) {
                    tasks.push(ColorTask::new(r, c - 1, Color::Left))
                }
            }
            Cell::BottomRight if pr > r => {
                if colors.set(r - 1, c, Color::Left) {
                    tasks.push(ColorTask::new(r - 1, c, Color::Left))
                }
                if colors.set(r, c - 1, Color::Left) {
                    tasks.push(ColorTask::new(r, c - 1, Color::Left))
                }
            }
            Cell::BottomRight if pc > c => {
                if colors.set(r - 1, c, Color::Right) {
                    tasks.push(ColorTask::new(r - 1, c, Color::Right))
                }
                if colors.set(r, c - 1, Color::Right) {
                    tasks.push(ColorTask::new(r, c - 1, Color::Right))
                }
            }
            Cell::TopLeft if pr < r => {
                if colors.set(r, c + 1, Color::Left) {
                    tasks.push(ColorTask::new(r, c + 1, Color::Left))
                }
                if colors.set(r + 1, c, Color::Left) {
                    tasks.push(ColorTask::new(r + 1, c, Color::Left))
                }
            }
            Cell::TopLeft if pc < c => {
                if colors.set(r, c + 1, Color::Right) {
                    tasks.push(ColorTask::new(r, c + 1, Color::Right))
                }
                if colors.set(r + 1, c, Color::Right) {
                    tasks.push(ColorTask::new(r + 1, c, Color::Right))
                }
            }
            Cell::TopRight if pr < r => {
                if colors.set(r, c - 1, Color::Right) {
                    tasks.push(ColorTask::new(r, c - 1, Color::Right))
                }
                if colors.set(r + 1, c, Color::Right) {
                    tasks.push(ColorTask::new(r + 1, c, Color::Right))
                }
            }
            Cell::TopRight if pc > c => {
                if colors.set(r, c - 1, Color::Left) {
                    tasks.push(ColorTask::new(r, c - 1, Color::Left))
                }
                if colors.set(r + 1, c, Color::Left) {
                    tasks.push(ColorTask::new(r + 1, c, Color::Left))
                }
            }
            Cell::BottomLeft if pr > r => {
                if colors.set(r, c + 1, Color::Right) {
                    tasks.push(ColorTask::new(r, c + 1, Color::Right))
                }
                if colors.set(r - 1, c, Color::Right) {
                    tasks.push(ColorTask::new(r - 1, c, Color::Right))
                }
            }
            Cell::BottomLeft if pc < c => {
                if colors.set(r, c + 1, Color::Left) {
                    tasks.push(ColorTask::new(r, c + 1, Color::Left))
                }
                if colors.set(r - 1, c, Color::Left) {
                    tasks.push(ColorTask::new(r - 1, c, Color::Left))
                }
            }
            v => todo!(
                "Handle {v:?} coming after {:?}",
                (r as isize - pr as isize, c as isize - pc as isize)
            ),
        }
    }

    while let Some(ColorTask { row, col, color }) = tasks.pop() {
        if row > 0 && colors.set(row - 1, col, color) {
            tasks.push(ColorTask::new(row - 1, col, color));
        }
        if row < grid.rows() - 1 && colors.set(row + 1, col, color) {
            tasks.push(ColorTask::new(row + 1, col, color));
        }
        if col > 0 && colors.set(row, col - 1, color) {
            tasks.push(ColorTask::new(row, col - 1, color));
        }
        if col < grid.cols() - 1 && colors.set(row, col + 1, color) {
            tasks.push(ColorTask::new(row, col + 1, color));
        }
    }
    let internal_color = if colors.colors[0][0].unwrap() == Color::Left {
        Color::Right
    } else {
        Color::Left
    };
    (0..grid.rows())
        .flat_map(|r| (0..grid.cols()).map(move |c| (r, c)))
        .inspect(|(r, c)| {
            debug_assert!(colors.colors[*r][*c].is_some());
        })
        .filter(|(r, c)| colors.colors[*r][*c].unwrap() == internal_color)
        .count()
}

fn get_connection_pipe(
    from_row: usize,
    from_col: usize,
    via_row: usize,
    via_col: usize,
    to_row: usize,
    to_col: usize,
) -> Cell {
    debug_assert!((from_row, from_col) != (via_row, via_col));
    debug_assert!((to_row, to_col) != (via_row, via_col));
    // Horizontal
    use std::cmp::Ordering::*;
    match (
        from_row.cmp(&via_row),
        from_col.cmp(&via_col),
        to_row.cmp(&via_row),
        to_col.cmp(&via_col),
    ) {
        (Equal, Greater, Greater, Equal) | (Greater, Equal, Equal, Greater) => Cell::BottomRight,
        (Equal, Less, Equal, Greater) | (Equal, Greater, Equal, Less) => Cell::Horizontal,
        (Less, Equal, Equal, Greater) | (Equal, Greater, Less, Equal) => Cell::TopRight,
        (Greater, Equal, Equal, Less) | (Equal, Less, Greater, Equal) => Cell::BottomLeft,
        (Less, Equal, Greater, Equal) | (Greater, Equal, Less, Equal) => Cell::Vertical,
        (Less, Equal, Equal, Less) | (Equal, Less, Less, Equal) => Cell::TopLeft,
        _ => unreachable!("These cells are not connected"),
    }
}

fn parse_grid(file_content: &str) -> Grid {
    file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Grid>()
}
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("./y23d10/example.txt");
    const EXAMPLE_2: &str = include_str!("./y23d10/example2.txt");
    const EXAMPLE_3: &str = include_str!("./y23d10/example3.txt");
    const EXAMPLE_4: &str = include_str!("./y23d10/example4.txt");
    const EXAMPLE_5: &str = include_str!("./y23d10/example5.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d10.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(EXAMPLE)), "8");
    }

    #[test]
    fn test_task1_2() {
        assert_eq!(format!("{}", solve_task1(EXAMPLE_2)), "4");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "6820");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(EXAMPLE_3)), "4");
    }
    #[test]
    fn test_task2_2() {
        assert_eq!(format!("{}", solve_task2(EXAMPLE_4)), "8");
    }
    #[test]
    fn test_task2_3() {
        assert_eq!(format!("{}", solve_task2(EXAMPLE_5)), "10");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "337");
    }

    #[test]
    fn test_get_connection_pipe() {
        let grid = parse_grid(ACTUAL);
        for r in 1..(grid.rows() - 1) {
            for c in 1..(grid.cols() - 1) {
                let actual = grid.get(r, c);
                if matches!(actual, Cell::Start) {
                    continue;
                }
                for (a, b) in [(r - 1, c), (r + 1, c), (r, c - 1), (r, c + 1)]
                    .into_iter()
                    .permutations(2)
                    .map(|v| v.into_iter().collect_tuple().unwrap())
                {
                    if !grid.is_connected(a.0, a.1, r, c) {
                        continue;
                    }
                    if !grid.is_connected(b.0, b.1, r, c) {
                        continue;
                    }
                    let pipe = get_connection_pipe(a.0, a.1, r, c, b.0, b.1);

                    assert_eq!(pipe, actual);
                }
            }
        }
    }
}
