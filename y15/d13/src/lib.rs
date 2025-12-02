use std::collections::BTreeMap;

use itertools::Itertools;
use nom::Parser;

pub fn solve_part_1(file_content: &str) -> i64 {
    let pleasure = parse_pleasure(file_content);
    solve(pleasure)
}

pub fn solve_part_2(file_content: &str) -> i64 {
    let mut pleasure = parse_pleasure(file_content);

    pleasure.insert("me", Default::default());

    solve(pleasure)
}

fn solve(pleasure: BTreeMap<&str, BTreeMap<&str, i64>>) -> i64 {
    let persons = pleasure.keys().copied().collect_vec();
    (0..persons.len())
        .permutations(persons.len())
        .map(|indexes| {
            indexes
                .iter()
                .copied()
                .chain(std::iter::once(indexes[0]))
                .map(|x| persons[x])
                .tuple_windows()
                .map(|(i, j)| {
                    pleasure
                        .get(i)
                        .and_then(|m| m.get(j).copied())
                        .unwrap_or_default()
                        + pleasure
                            .get(j)
                            .and_then(|m| m.get(i).copied())
                            .unwrap_or_default()
                })
                .sum::<i64>()
        })
        .max()
        .unwrap()
}

fn parse_pleasure(file_content: &str) -> BTreeMap<&str, BTreeMap<&str, i64>> {
    let mut pleasure = BTreeMap::new();
    for line in file_content.lines() {
        if line.is_empty() {
            continue;
        }
        // Alice would gain 54 happiness units by sitting next to Bob.
        let (_, (person, _, sign, _, abs, _, neighbour, _)) = (
            nom::character::complete::alpha1::<&str, nom::error::Error<&str>>,
            nom::bytes::complete::tag(" would "),
            nom::branch::alt((
                nom::bytes::complete::tag("lose").map(|_| -1i64),
                nom::bytes::complete::tag("gain").map(|_| 1i64),
            )),
            nom::bytes::complete::tag(" "),
            nom::character::complete::i64,
            nom::bytes::complete::tag(" happiness units by sitting next to "),
            nom::character::complete::alpha1,
            nom::bytes::complete::tag("."),
        )
            .parse(line)
            .unwrap();

        pleasure
            .entry(person)
            .or_insert(BTreeMap::new())
            .insert(neighbour, sign * abs);
    }
    pleasure
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "330");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "664");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "286");
    }

    #[test]
    #[ignore] // slow
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "640");
    }
}
