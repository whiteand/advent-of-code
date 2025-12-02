use advent_utils::nom::{self, Parser};
use fxhash::FxHashMap;
use itertools::Itertools;
use petgraph::{
    graph::DiGraph,
    visit::{IntoNodeReferences, Walker},
};

#[derive(Copy, Clone, Debug)]
enum Operation {
    Add,
    Minus,
    Multiply,
    Divide,
}

impl Operation {
    fn perform(self, a: isize, b: isize) -> Option<isize> {
        match self {
            Operation::Add => Some(a + b),
            Operation::Minus => Some(a - b),
            Operation::Multiply => Some(a * b),
            Operation::Divide => {
                if b != 0 && a % b == 0 {
                    Some(a / b)
                } else {
                    None
                }
            }
        }
    }
}
impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operation::Add => write!(f, "+"),
            Operation::Minus => write!(f, "-"),
            Operation::Multiply => write!(f, "*"),
            Operation::Divide => write!(f, "/"),
        }
    }
}

#[derive(Debug, Clone)]
enum NodeValue<'i> {
    Int(isize),
    Binary {
        op: Operation,
        first: &'i str,
        second: &'i str,
    },
}

impl<'i> NodeValue<'i> {
    fn dependencies(&self) -> impl Iterator<Item = &'i str> {
        match self {
            NodeValue::Int(_) => itertools::Either::Left(std::iter::empty()),
            NodeValue::Binary { first, second, .. } => {
                itertools::Either::Right([*first, *second].into_iter())
            }
        }
    }
    fn calculate(&self, values: &FxHashMap<&str, isize>) -> Option<isize> {
        match self {
            NodeValue::Int(x) => Some(*x),
            NodeValue::Binary { first, second, op } => {
                match (values.get(first), values.get(second)) {
                    (Some(a), Some(b)) => Some(op.perform(*a, *b)?),
                    _ => None,
                }
            }
        }
    }
}

fn parse_operation(input: &str) -> nom::IResult<&str, Operation> {
    nom::branch::alt((
        nom::combinator::value(Operation::Add, nom::character::complete::char('+')),
        nom::combinator::value(Operation::Minus, nom::character::complete::char('-')),
        nom::combinator::value(Operation::Multiply, nom::character::complete::char('*')),
        nom::combinator::value(Operation::Divide, nom::character::complete::char('/')),
    ))
    .parse(input)
}
fn parse_binary_node_value(input: &str) -> nom::IResult<&str, NodeValue<'_>> {
    let operator_with_spaces = nom::sequence::delimited(
        nom::character::complete::space1,
        parse_operation,
        nom::character::complete::space1,
    );
    (parse_operand, operator_with_spaces, parse_operand)
        .map(|(a, b, c)| NodeValue::Binary {
            op: b,
            first: a,
            second: c,
        })
        .parse(input)
}
fn parse_int_node_value(input: &str) -> nom::IResult<&str, NodeValue<'_>> {
    nom::parse_isize.map(NodeValue::Int).parse(input)
}
fn parse_node(input: &str) -> nom::IResult<&str, Node<'_>> {
    nom::sequence::separated_pair(
        nom::character::complete::alpha1,
        (
            nom::bytes::complete::tag(":"),
            nom::character::complete::space0,
        ),
        parse_node_value,
    )
    .map(|(a, b)| Node { name: a, value: b })
    .parse(input)
}
fn parse_node_value(input: &str) -> nom::IResult<&str, NodeValue<'_>> {
    nom::branch::alt((parse_binary_node_value, parse_int_node_value)).parse(input)
}
fn parse_operand(input: &str) -> nom::IResult<&str, &str> {
    nom::character::complete::alpha1(input)
}

fn parse_nodes(input: &str) -> nom::IResult<&str, Vec<Node<'_>>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_node).parse(input)
}

#[derive(Clone)]
struct Node<'i> {
    name: &'i str,
    value: NodeValue<'i>,
}

impl std::fmt::Display for NodeValue<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeValue::Int(v) => write!(f, "{}", v),
            NodeValue::Binary { op, first, second } => write!(f, "{} {} {}", first, op, second),
        }
    }
}
impl std::fmt::Display for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, &self.value)
    }
}
impl std::fmt::Debug for Node<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

#[derive(Clone)]
struct ValueCalculator<'i> {
    node: Node<'i>,
    deps: Vec<Node<'i>>,
}

impl<'i> ValueCalculator<'i> {
    fn build(value: &'i str, nodes: &[Node<'i>], graph: &DiGraph<&'i str, ()>) -> Self {
        let root_node = graph
            .node_references()
            .find(|(_, &n)| n == value)
            .map(|(id, _)| id)
            .unwrap();
        let mut deps = petgraph::visit::Bfs::new(graph, root_node)
            .iter(&graph)
            .map(|n| graph.node_weight(n).copied().unwrap())
            .map(|name| nodes.iter().find(|n| n.name == name).cloned().unwrap())
            .collect_vec();

        deps.reverse();

        Self {
            node: nodes.iter().find(|n| n.name == value).cloned().unwrap(),
            deps,
        }
    }

    fn calculate(&self, values: &mut FxHashMap<&'i str, isize>) -> Option<isize> {
        for d in self.deps.iter() {
            if !values.contains_key(&d.name) {
                let v = d.value.calculate(values)?;
                values.insert(d.name, v);
            }
        }
        if values.contains_key(self.node.name) {
            values.get(self.node.name).copied()
        } else {
            let value = self.node.value.calculate(values)?;
            values.insert(self.node.name, value);
            Some(value)
        }
    }
}

fn parse_graph_and_nodes(file_content: &str) -> (Vec<Node<'_>>, DiGraph<&'_ str, ()>) {
    let nodes = parse_nodes(file_content).map(|x| x.1).unwrap();
    let mut graph = DiGraph::new();
    let graph_nodes =
        fxhash::FxHashMap::from_iter(nodes.iter().map(|n| (n.name, graph.add_node(n.name))));

    for n in nodes.iter() {
        let n_node = graph_nodes.get(n.name).copied().unwrap();
        for d in n.value.dependencies() {
            let d_node = graph_nodes.get(d).copied().unwrap();

            graph.add_edge(n_node, d_node, ());
        }
    }

    (nodes, graph)
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> isize {
    let mut values = fxhash::FxHashMap::default();

    let (nodes, graph) = parse_graph_and_nodes(file_content);

    ValueCalculator::build("root", &nodes, &graph)
        .calculate(&mut values)
        .unwrap()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> isize {
    let (nodes, graph) = parse_graph_and_nodes(file_content);
    let root_node = nodes.iter().find(|x| x.name == "root").cloned().unwrap();

    let (left_node, right_node) = root_node.value.dependencies().collect_tuple().unwrap();
    let left_calculator = ValueCalculator::build(left_node, &nodes, &graph);
    let right_calculator = ValueCalculator::build(right_node, &nodes, &graph);
    let mut values = fxhash::FxHashMap::default();
    for i in 0isize.. {
        // tracing::info!(?i, "setting humn");
        values.clear();
        values.insert("humn", i);
        let Some(left) = left_calculator.calculate(&mut values) else {
            // tracing::info!("left is undefined");
            continue;
        };
        let Some(right) = right_calculator.calculate(&mut values) else {
            // tracing::info!("right is undefined");
            continue;
        };
        if left == right {
            return i;
        }
    }
    0
}

// y(n) = 63484871975792 - 9792 * n
// y(1) = 63484871966000

// (63484871975792 - 30328243757936) / 9792 == n

// n = 3386093568 * 1155 + 52

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "152")]
    #[case::actual(ACTUAL, "93813115694560")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "301")]
    // #[case::actual(ACTUAL, "3910938071092")] // too long to calculate, but the resulting sequence is linear
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
