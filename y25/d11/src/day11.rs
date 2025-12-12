use rustc_hash::FxHashMap;
use std::hash::RandomState;

use itertools::Itertools;
use petgraph::graph::{DiGraph, NodeIndex};

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let graph = parse_graph(file_content);
    graph.paths_from_to("you", "out")
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let graph = parse_graph(file_content);
    graph.count_paths_from_to_visiting("svr", "out", &["dac", "fft"])
        + graph.count_paths_from_to_visiting("svr", "out", &["fft", "dac"])
}

struct StringGraph<'i> {
    graph: DiGraph<&'i str, ()>,
    nodes: FxHashMap<&'i str, NodeIndex>,
}

impl<'i> StringGraph<'i> {
    fn paths_from_to(&self, from: &str, to: &str) -> usize {
        let you_node = self.nodes.get(from).copied().unwrap();
        let out_node = self.nodes.get(to).copied().unwrap();
        petgraph::algo::all_simple_paths::<Discard, _, RandomState>(
            &self.graph,
            you_node,
            out_node,
            0,
            None,
        )
        .count()
    }

    fn get_index(&self, name: &str) -> NodeIndex {
        self.nodes.get(name).copied().unwrap()
    }

    fn raw_count_paths_from_to_visiting<'a>(
        &self,
        from: NodeIndex,
        to: NodeIndex,
        visiting: &'a [NodeIndex],
        visited: &mut Vec<NodeIndex>,
        memory: &mut FxHashMap<(NodeIndex, NodeIndex, &'a [NodeIndex]), usize>,
    ) -> usize {
        if visited.contains(&to) {
            return 0;
        }

        // from = 0 to = 10, visiting = [5, 10] ->
        // from = 0 to = 10, visiting = [5];
        //
        if let Some((last, init)) = visiting.split_last()
            && to == *last
        {
            return self.raw_count_paths_from_to_visiting(from, to, init, visited, memory);
        }
        if from == to {
            return if visiting.is_empty() { 1 } else { 0 };
        }

        if let Some(n) = memory.get(&(from, to, visiting)).copied() {
            return n;
        }

        visited.push(to);

        let cnt = self
            .graph
            .neighbors_directed(to, petgraph::Direction::Incoming)
            .map(|i| self.raw_count_paths_from_to_visiting(from, i, visiting, visited, memory))
            .sum::<usize>();

        visited.pop();

        memory.insert((from, to, visiting), cnt);

        cnt
    }

    fn count_paths_from_to_visiting(&self, from: &str, to: &str, visiting: &[&str]) -> usize {
        let from_ind = self.get_index(from);
        let to_ind = self.get_index(to);
        let visiting = visiting.iter().map(|x| self.get_index(x)).collect_vec();
        let mut from_to_visiting = FxHashMap::default();
        let mut visited = Vec::with_capacity(self.nodes.len());

        self.raw_count_paths_from_to_visiting(
            from_ind,
            to_ind,
            &visiting,
            &mut visited,
            &mut from_to_visiting,
        )
    }
}

fn parse_graph(file_content: &str) -> StringGraph<'_> {
    let edges_strings = file_content
        .trim()
        .lines()
        .flat_map(|line| {
            let (first, second) = line.split_once(':').unwrap();
            second
                .trim()
                .split_ascii_whitespace()
                .map(move |second| (first, second))
        })
        .collect_vec();

    let node_strings = edges_strings
        .iter()
        .copied()
        .flat_map(|(a, b)| [a, b])
        .unique()
        .collect_vec();

    let mut nodes = FxHashMap::<&str, NodeIndex>::default();

    let mut graph = petgraph::graph::DiGraph::<&str, ()>::with_capacity(
        edges_strings.len(),
        edges_strings.len(),
    );

    for n in &node_strings {
        let node = graph.add_node(n);
        nodes.insert(n, node);
    }

    for (a, b) in &edges_strings {
        let a = nodes.get(a).copied().unwrap();
        let b = nodes.get(b).copied().unwrap();
        graph.add_edge(a, b, ());
    }

    StringGraph { graph, nodes }
}

struct Discard;
impl<V> FromIterator<V> for Discard {
    fn from_iter<T: IntoIterator<Item = V>>(_: T) -> Self {
        Discard
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "5")]
    #[case::actual(ACTUAL, "640")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(
        "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out",
        "2"
    )]
    #[case::actual(ACTUAL, "367579641755680")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
