mod trie;
use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> usize {
    let (replacements, molecule) = parse_input(file_content);
    println!("{replacements:?}\n{molecule}");
    let inputs = replacements.keys().copied().collect_vec();
    modify(&replacements, &inputs, molecule).len()
}

fn modify(
    replacements: &BTreeMap<&str, Vec<&str>>,
    inputs: &[&str],
    molecule: &str,
) -> Vec<String> {
    let mut buf = String::with_capacity(molecule.len());
    let mut res: BTreeSet<String> = BTreeSet::new();

    for i in 0..molecule.as_bytes().len() {
        for inp in inputs {
            if molecule.as_bytes()[i..].starts_with(inp.as_bytes()) {
                let options = replacements.get(inp).unwrap();
                for option in options.iter() {
                    buf.push_str(molecule.get(0..i).unwrap_or_default());
                    buf.push_str(option);
                    buf.push_str(molecule.get((i + inp.len())..).unwrap_or_default());
                    res.insert(buf.clone());
                    buf.clear();
                }
            }
        }
    }

    res.into_iter().collect_vec()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let (replacements, molecule) = parse_reverse_input(file_content);
    let mut tasks = BinaryHeap::new();
    let mut inputs = replacements.keys().copied().collect_vec();
    inputs.sort_by_key(|a| a.len());
    inputs.reverse();
    println!("{inputs:?}");
    let max_diff = replacements
        .iter()
        .flat_map(|(prev, nexts)| nexts.iter().map(|next| prev.len() - next.len()))
        .max()
        .unwrap_or(usize::MAX);
    tasks.push(Task::new(0, molecule.trim().to_owned(), max_diff));
    let mut visited = trie::Trie::new();
    while let Some(task) = tasks.pop() {
        if tasks.len() % 1000 == 0 {
            println!("{} => {} | rem: {}", task.score, task.str, tasks.len());
        }
        if &task.str == "e" {
            return task.steps;
        }
        let next_iterations = modify(&replacements, &inputs, &task.str);
        visited.push(&task.str);
        for it in next_iterations {
            if visited.contains(&it) {
                continue;
            }
            tasks.push(Task::new(task.steps + 1, it, max_diff));
        }
    }
    0
}

fn parse_input(file_content: &str) -> (BTreeMap<&str, Vec<&str>>, &str) {
    let (replacements_input, molecule) = file_content.split_once("\n\n").unwrap();
    let replacements = replacements_input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" => ").unwrap();
            (left, right)
        })
        .fold(
            BTreeMap::new(),
            |mut map: BTreeMap<&str, Vec<&str>>, (left, right)| {
                map.entry(left).or_default().push(right);
                map
            },
        );

    (replacements, molecule)
}

#[derive(Eq, PartialEq, Debug)]
struct Task {
    str: String,
    steps: usize,
    score: usize,
}
impl Task {
    fn new(steps: usize, str: String, max_diff: usize) -> Self {
        let score = steps * max_diff + (str.len() - 1);
        Self { str, steps, score }
    }
}
impl std::cmp::PartialOrd for Task {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Task {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}
fn parse_reverse_input(file_content: &str) -> (BTreeMap<&str, Vec<&str>>, &str) {
    let (replacements_input, molecule) = file_content.split_once("\n\n").unwrap();
    let replacements = replacements_input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" => ").unwrap();
            (right, left)
        })
        .fold(
            BTreeMap::new(),
            |mut map: BTreeMap<&str, Vec<&str>>, (left, right)| {
                map.entry(left).or_default().push(right);
                map
            },
        );

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
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
