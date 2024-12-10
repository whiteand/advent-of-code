mod trie;

use itertools::Itertools;
use trie::Trie;

pub fn solve_part_1(file_content: &str) -> usize {
    let (replacements, molecule) = parse_input(file_content);
    let mut output = Vec::with_capacity(molecule.len());
    modify(&replacements, molecule.trim(), &mut output);
    output.len()
}

fn modify(replacements: &[(&str, Vec<&str>)], molecule: &str, output: &mut Vec<String>) {
    let mut buf = String::with_capacity(molecule.len());
    let mut visited = Trie::new();

    for (inp, options) in replacements {
        let mut next_ind = 0;
        while let Some(ind) = molecule.get(next_ind..).unwrap_or_default().find(inp) {
            let real_index = ind + next_ind;
            for option in options {
                buf.push_str(molecule.get(0..real_index).unwrap_or_default());
                buf.push_str(option);
                buf.push_str(molecule.get((real_index + inp.len())..).unwrap_or_default());
                if !visited.contains(&buf) {
                    visited.push(&buf);
                    output.push(buf.clone())
                }
                buf.clear();
            }
            next_ind = real_index + 1;
        }
    }
}
pub fn solve_part_2(file_content: &str) -> usize {
    let (replacements, molecule) = parse_input(file_content);
    let mut molecule = molecule.to_owned();
    let mut res = 0;
    loop {
        let mut done = true;
        for (input, outputs) in replacements.iter() {
            for output in outputs {
                if let Some(pos) = molecule.find(output) {
                    let mut new_string = String::new();
                    new_string.push_str(molecule.get(0..pos).unwrap_or_default());
                    new_string.push_str(input);
                    new_string.push_str(molecule.get((pos + output.len())..).unwrap_or_default());
                    molecule = new_string;
                    res += 1;
                    done = false;
                }
            }
        }
        if done {
            break;
        }
    }
    res
}

fn parse_input(file_content: &str) -> (Vec<(&str, Vec<&str>)>, &str) {
    let (replacements_input, molecule) = file_content.split_once("\n\n").unwrap();

    let mut replacements = replacements_input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" => ").unwrap();
            (left, right)
        })
        .into_group_map_by(|x| x.0)
        .into_iter()
        .map(|(a, b)| (a, b.into_iter().map(|x| x.1).collect_vec()))
        .collect_vec();

    for (_, nexts) in replacements.iter_mut() {
        nexts.sort_by_key(|x| x.len());
        nexts.reverse();
    }
    replacements.sort_by_key(|x| x.0.len());
    replacements.reverse();

    (replacements, molecule)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "7");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "518");
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            format!(
                "{}",
                solve_part_2(
                    r#"e => H
e => O
H => HO
H => OH
O => HH

HOHOHO"#
                )
            ),
            "6"
        );
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "200");
    }
}
