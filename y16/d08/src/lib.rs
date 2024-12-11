use std::collections::HashSet;

use advent_utils::{
    glam::UVec2,
    nom::{
        self,
        branch::alt,
        bytes::complete::tag,
        character::complete::{self, line_ending},
        multi::separated_list1,
        sequence::{preceded, separated_pair},
        Parser,
    },
};
use itertools::Itertools;

const SCREEN_SIZE: UVec2 = UVec2::new(50, 6);
#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, UVec2::new(50, 6))
}
fn solve(file_content: &str, screen_size: UVec2) -> usize {
    let (_, commands) = parse_instructions(file_content).unwrap();
    let mut screen = HashSet::new();
    for command in commands {
        match command {
            Instruction::Rect(x, y) => {
                assert!(x <= screen_size.x);
                assert!(y <= screen_size.y);
                for (x, y) in (0..x).cartesian_product(0..y) {
                    screen.insert(UVec2::new(x, y));
                }
            }
            _ => {}
        }
    }
    print_screen(&screen, screen_size);
    0
}

fn print_screen(screen: &HashSet<UVec2>, screen_size: UVec2) {
    for r in 0..screen_size.y {
        for c in 0..screen_size.x {
            let v = UVec2::new(c, r);
            if screen.contains(&v) {
                print!("#")
            } else {
                print!(".")
            }
        }
        println!();
    }
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    todo!("part 2 is not implemented yet: {file_content}")
}

#[derive(Debug)]
enum Instruction {
    Rect(u32, u32),
    RotateColumn { col: u32, step: u32 },
    RotateRow { row: u32, step: u32 },
}

fn parse_instructions(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((
        preceded(
            tag("rect "),
            separated_pair(complete::u32, tag("x"), complete::u32),
        )
        .map(|x| Instruction::Rect(x.0, x.1)),
        preceded(
            tag("rotate column "),
            separated_pair(
                preceded(tag("x="), complete::u32),
                tag(" by "),
                complete::u32,
            ),
        )
        .map(|x| Instruction::RotateColumn {
            col: x.0,
            step: x.1,
        }),
        preceded(
            tag("rotate row "),
            separated_pair(
                preceded(tag("y="), complete::u32),
                tag(" by "),
                complete::u32,
            ),
        )
        .map(|x| Instruction::RotateRow {
            row: x.0,
            step: x.1,
        }),
    ))(input)
}

#[cfg(test)]
mod tests {
    use advent_utils::glam::UVec2;

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
        assert_eq!(format!("{}", super::solve(EXAMPLE, UVec2::new(7, 3))), "6");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
