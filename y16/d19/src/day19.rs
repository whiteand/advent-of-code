use tracing::info;

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

#[derive(Debug)]
enum Task {
    Calc,
    HandleOdd(usize),
    HandleEven(usize),
}

fn g(x: usize) -> usize {
    if x <= 2 {
        return 1;
    }
    let mut res = x;
    let mut tasks = Vec::with_capacity(3014601);
    tasks.push(Task::Calc);
    while let Some(task) = tasks.pop() {
        info!(?res, ?task);
        match task {
            Task::Calc => {
                if res <= 2 {
                    res = 1;
                    continue;
                }
                if res & 0b1 == 1 {
                    let n = res >> 1;
                    tasks.push(Task::HandleOdd(n));
                    res -= 1;
                    tasks.push(Task::Calc);
                } else {
                    tasks.push(Task::HandleEven(res >> 1));
                    res -= 1;
                    tasks.push(Task::Calc);
                }
            }
            Task::HandleOdd(n) => {
                res = if res < n {
                    res + 1
                } else if res < (n * 2) {
                    res + 2
                } else {
                    1
                };
            }
            Task::HandleEven(n) => {
                res = if res < n {
                    res + 1
                } else if res < ((n << 1) - 1) {
                    res + 2
                } else {
                    1
                };
            }
        }
    }
    res
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
    #[case::n5("5", "2")]
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
