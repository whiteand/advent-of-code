use core::str;
use itertools::Itertools;
use rayon::prelude::*;

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
        .collect_vec();
    let mut buf = Vec::with_capacity(nums.len());
    for i in 0..blinks {
        tracing::info!(n = nums.len(), i, "blink");
        blink(&mut nums, &mut buf);
        (nums, buf) = (buf, nums);
    }
    tracing::info!(b = buf.len(), s = nums.len());

    if blinks % 2 == 0 {
        buf.len()
    } else {
        nums.len()
    }
}
fn blink(src: &mut Vec<usize>, dst: &mut Vec<usize>) {
    dst.clear();
    dst.par_extend(src.par_iter().flat_map_iter(|stone| {
        let stone = *stone;
        if stone == 0 {
            [1, 0].into_iter().take(1)
        } else if let Some((left, right)) = split_even(stone) {
            [left, right].into_iter().take(2)
        } else {
            [stone * 2024, 0].into_iter().take(1)
        }
    }));
}

fn split_even(n: usize) -> Option<(usize, usize)> {
    let digits = count_digits(n);
    if digits % 2 == 0 {
        Some(split_at(n, digits / 2))
    } else {
        None
    }
}
fn split_at(mut n: usize, k: usize) -> (usize, usize) {
    let mut reversed_right = 0;
    for _ in 0..k {
        reversed_right *= 10;
        reversed_right += n % 10;
        n /= 10;
    }
    let mut right = 0;
    for _ in 0..k {
        right *= 10;
        right += reversed_right % 10;
        reversed_right /= 10;
    }
    (n, right)
}
fn count_digits(n: usize) -> usize {
    if n < 10 {
        return 1;
    }
    return 1 + count_digits(n / 10);
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, 75)
}

#[cfg(test)]
mod tests {
    use crate::{solve, split_even};

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_split_half() {
        assert_eq!(None, split_even(123));
        assert_eq!(Some((12, 34)), split_even(1234));
    }
    #[test]
    fn test_zero() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        let res = solve("0", 16);
        assert_eq!(res, 0);
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
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
