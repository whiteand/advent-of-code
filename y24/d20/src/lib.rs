use advent_utils::{glam::IVec2, grid::Grid, parse};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1<const MIN_CHEAT_WIN: usize>(file_content: &str) -> usize {
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

    tracing::info!("\n{}", grid.render_ascii());

    let (path_without_cheat, without_cheat) = find_path(&grid, s, e);

    let possible_cheats = path_without_cheat
        .iter()
        .enumerate()
        .flat_map(|(i, p)| {
            [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y]
                .into_iter()
                .map(move |step| (i, *p, *p + step, *p + step * 2))
        })
        .filter_map(|(i, s, w, e)| {
            if grid.get(w) != Some(&b'#') || grid.get(e) != Some(&b'.') {
                return None;
            }
            let ep = path_without_cheat.iter().find_position(|x| **x == e);
            match ep {
                None => Some((s, w, e)),
                Some((ind, _)) if ind <= i => None,
                Some(_) => Some((s, w, e)),
            }
        })
        .collect_vec();

    let mut total = 0;

    for (_, wall, _) in possible_cheats {
        grid.set(wall, b'.');
        let (_, cost) = find_path(&grid, s, e);
        grid.set(wall, b'#');
        if cost < without_cheat && without_cheat - cost >= MIN_CHEAT_WIN {
            total += 1;
        }
    }

    total
}

fn find_path(grid: &Grid<u8>, start: IVec2, end: IVec2) -> (Vec<IVec2>, usize) {
    pathfinding::prelude::dijkstra(
        &start,
        |x| {
            grid.neighbours(*x, [IVec2::NEG_X, IVec2::X, IVec2::NEG_Y, IVec2::Y])
                .filter_map(|(p, &b)| {
                    let res = (b != b'#').then_some((p, 1));

                    // tracing::info!(?p, b = ?(b as char), ?res);

                    res
                })
        },
        |x| *x == end,
    )
    .expect("should be present")
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    file_content.len()
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
        assert_eq!(format!("{}", solve_part_1::<2>(EXAMPLE)), "44");
        assert_eq!(format!("{}", solve_part_1::<4>(EXAMPLE)), "30");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1::<100>(ACTUAL)), "1502");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
