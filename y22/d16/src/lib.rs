pub mod parse;
mod part1;
mod part2;
mod shortest;
mod step;
pub mod valve;

pub fn solve_part_1(file_content: &str, minutes: usize) -> usize {
    part1::solve_part_1(file_content, minutes)
}
pub fn solve_part_2(file_content: &str) -> usize {
    part2::solve_part_2(file_content, 26)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT, 30)), "1651");
    }

    #[test]
    #[ignore] // slow
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL, 30)), "1728");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "1707");
    }

    #[test]
    #[ignore] // slow
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "2304");
    }
}
