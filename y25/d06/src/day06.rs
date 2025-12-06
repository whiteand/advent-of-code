use advent_utils::{
    glam::IVec2,
    grid::Grid,
    nom::{
        self, AsChar, Parser,
        branch::alt,
        character::{self, complete::multispace1},
        multi::separated_list1,
    },
};
use itertools::Itertools;

#[derive(Debug, Copy, Clone)]
enum Operation {
    Multiply,
    Add,
}

impl Operation {
    pub fn execute(&self, values: impl Iterator<Item = u128>) -> u128 {
        match self {
            Operation::Multiply => values
                .reduce(|acc, element| acc * element)
                .unwrap_or_default(),
            Operation::Add => values
                .reduce(|acc, element| acc + element)
                .unwrap_or_default(),
        }
    }
}

fn parse_operation(input: &str) -> nom::IResult<&str, Operation> {
    alt((
        character::char('+').map(|_| Operation::Add),
        character::char('*').map(|_| Operation::Multiply),
    ))
    .parse(input)
}
fn parse_operations(input: &str) -> nom::IResult<&str, Vec<Operation>> {
    separated_list1(multispace1, parse_operation).parse(input)
}

#[tracing::instrument(skip(input))]
pub fn part1(input: &str) -> u128 {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let grid: Grid<u128> = lines
        .iter()
        .copied()
        .take(lines.len() - 1)
        .map(|line| advent_utils::parse::nums::<u128>(line))
        .collect();

    let operations = parse_operations(lines.last().unwrap())
        .map(|(_, ops)| ops)
        .unwrap();

    let grid = grid.transpose().expect("should be a rectangular grid");

    std::iter::zip(grid.rows(), operations.into_iter())
        .map(|(row, op)| op.execute(row.iter().copied()))
        .sum()
}
#[tracing::instrument(skip(input))]
pub fn part2(input: &str) -> u128 {
    let grid = advent_utils::parse::ascii_grid(input);
    let operations: Vec<(usize, Operation)> = grid
        .rows()
        .last()
        .into_iter()
        .flat_map(|r| r.iter().copied().enumerate())
        .filter_map(|(i, r)| match r {
            b'*' => Some((i, Operation::Multiply)),
            b'+' => Some((i, Operation::Add)),
            _ => None,
        })
        .collect();

    let max_col = grid.rows().map(|x| x.len()).max().unwrap_or_default();

    let filled: Vec<(usize, usize)> = operations
        .iter()
        .map(|(a, _)| *a)
        .chain(std::iter::once(max_col + 1))
        .tuple_windows()
        .map(|(a, b)| (a, b - 1))
        .collect();

    std::iter::zip(filled.into_iter(), operations.into_iter().map(|(_, op)| op))
        .map(|((start_col, end_col), op)| {
            op.execute((start_col..end_col).rev().map(|c| {
                grid.iter_line(IVec2::new(c as i32, 0), IVec2::Y)
                    .copied()
                    .filter(|c| c.is_dec_digit())
                    .fold(0, |acc, x| acc * 10 + (x - b'0') as u128)
            }))
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "4277556")]
    #[case::actual(ACTUAL, "5322004718681")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "3263827")]
    #[case::actual(ACTUAL, "9876636978528")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
