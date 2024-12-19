use std::collections::{HashMap, HashSet};

use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let (towels_str, patterns_str) = file_content.split_once("\n\n").unwrap();
    let towels = towels_str.split(", ").collect_vec();
    let mut possible = HashSet::new();
    let mut impossible = HashSet::new();
    let mut total = 0;
    for pattern in patterns_str.lines() {
        if is_possible(&towels, pattern, &mut possible, &mut impossible) {
            total += 1;
        }
    }

    total
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let (towels_str, patterns_str) = file_content.split_once("\n\n").unwrap();
    let towels = towels_str.split(", ").collect_vec();
    let mut possible = HashMap::new();
    let mut impossible = HashSet::new();
    let mut total = 0;
    for pattern in patterns_str.lines() {
        total += ways(&towels, pattern, &mut possible, &mut impossible)
    }

    total
}

#[tracing::instrument(skip(towels, possible, impossible), ret)]
fn is_possible<'t>(
    towels: &[&'t str],
    pattern: &'t str,
    possible: &mut HashSet<&'t str>,
    impossible: &mut HashSet<&'t str>,
) -> bool {
    if pattern.is_empty() {
        return true;
    }
    if possible.contains(pattern) {
        return true;
    }
    if impossible.contains(pattern) {
        return false;
    }
    for prefix in towels {
        if let Some(rest) = pattern.strip_prefix(prefix) {
            tracing::info!(?prefix);
            let is_possible = is_possible(towels, rest, possible, impossible);
            if is_possible {
                possible.insert(pattern);
                return true;
            }
        }
    }
    impossible.insert(pattern);
    false
}
#[tracing::instrument(skip(towels, possible, impossible), ret)]
fn ways<'t>(
    towels: &[&'t str],
    pattern: &'t str,
    possible: &mut HashMap<&'t str, usize>,
    impossible: &mut HashSet<&'t str>,
) -> usize {
    if pattern.is_empty() {
        return 1;
    }
    if let Some(x) = possible.get(pattern) {
        return *x;
    }
    if impossible.contains(pattern) {
        return 0;
    }
    let mut total_ways = 0;
    for prefix in towels {
        if let Some(rest) = pattern.strip_prefix(prefix) {
            tracing::info!(?prefix);
            total_ways += ways(towels, rest, possible, impossible);
        }
    }
    if total_ways > 0 {
        possible.insert(pattern, total_ways);
    } else {
        impossible.insert(pattern);
    }
    total_ways
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "6");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "296");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "16");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "619970556776002");
    }
}
