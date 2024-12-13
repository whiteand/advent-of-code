use advent_utils::{
    glam::U64Vec2,
    math,
    parse::{nums, ParseNumsIter},
};
use itertools::Itertools;

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
        .filter_map(|triple_of_lines| {
            parse_machine(triple_of_lines)
                .ok()
                .and_then(|mut machine| machine.move_prize(prize_offset).minimal_tokens_to_win())
        })
        .sum()
}

fn parse_machine(triple_lines: &str) -> Result<Machine, &str> {
    let mut nums = nums::<u64>(triple_lines);
    // Button A: X+21, Y+40
    // Button B: X+56, Y+21
    // Prize: X=15390, Y=2402
    nums.strip_prefix("Button A: X+")
        .map_err(|x| x.rest_str())?;
    let (ax, ay) = nums.next_tuple().ok_or("failed")?;
    nums.strip_prefix("\nButton B: X+")
        .map_err(|x| x.rest_str())?;
    let (bx, by) = nums.next_tuple().ok_or("failed")?;
    nums.strip_prefix("\nPrize: X=").map_err(|x| x.rest_str())?;
    let (tx, ty) = nums.next_tuple().ok_or("failed")?;

    Ok(Machine {
        button_a: U64Vec2::new(ax, ay),
        button_b: U64Vec2::new(bx, by),
        prize: U64Vec2::new(tx, ty),
    })
}

#[derive(Debug)]
struct Machine {
    button_a: U64Vec2,
    button_b: U64Vec2,
    prize: U64Vec2,
}

impl Machine {
    fn minimal_tokens_to_win(&self) -> Option<usize> {
        let U64Vec2 { x: ax, y: ay } = self.button_a;
        let U64Vec2 { x: bx, y: by } = self.button_b;
        let U64Vec2 { x: tx, y: ty } = self.prize;

        math::solve_system(
            [[ax.into(), bx.into()], [ay.into(), by.into()]],
            [tx.into(), ty.into()],
        )
        .and_then(|res| {
            res.into_iter()
                .filter_map(|x| usize::try_from(x).ok())
                .collect_tuple()
        })
        .map(|(a, b)| a * 3 + b)
    }

    fn move_prize(&mut self, prize_offset: U64Vec2) -> &mut Self {
        self.prize += prize_offset;
        self
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
