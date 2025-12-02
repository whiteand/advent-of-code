use std::{fmt::Write, fs};

use advent_utils::nom::{
    self, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    combinator::{all_consuming, value},
    multi::separated_list1,
    parse_usize,
    sequence::{preceded, separated_pair},
};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use petgraph::dot::Dot;

struct Schema<'t> {
    rules: FxHashMap<&'t str, Rule<'t>>,
}

type Values<'t> = FxHashMap<&'t str, usize>;

impl<'t> Schema<'t> {
    fn calculate_single(
        &self,
        initials: &Values<'t>,
        visited: &mut FxHashSet<&'t str>,
        calculated: &mut Values<'t>,
        var: &'t str,
    ) -> Result<usize, VerificationError> {
        if let Some(x) = initials.get(var) {
            return Ok(*x);
        }
        if let Some(x) = calculated.get(var) {
            return Ok(*x);
        }
        if !visited.insert(var) {
            return Err(VerificationError::Loop);
        }
        let Some(rule) = self.rules.get(var) else {
            panic!("{var} is not defined");
        };
        let a = self.calculate_single(initials, visited, calculated, rule.a)?;
        let b = self.calculate_single(initials, visited, calculated, rule.b)?;
        let res = match rule.op {
            Operation::And => a & b,
            Operation::Or => a | b,
            Operation::Xor => a ^ b,
        };
        calculated.insert(var, res);
        Ok(res)
    }
    fn calculate_many<'s, 'i>(
        &'s self,
        initials: &'i FxHashMap<&'t str, usize>,
        vars: impl Iterator<Item = &'t str> + 's,
    ) -> impl Iterator<Item = Result<(&'t str, usize), VerificationError>> + 's
    where
        'i: 's,
    {
        let mut calculated = FxHashMap::default();
        vars.into_iter().map(move |var| {
            self.calculate_single(initials, &mut FxHashSet::default(), &mut calculated, var)
                .map(|y| (var, y))
        })
    }
    fn calculate_usize(
        &self,
        initials: &Values,
        bits: &[&'t str],
    ) -> Result<usize, VerificationError> {
        let mut z = 0;
        self.calculate_many(initials, bits.iter().copied())
            .try_for_each(|r| {
                let (bit_name, bit) = r?;
                let offset = bit_name[1..].parse::<usize>().unwrap();
                z |= bit << offset;
                Ok(())
            })?;

        Ok(z)
    }

    fn swap_rules(&mut self, x: &'t str, y: &'t str) -> bool {
        if x == y {
            return false;
        }
        let mut rule_x = self.rules.remove(x).unwrap_or_else(|| {
            panic!("Failed to find {x} rule");
        });
        let mut rule_y = self.rules.remove(y).unwrap_or_else(|| {
            panic!("Failed to find {y} rule");
        });
        rule_x.dst = y;
        rule_y.dst = x;
        self.rules.insert(x, rule_y);
        self.rules.insert(y, rule_x);
        true
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let Input {
        zs,
        initials,
        schema,
        ..
    } = parse_input(file_content);

    schema.calculate_usize(&initials, &zs).unwrap()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> String {
    let Input {
        xs,
        ys,
        zs,
        initials,
        mut schema,
    } = parse_input(file_content);
    let x = schema.calculate_usize(&initials, &xs).expect("valid");
    let y = schema.calculate_usize(&initials, &ys).expect("valid");
    let expected = x + y;
    // I found this values by hands looking at graphviz visualization of dependencies and finding inconsistencies
    // in schemas of broken bits.
    let swaps = [
        Swap::new("djg", "z12"),
        Swap::new("sbg", "z19"),
        Swap::new("mcq", "hjm"),
        Swap::new("z37", "dsd"),
    ];
    for x in &swaps {
        assert!(schema.swap_rules(x.a, x.b));
    }
    validate_z(&schema, &zs, &initials, expected).expect("valid");

    let mut graph = petgraph::prelude::DiGraph::<&str, Operation>::new();
    let xs_nodes = xs.iter().copied().map(|x| graph.add_node(x)).collect_vec();
    let ys_nodes = ys.iter().copied().map(|x| graph.add_node(x)).collect_vec();
    let keys = schema.rules.keys().copied().collect_vec();
    // schema.get_all_dependents(xs[0..13].iter().copied().chain(ys[0..13].iter().copied()));

    let nodes = keys
        .iter()
        .copied()
        .map(|x| graph.add_node(x))
        .collect_vec();
    for i in 0..keys.len() {
        let key = keys[i];
        let node = nodes[i];
        let rule = schema.rules.get(key).unwrap();
        match &[&keys, &xs, &ys].map(|x| {
            x.iter()
                .copied()
                .find_position(|y| rule.a == *y)
                .map(|x| x.0)
        }) {
            [None, None, None] => {
                // panic!("failed to find the source of {}", keys[i]);
            }
            [Some(i), None, None] => {
                let a_node = nodes[*i];
                graph.add_edge(a_node, node, rule.op);
            }
            [None, None, Some(i)] => {
                let a_node = ys_nodes[*i];
                graph.add_edge(a_node, node, rule.op);
            }
            [None, Some(i), None] => {
                let a_node = xs_nodes[*i];
                graph.add_edge(a_node, node, rule.op);
            }
            x => {
                panic!("{x:?}");
            }
        };
        match &[&keys, &xs, &ys].map(|x| {
            x.iter()
                .copied()
                .find_position(|y| rule.b == *y)
                .map(|x| x.0)
        }) {
            [None, None, None] => {
                // panic!("failed to find the source of {}", keys[i]);
            }
            [Some(i), None, None] => {
                let b_node = nodes[*i];
                graph.add_edge(b_node, node, rule.op);
            }
            [None, None, Some(i)] => {
                let b_node = ys_nodes[*i];
                graph.add_edge(b_node, node, rule.op);
            }
            [None, Some(i), None] => {
                let b_node = xs_nodes[*i];
                graph.add_edge(b_node, node, rule.op);
            }
            x => {
                panic!("{x:?}");
            }
        };
    }

    fs::write("./adder.dot", Dot::with_config(&graph, &[]).to_string()).unwrap();

    swaps
        .into_iter()
        .flat_map(|x| x.into_iter())
        .sorted_unstable()
        .join(",")
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct Swap<'t> {
    a: &'t str,
    b: &'t str,
}
impl<'t> Swap<'t> {
    pub fn new(a: &'t str, b: &'t str) -> Swap<'t> {
        if a.gt(b) {
            Self { b: a, a: b }
        } else {
            Self { a, b }
        }
    }
}
impl<'t> IntoIterator for Swap<'t> {
    type Item = &'t str;

    type IntoIter = std::array::IntoIter<&'t str, 2>;

    fn into_iter(self) -> Self::IntoIter {
        [self.a, self.b].into_iter()
    }
}

#[derive(Debug)]
enum VerificationError {
    Loop,
    InvalidBits,
}

fn validate_z<'t>(
    schema: &Schema<'t>,
    zs: &[&'t str],
    initials: &FxHashMap<&'t str, usize>,
    expected: usize,
) -> Result<(), VerificationError> {
    let z = schema.calculate_usize(initials, zs)?;
    let mut invalid = Vec::with_capacity(46);
    for (i, _) in zs.iter().copied().enumerate() {
        let actual_bit = (z >> i) & 0b1;
        let expected_bit = (expected >> i) & 0b1;
        if actual_bit == expected_bit {
            continue;
        }
        invalid.push(i);
    }

    if invalid.is_empty() {
        Ok(())
    } else {
        Err(VerificationError::InvalidBits)
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

#[derive(Clone, PartialEq, Eq)]
struct Rule<'t> {
    op: Operation,
    a: &'t str,
    b: &'t str,
    dst: &'t str,
}

impl std::fmt::Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let op = match self {
            Operation::And => '&',
            Operation::Or => '|',
            Operation::Xor => '^',
        };
        f.write_char(op)
    }
}
impl std::fmt::Debug for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}
impl std::fmt::Display for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({} {} {} -> {});", self.a, self.op, self.b, self.dst)
    }
}
impl std::fmt::Debug for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

struct Input<'t> {
    xs: Vec<&'t str>,
    ys: Vec<&'t str>,
    zs: Vec<&'t str>,
    initials: FxHashMap<&'t str, usize>,
    schema: Schema<'t>,
}

fn parse_input(input: &str) -> Input<'_> {
    let (_, (initials, rules)) = parse_initials_and_rules(input.trim()).expect("valid");
    let mut xs = vec![];
    let mut ys = vec![];
    let mut zs = vec![];
    let mut initials_map = FxHashMap::default();
    let mut rules_map = FxHashMap::default();
    for (name, initial) in initials {
        if name.starts_with("x") {
            xs.push(name);
        }
        if name.starts_with("y") {
            ys.push(name);
        }
        if name.starts_with("z") {
            zs.push(name);
        }
        initials_map.insert(name, initial);
    }

    for rule in rules {
        if rule.dst.starts_with("z") {
            zs.push(rule.dst);
        }
        if rule.dst.starts_with("x") {
            xs.push(rule.dst);
        }
        if rule.dst.starts_with("y") {
            ys.push(rule.dst);
        }
        rules_map.insert(rule.dst, rule);
    }
    xs.sort_unstable();
    ys.sort_unstable();
    zs.sort_unstable();

    Input {
        xs,
        ys,
        zs,
        initials: initials_map,
        schema: Schema { rules: rules_map },
    }
}

type InitialsAndRules<'t> = (Vec<(&'t str, usize)>, Vec<Rule<'t>>);

fn parse_initials_and_rules(input: &str) -> nom::IResult<&str, InitialsAndRules<'_>> {
    all_consuming(separated_pair(
        parse_initials,
        (line_ending, line_ending),
        parse_rules,
    ))
    .parse(input)
}
fn parse_initials(input: &str) -> nom::IResult<&str, Vec<(&str, usize)>> {
    separated_list1(line_ending, parse_initial).parse(input)
}
fn parse_initial(input: &str) -> nom::IResult<&str, (&str, usize)> {
    separated_pair(alphanumeric1, tag(": "), parse_usize).parse(input)
}
fn parse_rules(input: &str) -> nom::IResult<&str, Vec<Rule<'_>>> {
    separated_list1(line_ending, parse_rule).parse(input)
}
fn parse_rule(input: &str) -> nom::IResult<&str, Rule<'_>> {
    (
        alphanumeric1,
        alt((
            value(Operation::Or, tag(" OR ")),
            value(Operation::And, tag(" AND ")),
            value(Operation::Xor, tag(" XOR ")),
        )),
        alphanumeric1,
        preceded(tag(" -> "), alphanumeric1),
    )
        .map(|(a, op, b, dst)| Rule { op, a, b, dst })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(part1(ACTUAL), 64755511006320);
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(part2(ACTUAL).as_str(), "djg,dsd,hjm,mcq,sbg,z12,z19,z37");
    }
}
