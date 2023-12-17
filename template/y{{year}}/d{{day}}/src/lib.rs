pub fn solve_part_1(file_content: &str) -> usize {
    todo!("part 1 is not implemented yet")
}
pub fn solve_part_2(_file_content: &str) -> usize {
    todo!("part 2 is not implemented yet")
}

#[cfg(test)]
mod tests {
    use super::solve;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "0");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "0");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
