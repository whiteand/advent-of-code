use std::collections::HashSet;

pub fn solve_part_1(file_content: &str) -> isize {
    file_content
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .scan(0isize, |a, b| {
            *a += b;
            Some(*a)
        })
        .last()
        .unwrap_or(0)
}
pub fn solve_part_2(file_content: &str) -> isize {
    let mut freq = 0isize;
    let mut seen = HashSet::new();
    let dvs = file_content
        .lines()
        .map(|x| x.parse::<isize>().unwrap())
        .collect::<Vec<_>>();
    for dv in dvs.iter() {
        freq += *dv;
        seen.insert(freq);
    }
    loop {
        for dv in dvs.iter() {
            freq += *dv;
            if seen.contains(&freq) {
                return freq;
            }
            seen.insert(freq);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "3");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "486");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "2");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "69285");
    }
}
