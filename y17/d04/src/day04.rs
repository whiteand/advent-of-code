use fxhash::FxHashSet;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let mut present = FxHashSet::default();
    let mut total = 0;
    'lines: for line in file_content.lines() {
        present.clear();
        for word in line.split_ascii_whitespace().filter(|x| !x.is_empty()) {
            if !present.insert(word) {
                continue 'lines;
            }
        }
        total += 1;
    }
    total
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let mut present = FxHashSet::default();
    let mut total = 0;
    'lines: for line in file_content.lines() {
        present.clear();
        for word in line.split_ascii_whitespace().filter(|x| !x.is_empty()) {
            let sorted = word.as_bytes().iter().copied().sorted().collect_vec();
            if !present.insert(sorted) {
                continue 'lines;
            }
        }
        total += 1;
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::actual(ACTUAL, "455")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(ACTUAL, "186")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
