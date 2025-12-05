use std::ops::RangeInclusive;

use advent_utils::nom::{
    self, Parser, character::complete::line_ending, combinator::all_consuming,
};
use itertools::Itertools;

pub fn parse_ranges(input: &str) -> nom::IResult<&str, Vec<RangeInclusive<usize>>> {
    nom::multi::separated_list1(
        nom::character::complete::line_ending,
        nom::sequence::separated_pair(
            nom::parse_usize,
            nom::bytes::complete::tag("-"),
            nom::parse_usize,
        )
        .map(|(a, b)| a..=b),
    )
    .parse(input)
}

fn parse_ids(input: &str) -> nom::IResult<&str, Vec<usize>> {
    nom::multi::separated_list1(nom::character::complete::line_ending, nom::parse_usize)
        .parse(input)
}

fn parse_input(input: &str) -> nom::IResult<&str, (Vec<RangeInclusive<usize>>, Vec<usize>)> {
    all_consuming(nom::sequence::separated_pair(
        parse_ranges,
        (line_ending, line_ending),
        parse_ids,
    ))
    .parse(input)
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let (_, (ranges, ids)) = parse_input.parse(file_content).unwrap();
    ids.iter()
        .filter(|id| ranges.iter().any(|r| r.contains(id)))
        .count()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let (_, (ranges, _)) = parse_input.parse(file_content).unwrap();
    let merged = merge_ranges(ranges);

    merged.into_iter().map(|x| x.end() - x.start() + 1).sum()
}

fn merge_ranges(mut ranges: Vec<RangeInclusive<usize>>) -> Vec<RangeInclusive<usize>> {
    let mut merged: Vec<RangeInclusive<usize>> = Vec::with_capacity(ranges.len());
    ranges.sort_by(|a, b| a.start().cmp(b.start()).then_with(|| a.end().cmp(b.end())));
    while !ranges.is_empty() {
        let min = *ranges[0].start();
        let max = ranges.iter().fold(min, |max, r| {
            if r.start().le(&(max + 1)) {
                max.max(*r.end())
            } else {
                max
            }
        });
        let new_range = min..=max;
        ranges.retain(|r| new_range.end() < r.start());
        merged.push(new_range);
    }
    merged
}

#[cfg(test)]
mod tests {
    use crate::day05::{merge_ranges, parse_ranges};

    use super::{part1, part2};
    use itertools::Itertools;
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const NEW_EXAMPLE: &str = include_str!("../new_example.txt");
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
    #[case::new_example(NEW_EXAMPLE, "14")]
    #[case::actual(ACTUAL, "366181852921027")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }

    #[rstest]
    #[case("3-5\n6-10", "3-10")]
    #[case("3-5\n7-10", "3-5\n7-10")]
    #[case("3-5\n5-8", "3-8")]
    #[case("3-5\n5-8", "3-8")]
    #[case("3-5\n4-8", "3-8")]
    #[case("3-5\n4-5", "3-5")]
    #[case("3-5\n2-8", "2-8")]
    #[case("3-5\n3-5", "3-5")]
    #[case("3-5\n2-5", "2-5")]
    #[case("3-5\n2-4", "2-5")]
    #[case("3-5\n2-3", "2-5")]
    #[case("3-5\n1-2", "1-5")]
    fn test_merge_ranges(#[case] inp: &str, #[case] expected: &str) {
        let (_, ranges) = parse_ranges(inp).unwrap();
        let (_, expected_merged) = parse_ranges(expected).unwrap();
        let n = ranges.len();
        for p in ranges.into_iter().permutations(n) {
            let actual_merged = merge_ranges(p);
            assert_eq!(
                actual_merged, expected_merged,
                "merge_ranges(\n{inp}\n) returned invalid response"
            )
        }
    }
}
