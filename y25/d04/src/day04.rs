use advent_utils::{
    glam::IVec2,
    grid::{Grid, N8},
};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(file_content);
    can_be_removed(&grid).count()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let mut grid = advent_utils::parse::ascii_grid(file_content);
    let mut removed = 0;
    loop {
        let to_remove = can_be_removed(&grid).collect_vec();
        if to_remove.is_empty() {
            break;
        }
        removed += to_remove.len();
        for (p, _) in to_remove {
            grid.set(p, b'.');
        }
    }
    removed
}

pub fn can_be_removed(grid: &Grid<u8>) -> impl Iterator<Item = (IVec2, u8)> {
    grid.entries()
        .map(|(p, c)| (p, *c))
        .filter(|(p, current_char)| {
            *current_char == b'@'
                && grid
                    .neighbours(*p, N8)
                    .map(|(p, c)| (p, *c))
                    .filter(|(_, neighbour)| *neighbour == b'@')
                    .count()
                    < 4
        })
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "13")]
    #[case::actual(ACTUAL, "1384")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "43")]
    #[case::actual(ACTUAL, "8013")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
