use advent_utils::{glam::IVec2, grid::Grid, parse};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve<const MIN_WIN: usize, const CHEAT_LEN: usize>(file_content: &str) -> usize {
    let (grid, start, end) = parse_input(file_content);
    let mut distance_to_finish_without_cheat = Grid::<Option<usize>>::new(grid.size(), None);

    for x in pathfinding::prelude::dijkstra_reach(&end, |pos, _| {
        grid.neighbours(*pos, [IVec2::NEG_X, IVec2::X, IVec2::Y, IVec2::NEG_Y])
            .filter_map(|(p, &v)| (v != b'#').then_some((p, 1)))
    }) {
        distance_to_finish_without_cheat.set(x.node, Some(x.total_cost));
    }

    let valid_points = distance_to_finish_without_cheat
        .entries()
        .filter_map(|(pos, v)| v.map(|v| (pos, v)))
        .collect_vec();

    let (starts, _) = find_path(&grid, start, end);

    let mut total = 0;
    for start in starts.iter() {
        let start = *start;
        let without_cheat = distance_to_finish_without_cheat
            .get(start)
            .unwrap()
            .as_ref()
            .copied()
            .unwrap();

        total += valid_points
            .iter()
            .copied()
            .filter(|(e, c)| {
                let d = dist(*e, start);
                if d > CHEAT_LEN {
                    return false;
                }
                let new_cost = *c + d;

                without_cheat
                    .checked_sub(new_cost)
                    .map_or(false, |x| x >= MIN_WIN)
            })
            .count();
    }

    total
}

fn dist(a: IVec2, b: IVec2) -> usize {
    (a - b).abs().dot(IVec2::splat(1)) as usize
}

fn parse_input(file_content: &str) -> (Grid<u8>, IVec2, IVec2) {
    let mut grid = parse::ascii_grid(file_content);
    let s = grid
        .entries()
        .find_map(|(c, b)| (*b == b'S').then_some(c))
        .expect("should be found");
    let e = grid
        .entries()
        .find_map(|(c, b)| (*b == b'E').then_some(c))
        .expect("should be found");

    grid.set(s, b'.');
    grid.set(e, b'.');

    (grid, s, e)
}

fn find_path(grid: &Grid<u8>, start: IVec2, end: IVec2) -> (Vec<IVec2>, usize) {
    pathfinding::prelude::dijkstra(
        &start,
        |x| {
            grid.neighbours(*x, [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y])
                .filter_map(|(p, &b)| (b != b'#').then_some((p, 1)))
        },
        |x| *x == end,
    )
    .expect("should be present")
}

#[cfg(test)]
mod tests {
    use super::solve;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve::<2, 2>(EXAMPLE)), "44");
        assert_eq!(format!("{}", solve::<4, 2>(EXAMPLE)), "30");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve::<100, 2>(ACTUAL)), "1502");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve::<50, 20>(EXAMPLE)), "285");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve::<100, 20>(ACTUAL)), "1028136");
    }
}
