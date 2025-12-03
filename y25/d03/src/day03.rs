use advent_utils::Array2d;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    solve(file_content, 2)
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    solve(file_content, 12)
}

fn solve(file_content: &str, combinations: usize) -> usize {
    let mut calculator = MaxJoltageCalculator {
        dp: Array2d::with_capacity(file_content.len()),
        digits_to_select: combinations,
    };
    file_content
        .lines()
        .map(|line| calculator.calculate(line.as_bytes()))
        .sum()
}

struct MaxJoltageCalculator {
    /// dp.get_copy(i, j) returns what is the largest possible number
    ///   - constructed by selecting `j` digits
    ///   - out of first `i` digits in a `row`
    ///   - (we will call those `i` digits "prefix")
    dp: Array2d<usize>,
    digits_to_select: usize,
}

impl MaxJoltageCalculator {
    pub fn calculate(&mut self, row: &[u8]) -> usize {
        let initial_digits_to_select = self.digits_to_select;

        let dp = &mut self.dp;

        // Reseting state
        dp.clear_and_resize(row.len() + 1, initial_digits_to_select + 1, usize::MAX);

        // If we want to select 0 digits
        // we will get `0` as a result
        dp.set_col(0, 0);

        // If we don't have digits to select from
        // we will get `0` as a result
        dp.get_row_mut(0).fill(0);

        // If we have only one digit
        // the digit itself is the maximum number.
        dp.get_row_mut(1).fill((row[0] - b'0') as usize);

        // If we want to select only one digit
        // the maximum possible result -
        // is the maximum digit in row[0..r]
        for prefix_len in 2..=row.len() {
            let last_digit_in_prefix = (row[prefix_len - 1] - b'0') as usize;
            let previous_max = dp.get_copy(prefix_len - 1, 1);
            dp.set(prefix_len, 1, previous_max.max(last_digit_in_prefix));
        }

        // If we want select more digits than we have
        // we will select all digits.
        for r in 2..=row.len().min(initial_digits_to_select) {
            for c in r..=initial_digits_to_select {
                let last_digit_in_prefix = (row[r - 1] - b'0') as usize;
                let all_previous_digits = dp.get_copy(r - 1, c - 1);
                let res = all_previous_digits * 10 + last_digit_in_prefix;
                dp.set(r, c, res);
            }
        }
        // if we have more digits
        // then we want to select
        for r in 3..=row.len() {
            for c in 2..(r.min(initial_digits_to_select + 1)) {
                // We get the max result excluding the last digit
                let max_result_without = dp.get_copy(r - 1, c);

                // We calculate the result if we will include the last digit
                let max_shorter_result = dp.get_copy(r - 1, c - 1);
                let last_digit_in_prefix = (row[r - 1] - b'0') as usize;
                let max_result_with = max_shorter_result * 10 + last_digit_in_prefix;

                // and select the maximum
                dp.set(r, c, max_result_with.max(max_result_without))
            }
        }

        dp.get_copy(row.len(), initial_digits_to_select)
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "357")]
    #[case::actual(ACTUAL, "17359")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "3121910778619")]
    #[case::actual(ACTUAL, "172787336861064")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
