use super::{condition::Condition, item::Item, monkey::Monkey, operation::Expression};
use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::newline},
    combinator,
    multi::separated_list1,
    sequence::{self, delimited, preceded, separated_pair, tuple},
    IResult,
};

fn parse_index(input: &str) -> IResult<&str, u64> {
    let mut parse_line =
        nom::sequence::delimited(tag("Monkey "), character::complete::u64, tag(":\n"));
    parse_line(input)
}

fn parse_items(input: &str) -> IResult<&str, Vec<Item>> {
    let mut parse = nom::sequence::delimited(
        tag("  Starting items: "),
        separated_list1(
            tag(", "),
            nom::combinator::map(character::complete::u64, |x| Item { worry_level: x }),
        ),
        newline,
    );
    parse(input)
}

fn parse_int(input: &str) -> IResult<&str, Expression> {
    let mut parse_int = combinator::map(character::complete::u64, Expression::Integer);
    parse_int(input)
}
fn parse_old(input: &str) -> IResult<&str, Expression> {
    let mut parse_old = combinator::map(tag("old"), |_| Expression::Var("old"));
    parse_old(input)
}
fn parse_sum(input: &str) -> IResult<&str, Expression> {
    combinator::map(
        separated_pair(parse_expression, tag(" + "), parse_expression),
        |(a, b)| Expression::Sum(Box::new((a, b))),
    )(input)
}
fn parse_product(input: &str) -> IResult<&str, Expression> {
    combinator::map(
        separated_pair(parse_expression, tag(" * "), parse_expression),
        |(a, b)| Expression::Product(Box::new((a, b))),
    )(input)
}
fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((parse_int, parse_old, parse_sum, parse_product))(input)
}

fn parse_operation(input: &str) -> IResult<&str, Expression> {
    let mut parse = nom::sequence::preceded(
        tag("  Operation: new = "),
        alt((
            nom::sequence::terminated(parse_int, newline),
            nom::sequence::terminated(parse_old, newline),
            nom::sequence::terminated(parse_sum, newline),
            nom::sequence::terminated(parse_product, newline),
        )),
    );
    parse(input)
}

fn parse_condition(input: &str) -> IResult<&str, Condition> {
    nom::combinator::map(
        tuple((
            delimited(
                tag("  Test: divisible by "),
                character::complete::u64,
                newline,
            ),
            delimited(
                tag("    If true: throw to monkey "),
                character::complete::u32,
                newline,
            ),
            preceded(
                tag("    If false: throw to monkey "),
                character::complete::u32,
            ),
        )),
        |(divisor, if_true, if_false)| {
            Condition::new_division_condition(divisor, if_true as usize, if_false as usize)
        },
    )(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let parse_tuple = tuple((parse_index, parse_items, parse_operation, parse_condition));
    let mut parse = nom::combinator::map(parse_tuple, |(_, items, operation, condition)| {
        Monkey::new(items, operation, condition)
    });

    parse(input)
}
pub(crate) fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(sequence::pair(newline, newline), parse_monkey)(input)
}
