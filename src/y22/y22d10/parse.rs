use super::command::Command;
use nom::IResult;

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
    ))(line)
}

pub fn parse_commands(input: &str) -> impl Iterator<Item = Command> + '_ {
    input.lines().map(|line| parse_command(line).unwrap().1)
}
