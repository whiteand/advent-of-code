use advent_utils::nom::{
    self, Parser,
    branch::alt,
    character::complete,
    combinator,
    multi::{many0, separated_list0},
    sequence::delimited,
};

use crate::model::{Button, Indicators, JoltageVec, Machine};

fn parse_indicators(input: &str) -> nom::IResult<&str, Indicators> {
    delimited(
        complete::char('['),
        many0(alt((
            combinator::value(0u32, complete::char('.')),
            combinator::value(1u32, complete::char('#')),
        )))
        .map(|bits| {
            Indicators::new(
                bits.iter()
                    .copied()
                    .enumerate()
                    .fold(0u32, |acc, (i, b)| acc | (b << i)),
                bits.len(),
            )
        }),
        complete::char(']'),
    )
    .parse(input)
}

fn parse_button(input: &str) -> nom::IResult<&str, Button> {
    delimited(
        complete::char('('),
        separated_list0(complete::char(','), complete::u32)
            .map(|bits| bits.into_iter().fold(0u32, |a, i| a | (1 << i)).into()),
        complete::char(')'),
    )
    .parse(input)
}

fn parse_joltage(input: &str) -> nom::IResult<&str, JoltageVec> {
    delimited(
        complete::char('{'),
        separated_list0(complete::char(','), complete::u16),
        complete::char('}'),
    )
    .parse(input)
}

fn parse_machine(input: &str) -> nom::IResult<&str, Machine> {
    (
        parse_indicators,
        complete::char(' '),
        separated_list0(complete::char(' '), parse_button),
        complete::char(' '),
        parse_joltage,
    )
        .map(|(target_indicators, _, buttons, _, joltage)| {
            Machine::new(target_indicators, buttons, joltage)
        })
        .parse(input)
}

pub fn parse_machines(input: &str) -> impl Iterator<Item = Machine> + '_ {
    input
        .trim()
        .lines()
        .map(|line| parse_machine(line).map(|x| x.1).unwrap())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use crate::model::Button;

    #[rstest]
    #[case("(3)", Button::from(8))]
    #[case("(1,3)", Button::from(10))]
    #[case("(2)", Button::from(4))]
    #[case("(2,3)", Button::from(12))]
    fn test_parse_button(#[case] input: &str, #[case] expected: Button) {
        use crate::parse::parse_button;

        let (_, m) = parse_button(input).unwrap();
        assert_eq!(m, expected);
    }
}
