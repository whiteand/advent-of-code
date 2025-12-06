use advent_utils::{grid::Grid, parse};
use glam::IVec2;
use itertools::Itertools;

fn get_dirs() -> [IVec2; 8] {
    [
        IVec2::NEG_X + IVec2::NEG_Y,
        IVec2::NEG_X,
        IVec2::NEG_X + IVec2::Y,
        IVec2::NEG_Y,
        IVec2::Y,
        IVec2::X + IVec2::NEG_Y,
        IVec2::X,
        IVec2::X + IVec2::Y,
    ]
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content.trim());

    let mut total = 0;
    for pos in grid.coords() {
        total += get_dirs()
            .iter()
            .copied()
            .filter(|dir| grid.matches("XMAS", pos, *dir))
            .count();
    }
    total
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content.trim());

    let mut total = 0;
    for pos in grid.coords() {
        if !grid.matches("MAS", pos, IVec2::new(1, 1))
            && !grid.matches("SAM", pos, IVec2::new(1, 1))
        {
            continue;
        }
        if !grid.matches("MAS", pos + IVec2::new(0, 2), IVec2::new(1, -1))
            && !grid.matches("SAM", pos + IVec2::new(0, 2), IVec2::new(1, -1))
        {
            continue;
        }
        total += 1;
    }
    total
}

trait GridExt {
    fn matches(&self, str: &str, pos: IVec2, dir: IVec2) -> bool;
}

impl GridExt for Grid<u8> {
    fn matches(&self, str: &str, pos: IVec2, dir: IVec2) -> bool {
        self.iter_line(pos, dir)
            .zip_longest(str.as_bytes())
            .take_while(|r| r.has_right())
            .all(|r| r.both().is_some_and(|(a, b)| a == b))
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "18");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "2344");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "9");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1815");
    }
}
