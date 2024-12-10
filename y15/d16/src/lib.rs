use std::collections::BTreeMap;

use itertools::Itertools;
use nom::Parser;

const DRAFT: &str = r#"children: 3
cats: 7
samoyeds: 2
pomeranians: 3
akitas: 0
vizslas: 0
goldfish: 5
trees: 3
cars: 2
perfumes: 1"#;

pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, is_sus)
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, is_sus2)
}

fn solve(
    file_content: &str,
    is_sus: fn(&BTreeMap<&str, usize>, &BTreeMap<&str, usize>) -> bool,
) -> usize {
    let sues = parse_sues(file_content).unwrap();
    let draft = DRAFT
        .lines()
        .map(|line| {
            let (prop, value) = line.split_once(": ").unwrap();
            (prop, value.parse::<u64>().unwrap() as usize)
        })
        .collect::<BTreeMap<_, _>>();

    sues.into_iter()
        .find_map(|sue| is_sus(&draft, &sue.properties).then_some(sue))
        .map(|x| x.id)
        .unwrap()
}

fn is_sus(draft: &BTreeMap<&str, usize>, actual: &BTreeMap<&str, usize>) -> bool {
    for (k, v) in actual {
        if let Some(expected_value) = draft.get(k) {
            if expected_value != v {
                return false;
            }
        }
    }
    true
}
fn is_sus2(draft: &BTreeMap<&str, usize>, actual: &BTreeMap<&str, usize>) -> bool {
    for (k, v) in actual {
        match *k {
            "cats" | "trees" => {
                if let Some(expected_value) = draft.get(k) {
                    if expected_value >= v {
                        return false;
                    }
                }
            }
            "pomeranians" | "goldfish" => {
                if let Some(expected_value) = draft.get(k) {
                    if expected_value <= v {
                        return false;
                    }
                }
            }
            _ => {
                if let Some(expected_value) = draft.get(k) {
                    if expected_value != v {
                        return false;
                    }
                }
            }
        }
    }
    true
}

#[derive(Debug)]
struct Sue<'i> {
    id: usize,
    properties: BTreeMap<&'i str, usize>,
}

//  Sue \d+: ((vizslas|pomeranians|perfumes|cats|trees|samoyeds|children|akitas|cars|goldfish): \d+(, )?)+

fn parse_sues(file_content: &str) -> Result<Vec<Sue>, nom::Err<nom::error::Error<&str>>> {
    file_content
        .lines()
        .map(parse_sue)
        .map_ok(|(_, x)| x)
        .collect()
}

fn parse_sue(line: &str) -> nom::IResult<&str, Sue> {
    let usize_parser = || nom::character::complete::u64.map(|x| x as usize);
    let entry_parser = nom::sequence::separated_pair(
        nom::character::complete::alpha1,
        nom::bytes::complete::tag(": "),
        usize_parser(),
    );
    let properties_parser =
        nom::multi::separated_list1(nom::bytes::complete::tag(", "), entry_parser)
            .map(|pairs| pairs.into_iter().collect::<BTreeMap<_, _>>());
    let mut sue_parser = nom::sequence::tuple((
        nom::bytes::complete::tag("Sue "),
        usize_parser(),
        nom::bytes::complete::tag(": "),
        properties_parser,
    ))
    .map(|(_, id, _, properties)| Sue { id, properties });

    sue_parser.parse(line)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "213");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "323");
    }
}
