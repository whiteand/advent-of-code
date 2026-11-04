use itertools::Itertools;

use super::numbers::Pairs;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> i32 {
    let mut numbers = Pairs::default();

    let nums = file_content
        .trim()
        .lines()
        .map(|line| numbers.parse_number(line).unwrap())
        .collect_vec();

    let sum = nums.into_iter().reduce(|a, b| numbers.add(a, b)).unwrap();

    numbers.get(sum).magnitude()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> i32 {
    let mut numbers = Pairs::default();

    let nums = file_content
        .trim()
        .lines()
        .map(|line| numbers.parse_number(line).unwrap())
        .collect_vec();

    let possible_sums = nums
        .into_iter()
        .permutations(2)
        .map(|elements| numbers.add(elements[0], elements[1]))
        .collect_vec();

    let max = possible_sums
        .into_iter()
        .map(|i| numbers.get(i).magnitude())
        .max()
        .unwrap_or_default();

    max
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "4140")]
    #[case::actual(ACTUAL, "3793")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "3993")]
    #[case::actual(ACTUAL, "4695")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
