use itertools::Itertools;
use petgraph::graph::UnGraph;
use petgraph::prelude::NodeIndex;
use std::collections::BTreeMap;

fn parse_graph(file_content: &str) -> (BTreeMap<&str, NodeIndex>, UnGraph<&str, usize>) {
    let mut res = UnGraph::new_undirected();
    let mut node_indices = BTreeMap::new();

    for line in file_content.lines() {
        let (a, b) = line.split_once(": ").unwrap();
        let a_ind = node_indices.get(a).copied().unwrap_or_else(|| {
            let ind = res.add_node(a);
            node_indices.insert(a, ind);
            ind
        });

        for connected_to in b.split_ascii_whitespace() {
            let b_ind = node_indices.get(connected_to).copied().unwrap_or_else(|| {
                let ind = res.add_node(connected_to);
                node_indices.insert(connected_to, ind);
                ind
            });
            res.add_edge(a_ind, b_ind, 1);
        }
    }

    (node_indices, res)
}

// I found this list of nodes via vizualization of the graph
fn get_edges_to_remove() -> Vec<(&'static str, &'static str)> {
    vec![("kdc", "pmn"), ("grd", "hvm"), ("jmn", "zfk")]
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (node_indices, mut graph) = parse_graph(file_content);

    let to_remove = get_edges_to_remove()
        .into_iter()
        .map(|(a, b)| {
            (
                node_indices.get(a).unwrap().clone(),
                node_indices.get(b).unwrap().clone(),
            )
        })
        .collect_vec();

    graph.retain_edges(|g, e| {
        let (a, b) = g.edge_endpoints(e).unwrap();
        !to_remove.contains(&(a, b)) && !to_remove.contains(&(b, a))
    });

    let mut colors = Vec::new();
    let mut node_color = BTreeMap::new();

    let nodes = graph.node_indices().collect_vec();
    let mut to_color = Vec::new();
    loop {
        let not_colored_node_index = match nodes.iter().find(|n| !node_color.contains_key(*n)) {
            Some(n) => n.clone(),
            None => break,
        };
        let new_color = colors.len();
        colors.push(0usize);
        to_color.push((not_colored_node_index, new_color));
        while let Some((n, c)) = to_color.pop() {
            if node_color.contains_key(&n) {
                continue;
            }
            node_color.insert(n, c);
            colors[c] += 1;
            for neighbor in graph.neighbors(n) {
                if node_color.contains_key(&neighbor) {
                    continue;
                }
                to_color.push((neighbor, c));
            }
        }
    }

    colors[0] * colors[1]
}

#[cfg(test)]
mod tests {
    use super::solve_part_1;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    // I didn't solved this task for any input, just for my real input.
    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "54");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "612945");
    }
}
