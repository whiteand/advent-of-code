use advent_utils::{glam::IVec2, grid::Grid, parse};

pub fn solve_part_1(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content.trim());
    let pos = grid
        .coords()
        .find(|x| grid.get(*x).copied().unwrap() == b'^')
        .unwrap();

    let (positions, _) = traverse(&grid, pos, Dir::Up);

    positions.into_iter().filter(|x| *x).count()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let mut grid = parse::ascii_grid(file_content.trim());
    let pos = grid
        .coords()
        .find(|x| grid.get(*x).copied().unwrap() == b'^')
        .unwrap();

    let (positions, _) = traverse(&grid, pos, Dir::Up);

    positions
        .coords()
        .filter(|p| positions.get(*p).copied() == Some(true))
        .filter(|p| {
            grid.set(*p, b'#');
            let (_, has_loop) = traverse(&grid, pos, Dir::Up);
            grid.set(*p, b'.');
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
    fn apply(&self, pos: IVec2) -> Option<IVec2> {
        match self {
            Dir::Up => (pos.y > 0).then_some(pos + IVec2::NEG_Y),
            Dir::Right => Some(pos + IVec2::X),
            Dir::Down => Some(pos + IVec2::Y),
            Dir::Left => (pos.x > 0).then_some(pos + IVec2::NEG_X),
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

fn traverse(grid: &Grid<u8>, mut pos: IVec2, mut dir: Dir) -> (Grid<bool>, bool) {
    let mut positions = grid.map(|_, _| false);
    let mut states = grid.map(|_, _| State::default());
    let mut has_loop = false;
    loop {
        if let Some(s) = states.get_mut(pos) {
            if s.visit(dir) {
                has_loop = true;
                break;
            }
        }
        positions.set(pos, true);
        let Some(next_pos) = dir.apply(pos) else {
            break;
        };
        let Some(next_cell) = grid.get(next_pos).copied() else {
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
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1770");
    }
}
