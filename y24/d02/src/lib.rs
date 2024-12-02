use std::cmp::Ordering;

use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> usize {
    solve::<0>(file_content)
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve::<1>(file_content)
}

fn solve<const TOLERATES: usize>(file_content: &str) -> usize {
    file_content
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .filter_map(|mut report| is_safe(&mut report, TOLERATES).then_some(1))
        .count()
}

fn is_safe(report: &mut [usize], tolerates: usize) -> bool {
    if report.len() <= tolerates + 1 {
        return true;
    }
    let mut bad = 0;
    let mut last = 0;
    let order = get_order(report, tolerates);
    let mut i = 1;

    // assuming that first element is good
    // invalid levels will be collected into `bad` variable
    while i < report.len() && bad <= tolerates {
        if good_order(report[last], report[i], order) {
            last = i;
            i += 1;
            continue;
        }
        bad += 1;
        i += 1;
    }

    if bad <= tolerates {
        true
    } else if tolerates > 0 {
        // Removing first element
        is_safe(&mut report[1..], tolerates - 1)
    } else {
        false
    }
}

fn good_order(prev: usize, next: usize, order: Ordering) -> bool {
    next.cmp(&prev) == order && next.abs_diff(prev) <= 3
}
/// Makes a voting of first (tolerates + 1) items orderings
fn get_order(report: &[usize], tolerates: usize) -> Ordering {
    let votes = (tolerates + 2).min(report.len() - 1);
    let incs = (0..votes).filter(|&i| report[i] < report[i + 1]).count();

    if incs >= votes - incs {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "2");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "321");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "4");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "386");
    }
}
