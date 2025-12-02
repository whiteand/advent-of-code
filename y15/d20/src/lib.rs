pub fn solve_part_1(file_content: &str) -> usize {
    let value = file_content.trim().parse::<u64>().unwrap() as usize;
    (1..)
        .find(|&x| divisors(x).sum::<usize>() * 10 >= value)
        .unwrap_or_default()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let value = file_content.trim().parse::<u64>().unwrap() as usize;
    (1..)
        .find(|&i| divisors(i).filter(|d| i <= 50 * d).sum::<usize>() * 11 >= value)
        .unwrap_or_default()
}

fn divisors(n: usize) -> impl Iterator<Item = usize> {
    (1..=n)
        .take_while(move |&x| x * x <= n)
        .filter(move |x| n.is_multiple_of(*x))
        .flat_map(move |x| [x, n / x].into_iter().skip(if x * x == n { 1 } else { 0 }))
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    #[ignore] // slow
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "831600");
    }

    #[test]
    #[ignore] // slow
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "884520");
    }
}
