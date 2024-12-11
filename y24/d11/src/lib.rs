use core::str;
use itertools::Itertools;
use std::collections::HashMap;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, 25)
}
#[tracing::instrument(skip(file_content))]
pub fn solve(file_content: &str, blinks: usize) -> usize {
    let mut nums = file_content
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .counts();
    let mut buf = HashMap::new();

    for _ in 0..blinks {
        blink(&nums, &mut buf);
        (nums, buf) = (buf, nums);
    }

    if blinks % 2 == 0 {
        buf.values().sum()
    } else {
        nums.values().sum()
    }
}
fn blink(src: &HashMap<usize, usize>, dst: &mut HashMap<usize, usize>) {
    dst.clear();
    for (stone, n) in src {
        let stone = *stone;
        if stone == 0 {
            *dst.entry(1).or_insert(0) += n;
        } else if let Some((left, right)) = split_even(stone) {
            *dst.entry(left).or_insert(0) += n;
            *dst.entry(right).or_insert(0) += n;
        } else {
            *dst.entry(stone * 2024).or_insert(0) += n;
        }
    }
}

fn split_even(n: usize) -> Option<(usize, usize)> {
    match n {
        0..10 => None,
        10..100 => Some((n / 10, n % 10)),
        100..1000 => None,
        1000..10000 => Some((n / 100, n % 100)),
        10000..100000 => None,
        100000..1000000 => Some((n / 1000, n % 1000)),
        1000000..10000000 => None,
        10000000..100000000 => Some((n / 10000, n % 10000)),
        100000000..1000000000 => None,
        1000000000..10000000000 => Some((n / 100000, n % 100000)),
        10000000000..100000000000 => None,
        100000000000..1000000000000 => Some((n / 1000000, n % 1000000)),
        1000000000000..10000000000000 => None,
        10000000000000..100000000000000 => Some((n / 10000000, n % 10000000)),
        100000000000000..1000000000000000 => None,
        1000000000000000..10000000000000000 => Some((n / 100000000, n % 100000000)),
        10000000000000000..100000000000000000 => None,
        100000000000000000..1000000000000000000 => Some((n / 1000000000, n % 1000000000)),
        x => unreachable!("{x}"),
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, 75)
}

#[cfg(test)]
mod tests {

    use crate::split_even;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_split_half() {
        assert_eq!(None, split_even(123));
        assert_eq!(Some((12, 34)), split_even(1234));
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "55312");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "185894");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "221632504974231");
    }
}
