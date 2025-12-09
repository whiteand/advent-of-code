use std::collections::BinaryHeap;

use advent_utils::{
    algo::IntoTopArrayIteratorExt, disjoint_set::DisjointSets, glam::I64Vec3, parse,
};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (points, mut disjoint_sets, mut possible_connections) = parse_and_prepare(file_content);

    std::iter::from_fn(|| possible_connections.pop())
        .take(points.len())
        .for_each(|c| {
            disjoint_sets.join(c.i, c.j);
        });

    disjoint_sets
        .sizes()
        .into_top_array::<3>()
        .expect("shoudl be present")
        .into_iter()
        .product()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let (points, mut disjoint_sets, mut possible_connections) = parse_and_prepare(file_content);

    std::iter::from_fn(|| possible_connections.pop())
        .take_while_inclusive(|c| disjoint_sets.join(c.i, c.j).size != points.len())
        .last()
        .map(|c| (points[c.i].x * points[c.j].x) as usize)
        .unwrap()
}

fn parse_and_prepare(input: &str) -> (Vec<I64Vec3>, DisjointSets, BinaryHeap<PossibleConnection>) {
    let points = input
        .trim()
        .lines()
        .map(|line| {
            parse::nums::<i64>(line)
                .collect_tuple()
                .map(|(x, y, z)| I64Vec3::new(x, y, z))
                .unwrap()
        })
        .collect_vec();

    let disjoint_sets = DisjointSets::new(points.len());

    let mut possible_connections = BinaryHeap::with_capacity(points.len() * (points.len() - 1) / 2);

    possible_connections.extend(points.iter().copied().enumerate().tuple_combinations().map(
        |((i, p1), (j, p2))| PossibleConnection {
            i,
            j,
            distance_squared: p1.distance_squared(p2),
        },
    ));

    (points, disjoint_sets, possible_connections)
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct PossibleConnection {
    i: usize,
    j: usize,
    distance_squared: i64,
}

impl PartialOrd for PossibleConnection {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PossibleConnection {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance_squared.cmp(&self.distance_squared)
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");

    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case::example(EXAMPLE, "45")]
    #[case::actual(ACTUAL, "122430")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "25272")]
    #[case::actual(ACTUAL, "8135565324")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
