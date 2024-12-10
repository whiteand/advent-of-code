pub fn solve_part_1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = Visited::new(rows, cols);
    let mut to_visit = ToVisit::new(rows, cols);

    to_visit.start_with((0, 0, Direction::Right));

    get_energized(&grid, &mut visited, &mut to_visit)
}
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let rows = grid.len();
    let cols = grid[0].len();
    let mut visited = Visited::new(rows, cols);
    let mut to_visit = ToVisit::new(rows, cols);

    [
        (0, 0, Direction::Down),
        (0, cols - 1, Direction::Left),
        (0, cols - 1, Direction::Down),
        (rows - 1, cols - 1, Direction::Up),
        (rows - 1, cols - 1, Direction::Left),
        (rows - 1, 0, Direction::Right),
        (rows - 1, 0, Direction::Up),
        (0, 0, Direction::Right),
    ]
    .into_iter()
    .chain((1..(rows - 1)).map(|r| (r, 0, Direction::Right)))
    .chain((1..(rows - 1)).map(|r| (r, rows - 1, Direction::Left)))
    .chain((1..(cols - 1)).map(|c| (0, c, Direction::Down)))
    .chain((1..(cols - 1)).map(|c| (rows - 1, c, Direction::Up)))
    .map(move |pos| {
        visited.clear();
        to_visit.start_with(pos);
        get_energized(&grid, &mut visited, &mut to_visit)
    })
    .max()
    .unwrap_or_default()
}

fn parse_grid(input: &str) -> Vec<Vec<Option<Object>>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Object::try_from(c).ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Copy, PartialEq, Debug, Eq, Clone)]
enum Object {
    VerticalSplitter,
    HorizontalSplitter,
    RightDownMirror,
    RightUpMirror,
}

impl TryFrom<char> for Object {
    type Error = String;
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '|' => Ok(Object::VerticalSplitter),
            '-' => Ok(Object::HorizontalSplitter),
            '/' => Ok(Object::RightUpMirror),
            '\\' => Ok(Object::RightDownMirror),
            _ => Err(format!("Invalid char: {}", c)),
        }
    }
}

#[derive(Copy, PartialEq, Eq, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for u8 {
    fn from(d: Direction) -> Self {
        match d {
            Direction::Up => 1,
            Direction::Right => 2,
            Direction::Down => 4,
            Direction::Left => 8,
        }
    }
}

struct Visited {
    visited: Vec<Vec<u8>>,
}
impl Visited {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            visited: vec![vec![0; cols]; rows],
        }
    }
    fn clear(&mut self) {
        for r in self.visited.iter_mut() {
            r.fill(0);
        }
    }
    fn mark_as_visited(&mut self, row: usize, col: usize, dir: Direction) {
        self.visited[row][col] |= u8::from(dir);
    }
    fn is_visited(&self, row: usize, col: usize, dir: Direction) -> bool {
        self.visited[row][col] & u8::from(dir) != 0
    }
}

struct ToVisit {
    rows: usize,
    cols: usize,
    positions: Vec<(usize, usize, Direction)>,
}

impl ToVisit {
    fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            positions: Vec::new(),
        }
    }

    fn pop(&mut self) -> Option<(usize, usize, Direction)> {
        self.positions.pop()
    }
    fn start_with(&mut self, pos: (usize, usize, Direction)) {
        debug_assert!(self.positions.is_empty());
        self.positions.push(pos);
    }
    fn up(&mut self, row: usize, col: usize) {
        if row > 0 {
            self.positions.push((row - 1, col, Direction::Up));
        }
    }
    fn right(&mut self, row: usize, col: usize) {
        if col < self.cols - 1 {
            self.positions.push((row, col + 1, Direction::Right));
        }
    }
    fn down(&mut self, row: usize, col: usize) {
        if row < self.rows - 1 {
            self.positions.push((row + 1, col, Direction::Down));
        }
    }
    fn left(&mut self, row: usize, col: usize) {
        if col > 0 {
            self.positions.push((row, col - 1, Direction::Left));
        }
    }
}

fn get_energized(
    grid: &[Vec<Option<Object>>],
    visited: &mut Visited,
    to_visit: &mut ToVisit,
) -> usize {
    while let Some((row, col, dir)) = to_visit.pop() {
        if visited.is_visited(row, col, dir) {
            continue;
        }

        visited.mark_as_visited(row, col, dir);

        use Direction::*;
        use Object::*;

        match (dir, grid[row][col]) {
            (Right, None | Some(HorizontalSplitter))
            | (Down, Some(RightDownMirror))
            | (Up, Some(RightUpMirror)) => {
                to_visit.right(row, col);
            }
            (Right | Left, Some(VerticalSplitter)) => {
                to_visit.up(row, col);
                to_visit.down(row, col);
            }
            (Right, Some(RightDownMirror))
            | (Left, Some(RightUpMirror))
            | (Down, None | Some(VerticalSplitter)) => to_visit.down(row, col),
            (Down | Up, Some(HorizontalSplitter)) => {
                to_visit.left(row, col);
                to_visit.right(row, col);
            }
            (Right, Some(RightUpMirror))
            | (Up, None | Some(VerticalSplitter))
            | (Left, Some(RightDownMirror)) => to_visit.up(row, col),
            (Up, Some(RightDownMirror))
            | (Down, Some(RightUpMirror))
            | (Left, None | Some(HorizontalSplitter)) => to_visit.left(row, col),
        }
    }
    visited
        .visited
        .iter()
        .flat_map(|row| row.iter())
        .filter(|col| **col > 0)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "46");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "7199");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "51");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "7438");
    }
}
