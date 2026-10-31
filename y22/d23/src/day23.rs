use std::collections::HashSet;

use advent_utils::{glam::IVec2, grid::N8};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(file_content);
    println!("{}", grid.render_ascii());
    let elves_positions = grid
        .entries_copy()
        .filter(|(_, v)| *v == b'#')
        .map(|(p, _)| p)
        .collect::<HashSet<_>>();

    let iteration = 0;

    let half_round_actions = elves_positions
        .iter()
        .copied()
        .map(|p| {
            let neighbours = N8::neighbours_of(p).map(|x| elves_positions.contains(&x));

            // If no other Elves are in one of those eight positions,
            // the Elf does not do anything during this round.
            if !neighbours.contains(&true) {
                return None;
            }
            let [nw, n, ne, w, e, sw, s, se] = neighbours;

            let dirs = [
                ([n, ne, nw], IVec2::NEG_Y),
                ([s, se, sw], IVec2::Y),
                ([w, nw, sw], IVec2::NEG_X),
                ([e, ne, se], IVec2::X),
            ];

            for (has, dir) in dirs
                .iter()
                .cycle()
                .copied()
                .skip(iteration % dirs.len())
                .take(dirs.len())
            {
                if !has.contains(&true) {
                    return Some((p, dir));
                }
            }

            None
        })
        .filter_map(|x| x)
        .collect_vec();

    println!("{half_round_actions:?}");

    file_content.len()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
