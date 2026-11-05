use std::collections::{BTreeMap, HashMap};

use advent_utils::glam::IVec3;
use itertools::Itertools;

use crate::model::{Orientation, ScannerPositions};

use super::parse::parse_scanners;

#[tracing::instrument(skip(file_content))]
pub fn part1(mut file_content: &str) -> usize {
    let scanners = parse_scanners(&mut file_content).unwrap();
    let distances = scanners
        .iter()
        .map(|scanner| scanner.distances())
        .collect_vec();

    const MIN_SIMILARITY: usize = 12;
    let connections = distances
        .iter()
        .array_combinations::<2>()
        .map(|[d1, d2]| d1.get_same_distances(d2))
        .filter(|matching_distances| {
            matching_distances.matches[0].len() >= MIN_SIMILARITY * (MIN_SIMILARITY - 1) / 2
        })
        .collect_vec();

    let aliases = connections.iter().map(|c| c.get_aliases()).collect_vec();

    let mut scanner_positions = ScannerPositions::new();

    scanner_positions.set_position(scanners[0].id, Orientation::identity());

    while !aliases.is_empty() {
        for alias in &aliases {
            let scanners = alias
                .scanner_ids
                .map(|id| scanners.iter().find(|x| x.id == id).unwrap());
            if let Some(left_pos) = scanner_positions.get_position(alias.scanner_ids[0]) {
                todo!();
                // scanner_positions.infer_position(
                //     alias,
                //     0,
                //     1,
                //     left_pos,
                //     left_pos,
                //     &scanners[0].measurements,
                //     &alias.aliases,
                //     &scanners[1].measurements,
                // );
            }
        }
        let alias = aliases.iter().flat_map(|a| {
            a.scanner_ids
                .iter()
                .copied()
                .flat_map(|i| scanner_positions.get_position(i))
        });
    }

    println!("{:?}", aliases);

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
