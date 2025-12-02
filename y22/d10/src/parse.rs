use super::command::Command;
use advent_utils::nom;
use nom::{IResult, Parser};

fn parse_command(line: &str) -> IResult<&str, Command> {
    nom::branch::alt((
        nom::combinator::map(nom::bytes::complete::tag("noop"), |_| Command::Noop),
        nom::combinator::map(
            nom::sequence::preceded(
                nom::bytes::complete::tag("addx "),
                nom::character::complete::i32,
            ),
            Command::Addx,
        ),
    ))
    .parse(line)
}

pub fn parse_commands(input: &str) -> impl Iterator<Item = Command> + '_ {
    input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| match parse_command(line) {
            Ok((_, command)) => command,
            Err(err) => {
                panic!("Failed to parse: '{line}' error: {err}")
            }
        })
}
