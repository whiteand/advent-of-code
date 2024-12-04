#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    todo!("part 1 is not implemented yet: {file_content}")
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    todo!("part 2 is not implemented yet: {file_content}")
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
