use advent_utils::Grid;

pub fn solve_part_1(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content);
    let pos = grid
        .coords()
        .find(|x| grid.get(x.0, x.1).copied().unwrap() == b'^')
        .unwrap();

    let (positions, _) = traverse(&grid, pos, Dir::Up);

    positions.into_iter().filter(|x| *x).count()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let mut grid = Grid::from_ascii_grid(file_content);
    let pos = grid
        .coords()
        .find(|x| grid.get(x.0, x.1).copied().unwrap() == b'^')
        .unwrap();

    let (positions, _) = traverse(&grid, pos, Dir::Up);

    positions
        .coords()
        .filter(|p| positions.get(p.0, p.1).copied() == Some(true))
        .filter(|(r, c)| {
            grid.set(*r, *c, b'#');
            let (_, has_loop) = traverse(&grid, pos, Dir::Up);
            grid.set(*r, *c, b'.');
            has_loop
        })
        .count()
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Dir {
    Up,
    Right,
    Down,
    Left,
}

impl Dir {
    fn apply(&self, pos: (usize, usize)) -> Option<(usize, usize)> {
        match self {
            Dir::Up => pos.0.checked_sub(1).map(|r| (r, pos.1)),
            Dir::Right => pos.1.checked_add(1).map(|c| (pos.0, c)),
            Dir::Down => pos.0.checked_add(1).map(|r| (r, pos.1)),
            Dir::Left => pos.1.checked_sub(1).map(|c| (pos.0, c)),
        }
    }
    fn next(&self) -> Dir {
        match self {
            Dir::Up => Self::Right,
            Dir::Right => Self::Down,
            Dir::Down => Self::Left,
            Dir::Left => Self::Up,
        }
    }
}

#[derive(Default)]
struct State {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl State {
    fn visit(&mut self, dir: Dir) -> bool {
        match dir {
            Dir::Up => {
                let prev = self.up;
                self.up = true;
                prev
            }
            Dir::Right => {
                let prev = self.right;
                self.right = true;
                prev
            }
            Dir::Down => {
                let prev = self.down;
                self.down = true;
                prev
            }
            Dir::Left => {
                let prev = self.left;
                self.left = true;
                prev
            }
        }
    }
}

fn traverse(grid: &Grid<u8>, mut pos: (usize, usize), mut dir: Dir) -> (Grid<bool>, bool) {
    let mut positions = grid.map(|_, _, _| false);
    let mut states = grid.map(|_, _, _| State::default());
    let mut has_loop = false;
    loop {
        if let Some(s) = states.get_mut(pos.0, pos.1) {
            if s.visit(dir) {
                has_loop = true;
                break;
            }
        }
        positions.set(pos.0, pos.1, true);
        let Some(next_pos) = dir.apply(pos) else {
            break;
        };
        let Some(next_cell) = grid.get(next_pos.0, next_pos.1).copied() else {
            break;
        };
        if next_cell == b'#' {
            dir = dir.next();
            continue;
        }
        pos = next_pos;
    }
    (positions, has_loop)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "41");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "5086");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "6");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1770");
    }
}
