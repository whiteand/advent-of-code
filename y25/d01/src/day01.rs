#[derive(Debug, Copy, Clone)]
enum Instruction {
    TurnLeft(usize),
    TurnRight(usize),
}

#[derive(Debug, Copy, Clone)]
struct Ciferblat(usize);

impl Ciferblat {
    fn perform(&mut self, instruction: Instruction) -> usize {
        match instruction {
            Instruction::TurnLeft(steps) => self.perform_left_turn(steps),
            Instruction::TurnRight(steps) => self.perform_right_turn(steps),
        }
    }
    fn perform_left_turn(&mut self, steps: usize) -> usize {
        let skipped_zeros = steps / CIFERBLAT_SIZE;
        let steps = steps % CIFERBLAT_SIZE;
        let additional_zero = (steps >= self.0 && self.0 != 0) as usize;
        self.0 = (self.0 + CIFERBLAT_SIZE - steps) % CIFERBLAT_SIZE;
        skipped_zeros + additional_zero
    }
    fn perform_right_turn(&mut self, steps: usize) -> usize {
        let skipped_zeros = steps / CIFERBLAT_SIZE;
        let steps = steps % CIFERBLAT_SIZE;
        let additional_zero = (steps + self.0 >= CIFERBLAT_SIZE && self.0 != 0) as usize;
        self.0 = (self.0 + steps) % CIFERBLAT_SIZE;
        skipped_zeros + additional_zero
    }
}

fn parse_instruction(line: &str) -> Instruction {
    let first = line.as_bytes()[0];
    let remaining = &line[1..];
    let steps: usize = remaining.parse().expect("valid number");
    match first {
        b'L' => Instruction::TurnLeft(steps),
        b'R' => Instruction::TurnRight(steps),
        _ => unreachable!(),
    }
}

fn parse_instructions(file_content: &str) -> impl Iterator<Item = Instruction> {
    file_content.lines().map(parse_instruction)
}

const CIFERBLAT_SIZE: usize = 100;
fn solve(file_content: &str, cb: impl Fn(&mut Ciferblat, Instruction) -> usize) -> usize {
    parse_instructions(file_content)
        .scan(Ciferblat(50), |s, instruction| Some(cb(s, instruction)))
        .sum()
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    solve(file_content, |s, instruction| {
        s.perform(instruction);

        (s.0 == 0) as usize
    })
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    solve(file_content, |s, instruction| s.perform(instruction))
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "3")]
    #[case::actual(ACTUAL, "1147")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "6")]
    #[case::actual(ACTUAL, "6789")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
