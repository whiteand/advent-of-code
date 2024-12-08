use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    count_antinodes(file_content, 2, 1)
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    count_antinodes(file_content, 0, usize::MAX)
}

#[tracing::instrument(skip(file_content))]
pub fn count_antinodes(file_content: &str, skip: usize, take: usize) -> usize {
    let (sizes, map) = parse(file_content);

    let mut antinodes = HashSet::new();
    for pair in map.keys().copied().combinations(2).filter(
        |combo| matches!((map.get(&combo[0]), map.get(&combo[1])),(Some(x), Some(y)) if x == y),
    ) {
        let [a, b] = pair[..] else {
            unreachable!();
        };
        for first in iter_antinodes(sizes, a, b).skip(skip).take(take) {
            antinodes.insert(first);
        }
        for second in iter_antinodes(sizes, b, a).skip(skip.max(2)).take(take) {
            antinodes.insert(second);
        }
    }

    antinodes.len()
}

pub fn iter_antinodes(sizes: IVec2, a: IVec2, b: IVec2) -> impl Iterator<Item = IVec2> {
    let diff = b - a;
    std::iter::successors(Some(a), move |x| Some(*x + diff))
        .take_while(move |p| (0..sizes.x).contains(&p.x) && (0..sizes.y).contains(&p.y))
}

fn parse(input: &str) -> (IVec2, HashMap<glam::IVec2, char>) {
    let rows = input.lines().count();
    let cols = input.lines().map(|x| x.len()).next().unwrap();

    (
        IVec2::new(cols as i32, rows as i32),
        input
            .lines()
            .filter(|x| !x.is_empty())
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, x)| *x != '.')
                    .map(move |(c, b)| (IVec2::new(r as i32, c as i32), b))
            })
            .collect(),
    )
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "14");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "394");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "34");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1277");
    }
}
