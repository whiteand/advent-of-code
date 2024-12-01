use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> u64 {
    let (mut xs, mut ys) = parse(file_content);

    xs.sort_unstable();
    ys.sort_unstable();

    xs.into_iter()
        .zip(ys)
        .map(|(a, b)| if a > b { a - b } else { b - a })
        .sum()
}
pub fn solve_part_2(file_content: &str) -> u64 {
    let (xs, ys) = parse(file_content);
    let ys_cnt = ys.into_iter().counts();
    xs.into_iter()
        .map(|x| ys_cnt.get(&x).copied().unwrap_or_default() as u64 * x)
        .sum()
}

fn parse(file_content: &str) -> (Vec<u64>, Vec<u64>) {
    let mut xs = vec![];
    let mut ys = vec![];
    for (x, y) in file_content.lines().filter(|x| !x.is_empty()).map(|line| {
        line.split_ascii_whitespace()
            .map(|x| x.parse().unwrap())
            .collect_tuple::<(u64, u64)>()
            .unwrap()
    }) {
        xs.push(x);
        ys.push(y);
    }
    (xs, ys)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "11");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "765748");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "31");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "27732508");
    }
}
