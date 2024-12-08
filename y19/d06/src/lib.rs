use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::{self, complete::line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};
use petgraph::{
    graph::{DiGraph, NodeIndex},
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
    let parent_map = pairs
        .iter()
        .copied()
        .map(|(k, v)| (v, k))
        .collect::<HashMap<_, _>>();

    let mut graph = petgraph::graph::UnGraph::new_undirected();
    let mut parent_idx_map = HashMap::<NodeIndex, NodeIndex>::new();

    let mut node_idxs: HashMap<&str, NodeIndex> = HashMap::new();

    for a in pairs.iter().copied().flat_map(|(a, b)| [a, b]) {
        match node_idxs.entry(a) {
            Entry::Occupied(_) => {
                continue;
            }
            Entry::Vacant(e) => {
                let dir_node_idx = graph.add_node(a);
                e.insert(dir_node_idx);
            }
        };
    }
    for (a, b) in parent_map {
        let parent = node_idxs.get(b).copied().unwrap();
        let child = node_idxs.get(a).copied().unwrap();
        parent_idx_map.insert(child, parent);
    }
    for (a, b) in pairs {
        let a_idx = node_idxs.get(a).copied().unwrap();
        let b_idx = node_idxs.get(b).copied().unwrap();
        graph.add_edge(a_idx, b_idx, 1usize);
    }

    let you_idx = node_idxs.get("YOU").copied().unwrap();
    let san_idx = node_idxs.get("SAN").copied().unwrap();
    let you_parent_idx = parent_idx_map.get(&you_idx).copied().unwrap();
    let san_parent_idx = parent_idx_map.get(&san_idx).copied().unwrap();

    let distances =
        petgraph::algo::dijkstra::dijkstra(&graph, you_parent_idx, Some(san_parent_idx), |_| 1);

    distances
        .get(&san_parent_idx)
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
