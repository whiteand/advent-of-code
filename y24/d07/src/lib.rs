#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> u128 {
    solve::<AddMul>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> u128 {
    solve::<AddMulConcat>(file_content)
}

struct AddMul;
impl Operations for AddMul {
    fn reverse(result: u128, op: u128) -> impl Iterator<Item = u128> {
        let mut state = 0;
        std::iter::from_fn(move || match state {
            0 => {
                state += 1;
                (result % op == 0).then(|| result / op)
            }
            1 => {
                state += 1;
                (result >= op).then(|| result - op)
            }
            _ => None,
        })
    }
}
struct AddMulConcat;

impl Operations for AddMulConcat {
    fn reverse(result: u128, op: u128) -> impl Iterator<Item = u128> {
        std::iter::from_fn(move || trim_suffix(result, op))
            .take(1)
            .chain(AddMul::reverse(result, op))
    }
}

trait Operations {
    fn reverse(result: u128, op: u128) -> impl Iterator<Item = u128>;
}

fn solve<Ops: Operations>(file_content: &str) -> u128 {
    let mut buf = Vec::with_capacity(20);
    file_content
        .lines()
        .filter_map(|line| {
            let (test_s, eq_s) = line.split_once(": ")?;
            let test = test_s.parse::<u128>().ok()?;
            buf.clear();
            buf.extend(
                eq_s.split_ascii_whitespace()
                    .map(|x| x.parse::<u128>().unwrap()),
            );
            can_be_constructed::<Ops>(test, &buf).then_some(test)
        })
        .sum()
}

fn can_be_constructed<Ops: Operations>(result: u128, operands: &[u128]) -> bool {
    let Some((last, rest)) = operands.split_last() else {
        return false;
    };
    let last = *last;
    if rest.is_empty() {
        result == last
    } else {
        Ops::reverse(result, last).any(|prev_result| can_be_constructed::<Ops>(prev_result, rest))
    }
}

fn trim_suffix(mut long: u128, mut short: u128) -> Option<u128> {
    if short == 0 {
        return (long % 10 == 0).then_some(long / 10);
    }
    while long % 10 == short % 10 && short > 0 {
        long /= 10;
        short /= 10;
    }
    (short == 0).then_some(long)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "3749");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1620690235709");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "11387");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "145397611075341");
    }
}
