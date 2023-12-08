use std::{fmt::Write, ops::Deref, str::FromStr};

use nom::{
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

impl Into<usize> for Position {
    fn into(self) -> usize {
        self.0
    }
}

impl Position {
    const START: Self = Self(0);
    const END: Self = Self(25 * 26 * 26 + 25 * 26 + 25);
    fn new() -> Self {
        Self::START
    }
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

pub fn solve_task1(file_content: &str) -> usize {
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

#[derive(Debug, Clone, PartialEq, Eq)]
struct Ending {
    cycle: usize,
    ending_position: Position,
    turn_index: usize,
    id: usize,
}
#[derive(Debug)]
struct PositionInfo {
    loop_start_cycle: usize,
    loop_reset_cycle: usize,
    endings: Vec<Ending>,
}

struct EndingIterator {
    endings: Vec<Ending>,
    loop_start_ending_index: usize,
    cycles_per_loop: usize,
    index: usize,
    loops: usize,
    cycle: usize,
    turn_index: usize,
}

impl EndingIterator {
    fn time(&self, turns_per_loop: usize) -> usize {
        self.cycle * turns_per_loop + self.turn_index
    }
}
impl Iterator for EndingIterator {
    type Item = Ending;

    fn next(&mut self) -> Option<Self::Item> {
        let ind = self.index;

        let mut ending = self.endings[ind].clone();
        ending.cycle += self.loops * self.cycles_per_loop;

        self.index += 1;
        if self.index >= self.endings.len() {
            self.loops += 1;
            self.index = self.loop_start_ending_index;
        };

        self.cycle = ending.cycle;
        self.turn_index = ending.turn_index;

        Some(ending)
    }
}

impl IntoIterator for PositionInfo {
    type Item = Ending;
    type IntoIter = EndingIterator;

    fn into_iter(self) -> Self::IntoIter {
        let loop_start_ending_index = self
            .endings
            .iter()
            .position(|e| e.cycle >= self.loop_start_cycle)
            .unwrap();

        EndingIterator {
            endings: self.endings,
            cycles_per_loop: self.loop_reset_cycle - self.loop_start_cycle,
            index: 0,
            loops: 0,
            cycle: 0,
            turn_index: 0,
            loop_start_ending_index,
        }
    }
}

fn get_position_info(
    turns: &[Direction],
    network: &[(Position, Position)],
    start: Position,
) -> PositionInfo {
    let mut pos = start;
    let mut cycle = 0;
    let mut cycle_finish_positions = vec![start];
    let mut endings = Vec::new();
    let loop_start_cycle;
    let loop_reset_cycle;
    loop {
        for (i, &t) in turns.iter().enumerate() {
            pos.turn(t, network);
            if pos.0 % 26 == 25 {
                endings.push(Ending {
                    id: endings.len(),
                    cycle,
                    ending_position: pos,
                    turn_index: i,
                });
            }
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
        endings,
    }
}

pub fn solve_task2(file_content: &str) -> usize {
    let (turns, nodes) = parse_input(file_content);
    let turns_per_cycle = turns.len();
    println!("Turns: {} ", turns_per_cycle);

    let mut network = vec![(Position::START, Position::START); 26 * 26 * 26];
    for node in &nodes {
        network[node.0 .0] = (node.1, node.2);
    }

    let position_infos = nodes
        .into_iter()
        .map(|(n, _, _)| n)
        .filter(|n| n.0 % 26 == 0)
        .map(|p| get_position_info(&turns, &network, p))
        .collect::<Vec<_>>();

    let mut carets = position_infos
        .into_iter()
        .map(|p| p.into_iter())
        .collect::<Vec<_>>();

    for c in &mut carets {
        c.next().unwrap();
    }

    let mut i = 0;

    loop {
        let mut min_time = usize::MAX;
        let mut max_time = 0;
        let mut min_time_index = 0;
        for (i, c) in carets.iter().enumerate() {
            let t = c.time(turns_per_cycle);
            if t < min_time {
                min_time = t;
                min_time_index = i;
            }
            if t > max_time {
                max_time = t
            }
        }
        i += 1;
        if i % 10000 == 0 {
            println!("{}: {}", i, max_time - min_time);
        }
        if min_time == max_time {
            return min_time + 1;
        } else {
            carets[min_time_index].next().unwrap();
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d08/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d08.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "2");
        assert_eq!(
            format!(
                "{}",
                solve_task1(
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
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "16043");
    }

    #[test]
    fn test_task2() {
        // KKA -> KKB -> KKZ -> KKB -> KKZ
        // | 0           | 1           | 2
        assert_eq!(
            format!(
                "{}",
                solve_task2(
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
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
