use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{
    graph::{DiGraph, UnGraph},
    visit::IntoNodeIdentifiers,
};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let (_, pairs) = parse(file_content).unwrap();
    let planets = pairs
        .iter()
        .flat_map(|(a, b)| [*a, *b].into_iter())
        .unique()
        .enumerate()
        .map(|(i, x)| (x, i))
        .collect::<HashMap<_, _>>();

    let dir_graph: DiGraph<usize, usize, usize> =
        petgraph::graph::DiGraph::from_edges(pairs.into_iter().map(|(parent, child)| {
            (
                planets.get(child).copied().unwrap(),
                planets.get(parent).copied().unwrap(),
                1usize,
            )
        }));

    dir_graph
        .node_identifiers()
        .map(|planet_idx| {
            let mut parent = dir_graph.neighbors(planet_idx).next();
            let mut res = 0;
            while let Some(p_idx) = parent {
                res += 1;
                parent = dir_graph.neighbors(p_idx).next();
            }
            res
        })
        .sum()
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let (_, pairs) = parse(file_content).unwrap();
    let planets_idx = pairs
        .iter()
        .flat_map(|(a, b)| [a, b].into_iter().copied())
        .enumerate()
        .map(|(i, p)| (p, i))
        .collect::<HashMap<_, _>>();
    let parent_map = pairs
        .iter()
        .copied()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();

    let graph: UnGraph<usize, (), usize> =
        petgraph::graph::UnGraph::from_edges(pairs.iter().copied().map(|(a, b)| {
            (
                planets_idx.get(a).copied().unwrap(),
                planets_idx.get(b).copied().unwrap(),
                (),
            )
        }));

    let you_parent = parent_map.get("YOU").copied().unwrap();
    let san_parent = parent_map.get("SAN").copied().unwrap();
    let you_parent_idx = planets_idx.get(you_parent).copied().unwrap();
    let san_parent_idx = planets_idx.get(san_parent).copied().unwrap();

    let distances = petgraph::algo::dijkstra::dijkstra(
        &graph,
        you_parent_idx.into(),
        Some(san_parent_idx.into()),
        |_| 1,
    );

    distances
        .get(&san_parent_idx.into())
        .copied()
        .unwrap_or(usize::MAX)
}

fn parse(input: &str) -> IResult<&str, Vec<(&str, &str)>> {
    separated_list1(
        line_ending,
        separated_pair(
            character::complete::alphanumeric1,
            tag(")"),
            character::complete::alphanumeric1,
        ),
    )(input)
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "42");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "271151");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(
            format!(
                "{}",
                solve_part_2(
                    r#"COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN"#
                )
            ),
            "4"
        );
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "388");
    }
}
