use std::fmt::Write;

use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, line_ending},
    combinator::{all_consuming, value},
    multi::separated_list1,
    parse_usize,
    sequence::{preceded, separated_pair, tuple},
    Parser,
};
use fxhash::FxHashMap;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (_, (initials, rules)) = parse_input(file_content.trim()).expect("valid");
    tracing::info!(?initials, ?rules);
    let mut values = FxHashMap::default();
    for (x, initial) in initials {
        values.insert(x, Value::Initial(initial));
    }
    let mut zs = vec![];

    for rule in rules {
        if rule.dst.starts_with("z") {
            zs.push(rule.dst);
        }
        values.insert(rule.dst, Value::Rule(rule));
    }
    let mut z = 0;
    for z_var_name in zs.iter().copied() {
        let bit = calculate(&mut values, z_var_name);
        let offset = z_var_name[1..].parse::<usize>().unwrap();
        z |= bit << offset;
    }
    z
}

#[tracing::instrument(ret, skip(values))]
fn calculate<'t, 'u>(values: &mut FxHashMap<&'t str, Value<'t>>, var_name: &'u str) -> usize
where
    'u: 't,
{
    let res = values.get(var_name).expect("defined");
    let (a, b, op) = match res {
        Value::Initial(x) => return *x,
        Value::Calculated(x) => return *x,
        Value::Rule(rule) => (rule.a, rule.b, rule.op),
    };
    let a = calculate(values, a);
    let b = calculate(values, b);
    let res = match op {
        Operation::And => a & b,
        Operation::Or => a | b,
        Operation::Xor => a ^ b,
    };
    values.insert(var_name, Value::Calculated(res));
    res
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
}

enum Value<'t> {
    Initial(usize),
    Calculated(usize),
    Rule(Rule<'t>),
}

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum Operation {
    And,
    Or,
    Xor,
}

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
        write!(f, "let {} = {} {} {};", self.dst, self.a, self.op, self.b)
    }
}
impl std::fmt::Debug for Rule<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

fn parse_input<'t>(input: &str) -> nom::IResult<&str, (Vec<(&str, usize)>, Vec<Rule<'_>>)> {
    all_consuming(separated_pair(
        parse_initials,
        tuple((line_ending, line_ending)),
        parse_rules,
    ))(input)
}
fn parse_initials(input: &str) -> nom::IResult<&str, Vec<(&str, usize)>> {
    separated_list1(line_ending, parse_initial)(input)
}
fn parse_initial(input: &str) -> nom::IResult<&str, (&str, usize)> {
    separated_pair(alphanumeric1, tag(": "), parse_usize)(input)
}
fn parse_rules(input: &str) -> nom::IResult<&str, Vec<Rule<'_>>> {
    separated_list1(line_ending, parse_rule)(input)
}
fn parse_rule(input: &str) -> nom::IResult<&str, Rule> {
    tuple((
        alphanumeric1,
        alt((
            value(Operation::Or, tag(" OR ")),
            value(Operation::And, tag(" AND ")),
            value(Operation::Xor, tag(" XOR ")),
        )),
        alphanumeric1,
        preceded(tag(" -> "), alphanumeric1),
    ))
    .map(|(a, op, b, dst)| Rule { op, a, b, dst })
    .parse(input)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(part1(ACTUAL), 64755511006320);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(ACTUAL)), "0");
    }
}
