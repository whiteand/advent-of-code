use nom::{self, IResult};

use super::moves::{Direction, Move};

fn try_parse_move(line: &str) -> IResult<&str, Move> {
    nom::combinator::map(
        nom::sequence::separated_pair(
            nom::bytes::complete::is_a("LRUD"),
            nom::character::complete::space1,
            nom::character::complete::u32,
        ),
        |(a, distance): (&str, u32)| match a {
            "L" => Move {
                direction: Direction::Left,
                distance,
            },
            "U" => Move {
                direction: Direction::Up,
                distance,
            },
            "R" => Move {
                direction: Direction::Right,
                distance,
            },
            "D" => Move {
                direction: Direction::Down,
                distance,
            },
            _ => unreachable!(),
        },
    )(line)
}

pub fn parse_moves(input: &str) -> impl Iterator<Item = Move> + '_ {
    input
        .lines()
        .map(|line| try_parse_move(line).map(|x| x.1).unwrap())
}
