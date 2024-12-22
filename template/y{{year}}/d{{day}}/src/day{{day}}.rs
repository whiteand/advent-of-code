#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    file_content.len()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(ACTUAL)), "0");
    }
}
