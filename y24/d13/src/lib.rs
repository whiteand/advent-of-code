use advent_utils::{
    glam::U64Vec2,
    math,
    nom::{
        self,
        bytes::complete::tag,
        character::complete::{self, line_ending},
        sequence::{preceded, tuple},
        Parser,
    },
};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, U64Vec2::ZERO)
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, U64Vec2::splat(10000000000000))
}
pub fn solve(file_content: &str, prize_offset: U64Vec2) -> usize {
    file_content
        .split("\n\n")
        .flat_map(|line| parse_machine(line).ok().map(|x| x.1))
        .map(|machine| Machine {
            prize: machine.prize + prize_offset,
            ..machine
        })
        .map(|machine| machine.minimal_tokens_to_win().unwrap_or_default())
        .sum()
}

fn parse_machine(input: &str) -> nom::IResult<&str, Machine> {
    tuple((
        preceded(tag("Button A: "), parse_offset),
        line_ending,
        preceded(tag("Button B: "), parse_offset),
        line_ending,
        preceded(tag("Prize: "), parse_coord),
    ))
    .map(|(button_a, _, button_b, _, prize)| Machine {
        button_a,
        button_b,
        prize,
    })
    .parse(input)
}
fn parse_offset(input: &str) -> nom::IResult<&str, U64Vec2> {
    tuple((tag("X+"), complete::u64, tag(", Y+"), complete::u64))
        .map(|(_, x, _, y)| U64Vec2::new(x, y))
        .parse(input)
}
fn parse_coord(input: &str) -> nom::IResult<&str, U64Vec2> {
    tuple((tag("X="), complete::u64, tag(", Y="), complete::u64))
        .map(|(_, x, _, y)| U64Vec2::new(x, y))
        .parse(input)
}
#[derive(Debug)]
struct Machine {
    button_a: U64Vec2,
    button_b: U64Vec2,
    prize: U64Vec2,
}

impl Machine {
    fn minimal_tokens_to_win(&self) -> Option<usize> {
        let [a, b] = math::solve_system(
            [
                [self.button_a.x.into(), self.button_b.x.into()],
                [self.button_a.y.into(), self.button_b.y.into()],
            ],
            [self.prize.x.into(), self.prize.y.into()],
        )?;

        (a.bottom == 1 && b.bottom == 1 && a.top >= 0 && b.top >= 0)
            .then(|| (a.top * 3 + b.top) as usize)
    }
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "480");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "32041");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "875318608908");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "95843948914827");
    }
}
