use core::str;
use itertools::Itertools;
use std::collections::HashMap;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, 25)
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, 75)
}

#[tracing::instrument(skip(file_content))]
pub fn solve(file_content: &str, blinks: usize) -> usize {
    let mut nums = file_content
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .counts();
    let mut buf = HashMap::new();

    for _ in 0..(blinks / 2) {
        blink(&nums, &mut buf);
        blink(&buf, &mut nums);
    }

    if blinks % 2 == 1 {
        blink(&nums, &mut buf);
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
            *dst.entry(1).or_default() += n;
        } else if let Some(pair) = split_even(stone) {
            for x in pair {
                *dst.entry(x).or_default() += n;
            }
        } else {
            *dst.entry(stone * 2024).or_default() += n;
        }
    }
}

fn split_even(n: usize) -> Option<[usize; 2]> {
    let mut min = 10;
    let mut max = 100;
    let mut div = 10;
    loop {
        if n < min {
            return None;
        }
        if n < max {
            return Some([n / div, n % div]);
        }
        min *= 100;
        max *= 100;
        div *= 10;
    }
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
        assert_eq!(Some([12, 34]), split_even(1234));
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
