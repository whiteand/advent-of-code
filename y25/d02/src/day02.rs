use itertools::Itertools;
use std::ops::Range;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    solve(file_content, 2..3)
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    solve(file_content, 2..usize::MAX)
}

fn solve(file_content: &str, repeats: Range<usize>) -> usize {
    parse_ranges(file_content)
        .flat_map(move |range| {
            let start = range.start;
            let end = range.end;
            repeats
                .clone()
                .take_while(move |repeats| generate_invalids(*repeats).next().unwrap() <= end)
                .flat_map(move |repeats| {
                    generate_invalids(repeats)
                        .take_while(move |x| *x < end)
                        .filter(move |x| (start..end).contains(x))
                })
        })
        .unique()
        .sum()
}

fn parse_ranges(file_content: &str) -> impl Iterator<Item = Range<usize>> {
    file_content.trim().split(',').map(|pair| {
        let (a, b) = pair.split_once("-").unwrap();

        tracing::info!(?a, ?b, "splitted pair");
        let a = a.parse::<usize>().unwrap();
        let b = b.parse::<usize>().unwrap();
        a..(b + 1)
    })
}

fn generate_invalids(repeats: usize) -> impl Iterator<Item = usize> {
    (1..).flat_map(move |repeating_size| generate_invalids_of_size(repeating_size, repeats))
}
fn generate_invalids_of_size(size: u32, repeats: usize) -> impl Iterator<Item = usize> {
    let min_num = 10usize.pow(size - 1);
    let max_num = 10usize.pow(size);
    let multiplier = max_num;
    (min_num..max_num).map(move |x| {
        let mut res = x;
        for _ in 1..repeats {
            res *= multiplier;
            res += x;
        }
        res
    })
}

#[cfg(test)]
mod tests {
    use crate::day02::generate_invalids;

    use super::{part1, part2};
    use itertools::Itertools;
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case("11-22", "33")]
    #[case("95-115", "210")]
    #[case("998-1012", "2009")]
    #[case("1188511880-1188511890", "1188511885")]
    #[case::example(EXAMPLE, "4174379265")]
    #[case::actual(ACTUAL, "36037497037")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }

    #[test]
    fn test_generate_invalid_ids() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        let invalid_ids = generate_invalids(2).take(15).collect_vec();
        assert_eq!(
            invalid_ids,
            vec![
                11, 22, 33, 44, 55, 66, 77, 88, 99, 1010, 1111, 1212, 1313, 1414, 1515
            ]
        );
        let invalid_ids = generate_invalids(3).take(15).collect_vec();
        assert_eq!(
            invalid_ids,
            vec![
                111, 222, 333, 444, 555, 666, 777, 888, 999, 101010, 111111, 121212, 131313,
                141414, 151515
            ]
        );
    }

    #[rstest]
    #[case::example(EXAMPLE, "1227775554")]
    #[case::actual(ACTUAL, "17077011375")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
}
