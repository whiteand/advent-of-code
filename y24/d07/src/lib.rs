use std::marker::PhantomData;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> u128 {
    solve::<Or<Mul, Add>>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> u128 {
    solve::<Or<Concat, Or<Mul, Add>>>(file_content)
}

fn solve<Ops: Operation>(file_content: &str) -> u128 {
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

// #[tracing::instrument(ret)]
fn can_be_constructed<Ops: Operation>(result: u128, operands: &[u128]) -> bool {
    match operands {
        [] => false,
        [last] => result == *last,
        [rest @ .., last] => Ops::reverse(result, *last)
            .any(|prev_result| can_be_constructed::<Ops>(prev_result, rest)),
    }
}

macro_rules! impl_op {
    ($id:ident, $res:ident, $op:ident, $expr:expr) => {
        struct $id;
        impl Operation for $id {
            fn reverse($res: u128, $op: u128) -> impl Iterator<Item = u128> {
                std::iter::from_fn(move || $expr).take(1)
            }
        }
    };
}
impl_op!(Mul, result, op, (result % op == 0).then(|| result / op));
impl_op!(Add, result, op, (result >= op).then(|| result - op));
impl_op!(Concat, result, op, trim_suffix(result, op));

struct Or<A, B>(PhantomData<(A, B)>);
impl<A: Operation, B: Operation> Operation for Or<A, B> {
    fn reverse(result: u128, op: u128) -> impl Iterator<Item = u128> {
        A::reverse(result, op).chain(B::reverse(result, op))
    }
}

trait Operation {
    fn reverse(result: u128, op: u128) -> impl Iterator<Item = u128>;
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
