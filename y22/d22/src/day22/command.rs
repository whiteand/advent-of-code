use advent_utils::nom::{self, IResult, Parser};

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub enum Command {
    Move(usize),
    TurnRight,
    TurnLeft,
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Move(arg0) => write!(f, "{arg0}"),
            Self::TurnRight => write!(f, "R"),
            Self::TurnLeft => write!(f, "L"),
        }
    }
}
impl Command {
    pub fn parse(input: &str) -> IResult<&str, Command> {
        nom::branch::alt((
            nom::bytes::complete::tag("R").map(|_| Command::TurnRight),
            nom::bytes::complete::tag("L").map(|_| Command::TurnLeft),
            nom::parse_usize.map(Command::Move),
        ))
        .parse(input)
    }
}
