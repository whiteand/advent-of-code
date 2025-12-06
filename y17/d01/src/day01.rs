use advent_utils::parse::OnlyDecDigits;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let digits = file_content
        .as_bytes()
        .iter()
        .copied()
        .only_dec_digits()
        .collect_vec();

    digits
        .into_iter()
        .circular_tuple_windows()
        .filter_map(|(a, b)| (a == b).then_some(a as usize))
        .sum()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let digits = file_content
        .as_bytes()
        .iter()
        .copied()
        .only_dec_digits()
        .collect_vec();

    digits
        .iter()
        .copied()
        .enumerate()
        .filter_map(|(i, x)| {
            (x == digits[(i + digits.len() / 2) % digits.len()]).then_some(x as usize)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example("1122", "3")]
    #[case::example("1111", "4")]
    #[case::example("1234", "0")]
    #[case::example("91212129", "9")]
    #[case::actual(ACTUAL, "1393")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(ACTUAL, "1292")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
