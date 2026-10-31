use rustc_hash::{FxHashMap, FxHashSet};

use advent_utils::{glam::IVec2, grid::N8};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(file_content);
    let mut elves_positions = grid
        .entries_copy()
        .filter(|(_, v)| *v == b'#')
        .map(|(p, _)| p)
        .collect::<FxHashSet<_>>();

    for i in 0..10 {
        make_iteration(&mut elves_positions, i);
    }

    get_part1_score(&elves_positions)
}

const DIRECTIONS: [[IVec2; 3]; 4] = [
    [IVec2::new(0, -1), IVec2::new(-1, -1), IVec2::new(1, -1)],
    [IVec2::new(0, 1), IVec2::new(-1, 1), IVec2::new(1, 1)],
    [IVec2::new(-1, 0), IVec2::new(-1, -1), IVec2::new(-1, 1)],
    [IVec2::new(1, 0), IVec2::new(1, -1), IVec2::new(1, 1)],
];
fn make_iteration(elves_positions: &mut FxHashSet<IVec2>, iteration: usize) -> usize {
    let half_round_actions = elves_positions
        .iter()
        .copied()
        .map(|p| {
            if !N8::neighbours_of(p)
                .iter()
                .any(|x| elves_positions.contains(&x))
            {
                return None;
            }

            for side in DIRECTIONS
                .iter()
                .cycle()
                .skip(iteration % DIRECTIONS.len())
                .take(DIRECTIONS.len())
            {
                if side.iter().all(|x| !elves_positions.contains(&(x + p))) {
                    return Some((p, side[0]));
                }
            }

            None
        })
        .filter_map(|x| x)
        .collect_vec();

    let mut movements = FxHashMap::default();
    let mut forbidden_positions = FxHashSet::default();

    for (prev, dir) in half_round_actions {
        let new_pos = prev + dir;
        if movements.contains_key(&new_pos) || forbidden_positions.contains(&new_pos) {
            movements.remove(&new_pos);
            forbidden_positions.insert(new_pos);
        } else {
            movements.insert(new_pos, prev);
        }
    }

    let moves = movements.len();

    for (new_p, prev_p) in movements {
        elves_positions.remove(&prev_p);
        elves_positions.insert(new_p);
    }
    moves
}

fn get_bounding_box(positions: &FxHashSet<IVec2>) -> [IVec2; 2] {
    let first = positions.iter().next().copied().unwrap();
    let mut left_top = first;
    let mut bottom_right = first;
    for p in positions.iter().copied() {
        left_top = left_top.min(p);
        bottom_right = bottom_right.max(p);
    }

    [left_top, bottom_right]
}

fn get_part1_score(positions: &FxHashSet<IVec2>) -> usize {
    let [left_top, bottom_right] = get_bounding_box(positions);
    let total_area = ((bottom_right - left_top) + IVec2::new(1, 1)).element_product() as usize;
    total_area - positions.len()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(file_content);
    let mut elves_positions = grid
        .entries_copy()
        .filter(|(_, v)| *v == b'#')
        .map(|(p, _)| p)
        .collect::<FxHashSet<_>>();

    for i in 0.. {
        let moves = make_iteration(&mut elves_positions, i);
        if moves == 0 {
            return i + 1;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "110")]
    #[case::actual(ACTUAL, "4049")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "20")]
    #[case::actual(ACTUAL, "1021")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
