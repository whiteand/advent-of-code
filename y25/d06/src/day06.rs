use advent_utils::{algo::CollectDecDigits, grid::Grid, parse};

#[derive(Debug, Copy, Clone)]
enum Operation {
    Multiply,
    Add,
}

impl TryFrom<u8> for Operation {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            b'+' => Ok(Operation::Add),
            b'*' => Ok(Operation::Multiply),
            _ => Err("Unexpected operation"),
        }
    }
}

impl Operation {
    fn binary(&self, a: usize, b: usize) -> usize {
        match self {
            Operation::Multiply => a * b,
            Operation::Add => a + b,
        }
    }
    fn default(&self) -> usize {
        match self {
            Operation::Multiply => 1,
            Operation::Add => 0,
        }
    }
    fn execute(&self, values: impl Iterator<Item = usize>) -> usize {
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

#[tracing::instrument(skip(input))]
pub fn part1(input: &str) -> usize {
    let lines = input.trim().lines().collect::<Vec<_>>();

    let grid = lines
        .iter()
        .copied()
        .take(lines.len() - 1)
        .map(|line| advent_utils::parse::nums::<usize>(line))
        .collect::<Grid<usize>>();

    let operations = lines
        .last()
        .unwrap()
        .as_bytes()
        .iter()
        .copied()
        .filter_map(|b| Operation::try_from(b).ok())
        .collect::<Vec<_>>();

    std::iter::zip(0..grid.max_column(), operations.into_iter())
        .map(|(c, op)| op.execute(grid.iter_col_copied(c)))
        .sum()
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(input);
    let mut current_op = Operation::Add;
    let mut current_section = 0usize;
    let mut total = 0usize;
    let rows = grid.rows_len();
    for c in 0..grid.max_column() {
        if let Some(new_op) = grid
            .get_copy_at(rows - 1, c)
            .and_then(|x| Operation::try_from(x).ok())
        {
            current_op = new_op;
            total += current_section;
            current_section = new_op.default();
        }

        let Some(number) = parse::digits(grid.iter_col_copied(c)).collect_dec_digits::<usize>()
        else {
            // empty line
            continue;
        };

        current_section = current_op.binary(current_section, number);
    }
    total += current_section;
    total
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
