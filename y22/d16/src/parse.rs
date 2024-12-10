use advent_utils::nom::{
    branch::alt,
    bytes::complete::tag,
    character::{self, complete::alpha1},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

use super::valve::Valve;

pub fn parse(file_content: &str) -> Vec<Valve> {
    file_content
        .lines()
        .flat_map(|line| parse_valve(line))
        .map(|(_, b)| b)
        .collect()
}

fn parse_valve(line: &str) -> IResult<&str, Valve> {
    map(
        tuple((parse_name, parse_rate, parse_paths)),
        |(name, rate, paths)| Valve { rate, paths, name },
    )(line)
}

fn parse_name(line: &str) -> IResult<&str, usize> {
    preceded(tag("Valve "), parse_id)(line)
}

pub fn parse_id(line: &str) -> IResult<&str, usize> {
    let (input, letters) = alpha1(line)?;
    let mut res: usize = 0;
    for ch in letters.as_bytes().iter() {
        res <<= 8;
        res |= (*ch) as usize;
    }
    Ok((input, res))
}

fn parse_rate(input: &str) -> IResult<&str, u16> {
    preceded(tag(" has flow rate="), character::complete::u16)(input)
}
fn parse_paths(input: &str) -> IResult<&str, Vec<usize>> {
    let parse_list = separated_list1(tag(", "), parse_id);
    preceded(
        alt((
            tag("; tunnel leads to valve "),
            tag("; tunnels lead to valves "),
        )),
        parse_list,
    )(input)
}
