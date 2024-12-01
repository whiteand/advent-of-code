pub fn solve_part_1(file_content: &str) -> usize {
    let (mut xs, mut ys) = parse(file_content);

    xs.sort_unstable();
    ys.sort_unstable();

    xs.into_iter().zip(ys).map(|(a, b)| a.abs_diff(b)).sum()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let (xs, ys) = parse(file_content);
    let max_y = ys.iter().copied().max().unwrap_or_default();
    let mut cnt = vec![0; max_y + 1];
    for y in ys {
        cnt[y] += 1;
    }

    xs.into_iter()
        .filter(|x| *x <= max_y)
        .map(|x| cnt[x] * x)
        .sum()
}

fn parse(file_content: &str) -> (Vec<usize>, Vec<usize>) {
    file_content
        .lines()
        .filter(|x| !x.is_empty())
        .map(|line| {
            let (x, y) = line.split_once("   ").unwrap();
            (
                x.parse::<u64>().unwrap() as usize,
                y.parse::<u64>().unwrap() as usize,
            )
        })
        .unzip()
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
