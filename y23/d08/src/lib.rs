use std::{fmt::Write, ops::Deref, str::FromStr};

use advent_utils::nom::{
    self,
    branch::alt,
    character::complete,
    character::complete::newline,
    multi::{many1, separated_list1},
    sequence::separated_pair,
    IResult, Parser,
};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Position(usize);

impl Deref for Position {
    fn deref(&self) -> &Self::Target {
        &self.0
    }

    type Target = usize;
}

impl Position {
    const START: Self = Self(0);
    const END: Self = Self(25 * 26 * 26 + 25 * 26 + 25);
    fn turn(&mut self, direction: Direction, network: &[(Position, Position)]) {
        self.0 = match direction {
            Direction::Left => network[self.0].0 .0,
            Direction::Right => network[self.0].1 .0,
        };
    }
}

impl FromStr for Position {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut id = 0;
        for c in s.chars() {
            id *= 26;
            id += (c as u8 - b'A') as usize;
        }
        Ok(Self(id))
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut chars = vec!['A'; 3];
        let mut i = 0;
        let mut id = self.0;
        while id > 0 {
            chars[i] = ((id % 26) as u8 + b'A') as char;
            id /= 26;
            i += 1;
        }

        for i in chars.into_iter().rev() {
            f.write_char(i)?;
        }
        write!(f, "({})", self.0)?;
        Ok(())
    }
}

fn node(input: &str) -> IResult<&str, (Position, Position, Position)> {
    nom::sequence::separated_pair(
        complete::alpha1.map(|s: &str| s.parse::<Position>().unwrap()),
        nom::bytes::complete::tag(" = "),
        nom::sequence::delimited(
            complete::char('('),
            nom::sequence::separated_pair(
                complete::alpha1.map(|s: &str| s.parse::<Position>().unwrap()),
                nom::bytes::complete::tag(", "),
                complete::alpha1.map(|s: &str| s.parse::<Position>().unwrap()),
            ),
            complete::char(')'),
        ),
    )
    .map(|(id, (left, right))| (id, left, right))
    .parse(input)
}
fn network(input: &str) -> IResult<&str, Vec<(Position, Position, Position)>> {
    let (input, nodes) = separated_list1(newline, node).parse(input)?;

    Ok((input, nodes))
}

fn parse_input(file_content: &str) -> (Vec<Direction>, Vec<(Position, Position, Position)>) {
    let (_, (directions, nodes)) = separated_pair(
        many1(alt((
            complete::char('L').map(|_| Direction::Left),
            complete::char('R').map(|_| Direction::Right),
        ))),
        nom::bytes::complete::tag("\n\n"),
        network,
    )(file_content)
    .unwrap();

    (directions, nodes)
}

pub fn solve_part_1(file_content: &str) -> usize {
    let (turns, nodes) = parse_input(file_content);
    let mut network = vec![(Position::START, Position::START); 26 * 26 * 26];
    for node in nodes {
        network[node.0 .0] = (node.1, node.2);
    }

    turns
        .into_iter()
        .cycle()
        .scan(Position::START, |pos, t| {
            pos.turn(t, &network);
            Some(*pos)
        })
        .take_while(|pos| *pos != Position::END)
        .count()
        + 1
}

#[derive(Debug)]
struct PositionInfo {
    loop_start_cycle: usize,
    loop_reset_cycle: usize,
}

fn get_position_info(
    turns: &[Direction],
    network: &[(Position, Position)],
    start: Position,
) -> PositionInfo {
    let mut pos = start;
    let mut cycle = 0;
    let mut cycle_finish_positions = vec![start];
    let loop_start_cycle;
    let loop_reset_cycle;
    loop {
        for &t in turns.iter() {
            pos.turn(t, network);
        }
        cycle += 1;
        if let Some(i) = cycle_finish_positions.iter().position(|&p| p == pos) {
            loop_start_cycle = i;
            loop_reset_cycle = cycle;
            break;
        }
        cycle_finish_positions.push(pos);
    }

    PositionInfo {
        loop_reset_cycle,
        loop_start_cycle,
    }
}

fn gcd(a: usize, b: usize) -> usize {
    let mut a = a;
    let mut b = b;
    while a > 0 && b > 0 {
        if a > b {
            a %= b;
        } else {
            b %= a
        }
    }
    a.max(b)
}
fn nok(a: usize, b: usize) -> usize {
    a / gcd(a, b) * b
}

pub fn solve_part_2(file_content: &str) -> usize {
    let (turns, nodes) = parse_input(file_content);
    let turns_per_cycle = turns.len();

    let mut network = vec![(Position::START, Position::START); 26 * 26 * 26];
    for node in &nodes {
        network[node.0 .0] = (node.1, node.2);
    }

    nodes
        .into_iter()
        .map(|(n, _, _)| n)
        .filter(|n| n.0 % 26 == 0)
        .map(|p| get_position_info(&turns, &network, p))
        .map(|p| (p.loop_reset_cycle - p.loop_start_cycle) * turns_per_cycle)
        .fold(1usize, nok)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "2");
        assert_eq!(
            format!(
                "{}",
                solve_part_1(
                    "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
                )
            ),
            "6"
        );
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "16043");
    }

    #[test]
    fn test_part_2() {
        // KKA -> KKB -> KKZ -> KKB -> KKZ
        // | 0           | 1           | 2
        assert_eq!(
            format!(
                "{}",
                solve_part_2(
                    "LR

KKA = (KKB, XXX)
KKB = (XXX, KKZ)
KKZ = (KKB, XXX)
FFA = (FFB, XXX)
FFB = (FFC, FFC)
FFC = (FFZ, FFZ)
FFZ = (FFB, FFB)
XXX = (XXX, XXX)"
                )
            ),
            "6"
        );
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "15726453850399");
    }
}
