use std::ops::RangeInclusive;

use advent_utils::nom::{
    self, Parser, character::complete::line_ending, combinator::all_consuming,
};

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
    // sorting all ranges in order
    // - smaller start last
    // - smaller end last
    // Example of range order
    // ranges[0]:                       ##### (23-27)
    // ranges[1]:                       ### (23-25)
    // ranges[2]:                  ### (18-20)
    // ranges[3]:                ### (16-18)
    // ranges[4]:           ### (11-13)
    // ranges[5]:     ##### (5-9)
    // ranges[6]:   ### (3-5)
    // ranges[7]: ### (1-3)
    ranges.sort_by(|a, b| b.start().cmp(a.start()).then_with(|| b.end().cmp(a.end())));
    while let Some(first_range) = ranges.pop() {
        // Finding the largest max which merges all intersecting ranges
        // ranges
        // If ranges is:
        // ranges[0]                       ##### <-\
        // ranges[1]                       ###.    |
        // ranges[2]                  ###.         +---- these ranges will remain
        // ranges[3]                ###            |
        // ranges[4]           ### <---------------/
        // ranges[5]     ##### <------------------------ last merged range
        // ranges[6]   ###
        // ranges[7] ### <------------------------------ first_range

        let min = *first_range.start();
        let mut max = *first_range.end();
        let mut last_merged_index = None;

        for (i, r) in ranges.iter().enumerate().rev() {
            if *r.start() > max + 1 {
                break;
            }
            max = max.max(*r.end());
            last_merged_index = Some(i);
        }
        merged.push(min..=max);
        ranges.truncate(last_merged_index.unwrap_or(ranges.len()));
    }
    merged
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
