use std::ops::RangeInclusive;

use advent_utils::{
    algo::merge_inclusive_ranges,
    nom::{
        self, Parser, bytes::complete::tag, character::complete::line_ending,
        combinator::all_consuming, multi::separated_list1, parse_usize, sequence::separated_pair,
    },
};

pub fn parse_ranges(input: &str) -> nom::IResult<&str, Vec<RangeInclusive<usize>>> {
    separated_list1(
        line_ending,
        separated_pair(parse_usize, tag("-"), parse_usize).map(|(a, b)| a..=b),
    )
    .parse(input)
}

fn parse_ids(input: &str) -> nom::IResult<&str, Vec<usize>> {
    separated_list1(line_ending, parse_usize).parse(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, (Vec<RangeInclusive<usize>>, Vec<usize>)> {
    all_consuming(separated_pair(
        parse_ranges,
        (line_ending, line_ending),
        parse_ids,
    ))
    .parse(input)
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (_, (ranges, ids)) = parse_input(file_content).unwrap();

    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let (_, mut ranges) = parse_ranges(file_content).unwrap();
    merge_inclusive_ranges(&mut ranges)
        .map(|x| x.end() - x.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "3")]
    #[case::actual(ACTUAL, "798")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "14")]
    #[case::actual(ACTUAL, "366181852921027")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
