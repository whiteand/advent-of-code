use std::cmp::Ordering;

pub fn solve_part_1(file_content: &str) -> usize {
    solve::<0>(file_content)
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve::<1>(file_content)
}

fn solve<const TOLERATES: usize>(file_content: &str) -> usize {
    let mut report = Vec::with_capacity(10);
    let mut total = 0;
    for line in file_content.lines().filter(|line| !line.is_empty()) {
        let nums = line
            .split_ascii_whitespace()
            .map(|x| x.parse::<usize>().unwrap());
        report.extend(nums);
        if is_safe(&mut report, TOLERATES) {
            total += 1;
        }
        report.clear();
    }

    total
}

// [1,2,3,4]
// table:
//    [1,2,3,4]
// t\i 0 1 2 3
//   0 ?
//   1
// table[tolerations][index] = is_proper(index, index+1)
//    ? table[tolerations][index + 1] || is_proper(index, index + 2) && table[tolerations][index + 2]
//    : table[tolerations-1][index + 1]

fn is_safe(report: &mut [usize], tolerates: usize) -> bool {
    // Trivial case
    if report.len() <= tolerates + 1 {
        return true;
    }
    let order = get_order(report, tolerates);
    is_safe_dp(report, tolerates, order)
}

fn is_safe_dp(report: &mut [usize], tolerates: usize, order: Ordering) -> bool {
    let mut bad = 0;
    let mut last = 0;
    let mut i = 1;

    // assuming that first element is valid level
    // invalid levels will be collected into `bad` variable
    while i < report.len() && bad <= tolerates {
        if valid_neighbours(report[last], report[i], order) {
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
        is_safe_dp(&mut report[1..], tolerates - 1, order)
    } else {
        false
    }
}

fn valid_neighbours(prev: usize, next: usize, order: Ordering) -> bool {
    next.cmp(&prev) == order && next.abs_diff(prev) <= 3
}

/// Makes a voting of first (tolerates + 1) items orderings
/// to define what is the expected order of the report
fn get_order(report: &[usize], tolerates: usize) -> Ordering {
    let votes = (tolerates + 2).min(report.len() - 1);
    let greaters = (0..votes).filter(|&i| report[i] < report[i + 1]).count();

    if greaters >= votes - greaters {
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
