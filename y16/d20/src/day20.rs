use advent_utils::parse;
use itertools::Itertools;
use std::ops::RangeInclusive;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> u32 {
    let ranges = parse_ranges(file_content);
    let mut it = 0u32..;
    loop {
        let Some(x) = it.next() else {
            break 0;
        };
        let Some(block_range) = ranges.iter().find(|r| r.contains(&x)) else {
            return x;
        };
        it = (*block_range.end() + 1)..;
    }
}
fn merge_same(ranges: &mut Vec<RangeInclusive<u32>>) {
    let mut src = 1;
    let mut dst = 0;
    while src < ranges.len() {
        let next = &ranges[src];
        let next_start = *next.start();
        let next_end = *next.end();
        let prev = &ranges[dst];
        let prev_start = *prev.start();
        let prev_end = *prev.end();

        // 0..1 5..6
        if next_start > prev_end {
            dst += 1;
            src += 1;
            ranges[dst] = next_start..=next_end;
            continue;
        }
        // 0..=3 2..=6 => 0..=6
        // 0..=3 3..=6 => 0..=6
        if next_start <= prev_end {
            if next_end <= prev_end {
                src += 1;
                continue;
            }
            ranges[dst] = prev_start..=next_end;
            src += 1;
            continue;
        }
        panic!("Unreachable: {prev_start}..={prev_end} & {next_start}..={next_end}")
    }
    ranges.truncate(dst + 1);
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> u32 {
    let ranges = parse_ranges(file_content);
    u32::MAX
        - ranges
            .into_iter()
            .map(|x| *x.end() - *x.start() + 1)
            .sum::<u32>()
        + 1
}

fn parse_ranges(file_content: &str) -> Vec<RangeInclusive<u32>> {
    let mut ranges = parse::nums::<u32>(file_content)
        .chunks(2)
        .into_iter()
        .map(|chunk| {
            let (x, y) = chunk.into_iter().collect_tuple().unwrap();
            x..=y
        })
        .collect_vec();
    ranges.sort_unstable_by(|a, b| a.start().cmp(b.start()).then_with(|| b.end().cmp(a.end())));
    merge_same(&mut ranges);
    ranges
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "3")]
    #[case::actual(ACTUAL, "32259706")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example("1-1", "4294967295")]
    #[case::actual(ACTUAL, "113")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
