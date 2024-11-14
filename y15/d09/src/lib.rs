use std::collections::BTreeMap;

use itertools::Itertools;

fn paths_distances(file_content: &str) -> impl Iterator<Item = u64> + '_ {
    let mut neighbours = BTreeMap::new();
    for line in file_content.lines() {
        let (source, b) = line.split_once(" to ").unwrap();
        let (target, value) = b.split_once(" = ").unwrap();
        let value = value.parse::<u64>().unwrap();
        neighbours
            .entry(source)
            .or_insert(BTreeMap::new())
            .insert(target, value);
        neighbours
            .entry(target)
            .or_insert(BTreeMap::new())
            .insert(source, value);
    }
    let cities = neighbours.keys().copied().collect_vec();
    (0..neighbours.len())
        .permutations(neighbours.len())
        .map(move |path| {
            path.iter()
                .tuple_windows()
                .map(|(from, target)| {
                    neighbours
                        .get(cities[*from])
                        .unwrap()
                        .get(cities[*target])
                        .copied()
                        .unwrap_or(u64::MAX)
                })
                .fold(0u64, |a, b| a.saturating_add(b))
        })
}
pub fn solve_part_1(file_content: &str) -> u64 {
    paths_distances(file_content).min().unwrap()
}
pub fn solve_part_2(file_content: &str) -> u64 {
    paths_distances(file_content).max().unwrap()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "605");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "117");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "982");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "909");
    }
}
