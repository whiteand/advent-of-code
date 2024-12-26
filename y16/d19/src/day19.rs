#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let n = file_content.trim().parse().unwrap();
    f(n)
}
fn f(n: usize) -> usize {
    if n <= 2 {
        return 1;
    }
    (f(n >> 1) << 1) + ((n & 0b1) << 1) - 1
}

fn g(n: usize) -> usize {
    (3..(n + 1)).fold(1, |res, i| {
        let n = i >> 1;

        if res < n {
            res + 1
        } else if res < i - 1 {
            res + 2
        } else {
            1
        }
    })
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let n = file_content.trim().parse().unwrap();

    g(n)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "3")]
    #[case::actual(ACTUAL, "1834903")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }

    #[rstest]
    #[case::n1("1", "1")]
    #[case::n2("2", "1")]
    #[case::n3("3", "3")]
    #[case::n5("5", "2")]
    #[case::n10("10", "1")]
    #[case::n11("11", "2")]
    #[case::actual(ACTUAL, "1420280")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
