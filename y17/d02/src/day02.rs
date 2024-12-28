use advent_utils::parse;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(|line| {
            let (min, max) = parse::nums::<usize>(line)
                .fold((usize::MAX, usize::MIN), |(min, max), x| {
                    (min.min(x), max.max(x))
                });
            max - min
        })
        .sum()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content
        .lines()
        .flat_map(|line| {
            parse::nums::<usize>(line)
                .collect_vec()
                .into_iter()
                .tuple_combinations()
                .map(|(a, b)| if a >= b { (a, b) } else { (b, a) })
                .find_map(|(a, b)| if a % b == 0 { Some(a / b) } else { None })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "18")]
    #[case::actual(ACTUAL, "45351")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example("5 9 2 8\n9 4 7 3\n3 8 6 5", "9")]
    #[case::actual(ACTUAL, "275")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
