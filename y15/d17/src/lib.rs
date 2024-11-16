use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> usize {
    numbers_of_ways(&parse_containers(file_content), 150)
}

pub fn solve_part_2(file_content: &str) -> usize {
    min_numbers_of_ways(&parse_containers(file_content), 150)
}

fn numbers_of_ways(containers: &[usize], target: usize) -> usize {
    let mut res = 0;
    for i in 1..containers.len() {
        for combo in containers.iter().combinations(i) {
            let sum = combo.iter().copied().sum::<usize>();
            if sum == target {
                res += 1;
            }
        }
    }
    res
}
fn min_numbers_of_ways(containers: &[usize], target: usize) -> usize {
    for i in 1..containers.len() {
        let mut res = 0;
        for combo in containers.iter().combinations(i) {
            let sum = combo.iter().copied().sum::<usize>();
            if sum == target {
                res += 1;
            }
        }
        if res > 0 {
            return res;
        }
    }
    0
}

fn parse_containers(file_content: &str) -> Vec<usize> {
    file_content
        .lines()
        .map(|x| x.parse::<u16>().unwrap() as usize)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(
            super::numbers_of_ways(&super::parse_containers(EXAMPLE), 25),
            4
        );
    }
    #[test]
    fn test_part2() {
        assert_eq!(
            super::min_numbers_of_ways(&super::parse_containers(EXAMPLE), 25),
            3
        );
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1638");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "17");
    }
}
