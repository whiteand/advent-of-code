use std::collections::hash_map::Entry;

use advent_utils::nom::{
    self,
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, multispace1},
    combinator::all_consuming,
    multi::separated_list1,
    parse_usize,
    sequence::{delimited, preceded},
};
use fxhash::FxHashMap;
use itertools::Itertools;
use petgraph::{graph::NodeIndex, prelude::DiGraph, visit::EdgeRef};
use tracing::info;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> String {
    let (graph, _, _) = parse_graph(file_content);

    find_root(&graph).unwrap().to_string()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let (graph, weights, ids) = parse_graph(file_content);

    let root = find_root(&graph).unwrap();

    let root_id = ids.get(root).copied().unwrap();

    let (_actual, expected) = get_balanced_size(&graph, &weights, root_id).unwrap_or_default();
    info!(?_actual, ?expected);
    expected
}

fn get_balanced_size<'t>(
    graph: &DiGraph<&'t str, ()>,
    weights: &FxHashMap<&str, usize>,
    id: NodeIndex,
) -> Option<(usize, usize)> {
    for c in graph.edges(id) {
        if let Some(s) = get_balanced_size(graph, weights, c.target()) {
            return Some(s);
        }
    }
    let mut children_weights = graph
        .edges(id)
        .map(|e| e.target())
        .map(|id| {
            let name = graph.node_weight(id).copied().unwrap();
            let w = weights.get(name).copied().unwrap();
            w
        })
        .collect_vec();

    children_weights.sort_unstable();

    if children_weights.last() == children_weights.first() {
        return None;
    }
    let first = children_weights.first().copied().unwrap();
    let last = children_weights.last().copied().unwrap();
    assert!(children_weights.len() > 2);
    let middle = children_weights.get(1).copied().unwrap();
    let actual = if middle == first { last } else { first };
    Some((actual, middle))
}

fn find_root<'t>(graph: &DiGraph<&'t str, ()>) -> Option<&'t str> {
    for id in graph.node_indices() {
        if graph
            .edges_directed(id, petgraph::Direction::Incoming)
            .count()
            == 0
        {
            return Some(graph.node_weight(id).unwrap());
        }
    }
    None
}
fn parse_graph(
    file_content: &str,
) -> (
    DiGraph<&str, ()>,
    FxHashMap<&str, usize>,
    FxHashMap<&str, NodeIndex>,
) {
    let node_specs = parse_input(file_content).map(|x| x.1).unwrap();
    let mut graph = DiGraph::<&str, ()>::new();
    let mut weights = FxHashMap::default();
    let mut ids = FxHashMap::default();
    for x in node_specs {
        weights.insert(x.name, x.weight);
        for name in std::iter::once(x.name).chain(x.children.iter().copied()) {
            if let Entry::Vacant(e) = ids.entry(name) {
                e.insert(graph.add_node(name));
            }
        }
        let id = ids.get(x.name).copied().unwrap();
        for c in x.children.iter().copied() {
            let c_id = ids.get(c).copied().unwrap();
            graph.add_edge(id, c_id, ());
        }
    }
    (graph, weights, ids)
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct NodeSpec<'t> {
    name: &'t str,
    weight: usize,
    children: Vec<&'t str>,
}

fn parse_input(input: &str) -> nom::IResult<&str, Vec<NodeSpec<'_>>> {
    all_consuming(separated_list1(line_ending, parse_node_spec))(input.trim())
}
fn parse_node_spec(input: &str) -> nom::IResult<&str, NodeSpec<'_>> {
    let (input, name) = alpha1(input)?;
    let (input, _) = multispace1(input)?;
    let (input, weight) = delimited(tag("("), parse_usize, tag(")"))(input)?;
    if input.is_empty() || input.starts_with('\n') {
        return Ok((
            input,
            NodeSpec {
                name,
                weight,
                children: Vec::new(),
            },
        ));
    }
    let (input, children) = preceded(tag(" -> "), separated_list1(tag(", "), alpha1))(input)?;

    Ok((
        input,
        NodeSpec {
            name,
            weight,
            children,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "tknk")]
    #[case::actual(ACTUAL, "qibuqqg")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "60")]
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
