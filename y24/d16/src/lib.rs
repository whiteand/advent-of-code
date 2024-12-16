use advent_utils::{glam::IVec2, grid::Grid, parse};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let (grid, start, end) = parse(file_content);
    let (_, score) = pathfinding::prelude::astar(
        &(start, IVec2::X),
        |(p, dir)| next(&grid, *p, *dir),
        |(y, _)| heuristic(end, y),
        |(x, _)| *x == end,
    )
    .unwrap();

    score as usize
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let (grid, start, end) = parse(file_content);
    let (best, _) = pathfinding::prelude::astar_bag(
        &(start, IVec2::X),
        |(p, dir)| next(&grid, *p, *dir),
        |(y, _)| heuristic(end, y),
        |(x, _)| *x == end,
    )
    .unwrap();

    best.into_iter().flatten().map(|(x, _)| x).unique().count()
}

fn next(grid: &Grid<u8>, p: IVec2, dir: IVec2) -> impl Iterator<Item = ((IVec2, IVec2), i32)> + '_ {
    [
        (dir, 1),
        (dir.rotate(IVec2::Y), 1001),
        (dir.rotate(IVec2::NEG_Y), 1001),
    ]
    .map(|d| ((p + d.0, d.0), d.1))
    .into_iter()
    .filter(|((p, _), _)| grid.get(*p).map_or(false, |x| *x != b'#'))
}

fn heuristic(end: IVec2, x: &IVec2) -> i32 {
    (end - *x).abs().dot(IVec2::splat(1))
}

fn parse(file_content: &str) -> (Grid<u8>, IVec2, IVec2) {
    let grid = parse::ascii_grid(file_content);
    let start = IVec2::new(1, (grid.rows_len() - 2) as i32);
    let end = IVec2::new((grid.cols(1) - 2) as i32, 1);
    (grid, start, end)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "7036");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "111480");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "45");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "529");
    }
}
