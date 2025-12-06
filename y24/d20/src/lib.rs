use advent_utils::{
    glam::IVec2,
    grid::{Grid, NonDiagonal},
    parse,
};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve<const MIN_WIN: usize, const CHEAT_LEN: usize>(file_content: &str) -> usize {
    let (grid, start, end) = parse_input(file_content);
    let mut distance_to_finish_without_cheat = Grid::<Option<usize>>::new(grid.size(), None);

    for x in pathfinding::prelude::dijkstra_reach(&end, |pos, _| {
        grid.neighbours(*pos, NonDiagonal)
            .filter_map(|(p, &v)| (v != b'#').then_some((p, 1)))
    }) {
        distance_to_finish_without_cheat.set(x.node, Some(x.total_cost));
    }

    restore_path(&distance_to_finish_without_cheat, start)
        .collect_vec()
        .into_iter()
        .tuple_combinations()
        .filter(|((start, without_cheat), (e, c))| {
            let d = dist(*e, *start);
            if d > CHEAT_LEN {
                return false;
            }
            let new_cost = *c + d;

            without_cheat
                .checked_sub(new_cost)
                .is_some_and(|x| x >= MIN_WIN)
        })
        .count()
}

fn restore_path(
    dist: &Grid<Option<usize>>,
    pos: IVec2,
) -> impl Iterator<Item = (IVec2, usize)> + '_ {
    let cur_d = dist.get(pos).unwrap().unwrap();
    std::iter::successors(Some((pos, cur_d)), |&(x, mut d)| {
        if d == 0 {
            return None;
        }
        d -= 1;
        dist.neighbours(x, NonDiagonal)
            .find_map(|(p, x)| x.as_ref().copied().and_then(|x| (x == d).then_some((p, d))))
    })
}

fn dist(a: IVec2, b: IVec2) -> usize {
    (a - b).abs().dot(IVec2::splat(1)) as usize
}

fn parse_input(file_content: &str) -> (Grid<u8>, IVec2, IVec2) {
    let mut grid = parse::ascii_grid(file_content.trim());
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
