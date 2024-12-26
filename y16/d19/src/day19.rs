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
fn g(x: usize) -> usize {
    if x <= 2 {
        return 1;
    }
    if x & 0b1 == 1 {
        let n = x >> 1;
        let n2 = n << 1;
        let pr = g(n2);
        if pr < n {
            pr + 1
        } else if pr < n2 {
            pr + 2
        } else {
            1
        }
    } else {
        let prev = g(x - 1);
        let n = x >> 1;
        if prev < n {
            prev + 1
        } else if prev < ((n << 1) - 1) {
            prev + 2
        } else {
            1
        }
    }
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
    #[case::actual(ACTUAL, "0")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "2")]
    #[case::actual(ACTUAL, "0")]
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
