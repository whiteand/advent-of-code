use advent_utils::parse;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let mut jumps = parse::nums::<i16>(file_content).collect_vec();
    let mut ip: i16 = 0;
    let mut steps = 0;
    while let Some((ip_usize, j)) = usize::try_from(ip)
        .ok()
        .and_then(|ip| jumps.get(ip).copied().map(|x| (ip, x)))
    {
        jumps[ip_usize] += 1;
        ip += j;
        steps += 1;
    }
    steps
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let mut jumps = parse::nums::<i16>(file_content).collect_vec();
    let mut ip: i16 = 0;
    let mut steps = 0;
    while let Some((ip_usize, j)) = usize::try_from(ip)
        .ok()
        .and_then(|ip| jumps.get(ip).copied().map(|x| (ip, x)))
    {
        if j < 3 {
            jumps[ip_usize] += 1;
        } else {
            jumps[ip_usize] -= 1;
        }
        ip += j;
        steps += 1;
    }
    steps
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::actual(ACTUAL, "374269")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(ACTUAL, "27720699")] // 1s
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
