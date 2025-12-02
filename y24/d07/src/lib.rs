use itertools::Itertools;

type Int = u64;

#[inline(always)]
pub fn solve_part_1(file_content: &str) -> Int {
    solve::<Or<Mul, Add>>(file_content)
}

#[inline(always)]
pub fn solve_part_2(file_content: &str) -> Int {
    solve::<Or<Concat, Or<Mul, Add>>>(file_content)
}

#[inline(always)]
fn solve<O: Op>(file_content: &str) -> Int {
    let mut buf = Vec::with_capacity(16);
    file_content
        .lines()
        .filter_map(|line| {
            let (test_s, eq_s) = line.split_once(": ")?;
            let test = test_s.parse::<Int>().ok()?;
            buf.clear();
            buf.extend(
                eq_s.split_ascii_whitespace()
                    .map(|x| x.parse::<Int>().unwrap()),
            );

            can_be_constructed::<O>(test, &buf).then_some(test)
        })
        .sum()
}

#[inline(never)]
fn can_be_constructed<O: Op>(result: Int, operands: &[Int]) -> bool {
    match operands {
        [] => false,
        [last] => result == *last,
        [rest @ .., last] => {
            O::rev(result, *last).any(|prev_result| can_be_constructed::<O>(prev_result, rest))
        }
    }
}

macro_rules! impl_op {
    ($id:ident, $res:ident, $operand:ident, $expr:expr) => {
        struct $id;
        impl Op for $id {
            fn rev($res: Int, $operand: Int) -> impl Iterator<Item = Int> {
                std::iter::from_fn(move || $expr).take(1)
            }
        }
    };
}
impl_op!(
    Mul,
    result,
    op,
    result.is_multiple_of(op).then(|| result / op)
);
impl_op!(Add, result, op, (result >= op).then(|| result - op));
impl_op!(Concat, result, op, deconcat(result, op));

struct Or<A, B>((A, B));
impl<A: Op, B: Op> Op for Or<A, B> {
    fn rev(r: Int, a: Int) -> impl Iterator<Item = Int> {
        A::rev(r, a).chain(B::rev(r, a))
    }
}

trait Op {
    fn rev(r: Int, a: Int) -> impl Iterator<Item = Int>;
}

fn prefixes(n: Int) -> impl Iterator<Item = Int> {
    std::iter::successors(Some(n), |&x| Some(x / 10)).take_while(|&x| x > 0)
}
#[inline(always)]
fn deconcat(long: Int, suffix: Int) -> Option<Int> {
    use itertools::EitherOrBoth::*;
    prefixes(long)
        .zip_longest(prefixes(suffix))
        .take_while(|x| match x {
            Both(a, b) => a % 10 == b % 10,
            Left(_) => true,
            Right(_) => false,
        })
        .find_map(|x| x.just_left())
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_deconcat() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(super::deconcat(123, 32), None);
        assert_eq!(super::deconcat(123, 4123), None);
        assert_eq!(super::deconcat(123, 23), Some(1));
    }

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
