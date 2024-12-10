use std::collections::HashMap;

use advent_utils::nom::{
    bytes::complete::tag, character::complete, combinator, multi::separated_list1,
    sequence::separated_pair, IResult,
};
use itertools::Itertools;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    sets: Vec<HashMap<Color, usize>>,
    id: usize,
}

fn parse_color(line: &str) -> IResult<&str, Color> {
    advent_utils::nom::branch::alt((
        combinator::map(tag("red"), |_| Color::Red),
        combinator::map(tag("green"), |_| Color::Green),
        combinator::map(tag("blue"), |_| Color::Blue),
    ))(line)
}

fn game_parser(line: &str) -> IResult<&str, Game> {
    let (input, _) = tag("Game ")(line)?;
    let (input, id) = complete::u32(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, sets) = separated_list1(
        tag("; "),
        separated_list1(
            tag(", "),
            separated_pair(
                combinator::map(complete::u32, |v| v as usize),
                complete::space1,
                parse_color,
            ),
        ),
    )(input)?;

    let sets = sets
        .into_iter()
        .map(|set| {
            let mut res = HashMap::new();
            for (n, c) in set {
                *res.entry(c).or_insert(0) += n;
            }
            res
        })
        .collect_vec();

    Ok((
        input,
        Game {
            sets,
            id: id as usize,
        },
    ))
}

fn parse_game(line: &str) -> Game {
    let (_, game) = game_parser(line).expect("failed to parse game");
    game
}

static AVAILABLE: [(Color, usize); 3] = [(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)];

pub fn solve_part_1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(parse_game)
        .filter(|game| {
            AVAILABLE.iter().copied().all(|(c, n)| {
                game.sets
                    .iter()
                    .all(|p| p.get(&c).copied().unwrap_or_default() <= n)
            })
        })
        .map(|g| g.id)
        .sum()
}
fn get_game_power(g: Game) -> usize {
    let mut s: HashMap<Color, usize> = HashMap::new();

    for set in g.sets.into_iter() {
        for (c, n) in set {
            s.entry(c).and_modify(|f| *f = (*f).max(n)).or_insert(n);
        }
    }
    s.values().product()
}
pub fn solve_part_2(file_content: &str) -> usize {
    file_content
        .lines()
        .map(parse_game)
        .map(get_game_power)
        .sum::<usize>()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "8");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "2239");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "2286");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "83435");
    }
}
