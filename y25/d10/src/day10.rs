use itertools::Itertools;

use crate::{model::Machine, parse::parse_machines};

#[tracing::instrument(skip(input))]
pub fn part1(input: &str) -> usize {
    let machines = parse_machines(input).collect_vec();

    let mut dp = Machine::prepare_fewest_button_clicks_to_target();

    machines
        .into_iter()
        .map(|m| m.get_fewest_button_clicks_to_target(&mut dp))
        .sum()
}
#[tracing::instrument(skip(input))]
pub fn part2(input: &str) -> usize {
    let machines = parse_machines(input).collect_vec();

    machines
        .into_iter()
        .map(|m| m.get_fewest_button_clicks_to_joltage())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "7")]
    #[case::actual(ACTUAL, "434")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }

    #[rstest]
    #[case::example(EXAMPLE, "33")]
    #[case::actual(ACTUAL, "15132")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
