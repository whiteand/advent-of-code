#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let nums = advent_utils::parse::nums::<isize>(file_content);
    for n in nums {
        println!("{}", n)
    }
    0
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    // cargo test --package y22d20 --lib -- tests::test_part1::case_1_example --exact --show-output
    // cargo test --package y22d20 --lib -- day20::tests::test_part1::case_1_example --exact --nocapture
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
