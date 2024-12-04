use advent_utils::Grid;
use itertools::Itertools;

const DIRS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, 1),
    (0, -1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());

    let mut total = 0;
    for (i, j) in grid.coords() {
        total += DIRS
            .iter()
            .copied()
            .filter(|dir| grid.matches("XMAS", i, j, *dir))
            .count();
    }
    total
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());

    let mut total = 0;
    for (i, j) in grid.coords() {
        if !grid.matches("MAS", i, j, (1, 1)) && !grid.matches("SAM", i, j, (1, 1)) {
            continue;
        }
        if !grid.matches("MAS", i, j + 2, (1, -1)) && !grid.matches("SAM", i, j + 2, (1, -1)) {
            continue;
        }
        total += 1;
    }
    total
}

trait GridExt {
    fn matches(&self, str: &str, i: usize, j: usize, dir: (isize, isize)) -> bool;
}

impl GridExt for Grid<u8> {
    fn matches(&self, str: &str, i: usize, j: usize, dir: (isize, isize)) -> bool {
        self.iter_line(i as isize, j as isize, dir.0, dir.1)
            .zip_longest(str.as_bytes().into_iter())
            .take_while(|r| r.has_right())
            .all(|r| r.both().map_or(false, |(a, b)| a == b))
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    use tracing;
    use tracing_subscriber;
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
