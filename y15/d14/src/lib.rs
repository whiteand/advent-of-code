use itertools::Itertools;
use nom::Parser;

const TIME: usize = 2503;
pub fn solve_part_1(file_content: &str) -> usize {
    let horses = parse_horses(file_content).map(|(_, x)| x).unwrap();
    get_winner_distance_at(&horses, TIME)
}

fn get_winner_distance_at(horses: &[Horse], time: usize) -> usize {
    horses
        .iter()
        .map(|horse| horse.get_distance_at(time))
        .max()
        .unwrap_or_default()
}

pub fn solve_part_2(file_content: &str) -> usize {
    let horses = parse_horses(file_content).map(|(_, x)| x).unwrap();
    get_winner_score_at(&horses, TIME)
}
fn get_winner_score_at(horses: &[Horse], time: usize) -> usize {
    let scores = get_scores_at(horses, time);
    scores.into_iter().max().unwrap_or_default()
}

fn get_scores_at(horses: &[Horse<'_>], time: usize) -> Vec<usize> {
    let mut distances = horses.iter().map(|_| 0).collect_vec();
    let mut scores = horses.iter().map(|_| 0).collect_vec();
    for t in 1..=time {
        for (horse, distance) in horses.iter().zip(distances.iter_mut()) {
            *distance = horse.get_distance_at(t);
        }
        let winner_distance = distances.iter().copied().max().unwrap_or_default();
        for winner_index in distances
            .iter()
            .enumerate()
            .filter(|(_, d)| **d == winner_distance)
            .map(|(i, _)| i)
        {
            scores[winner_index] += 1;
        }
    }
    scores
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Horse<'i> {
    name: &'i str,
    /// in km/s
    velocity: usize,
    /// in seconds
    duration: usize,
    /// in seconds
    rest: usize,
}

impl<'i> Horse<'i> {
    fn get_distance_at(&self, mut time: usize) -> usize {
        let cycle = self.duration + self.rest;
        let cycle_len = self.velocity * self.duration;
        let mut total = 0usize;
        if time > cycle {
            let full_cycles = time / cycle;
            time -= full_cycles * cycle;
            total += full_cycles * cycle_len;
        }
        total + time.min(self.duration) * self.velocity
    }
}

fn parse_horses(file_content: &str) -> nom::IResult<&str, Vec<Horse<'_>>> {
    nom::multi::separated_list1(nom::character::complete::newline, parse_horse).parse(file_content)
}

// \w+ can fly \d+ km/s for \d+ seconds, but then must rest for \d+ seconds.
fn parse_horse(line: &str) -> nom::IResult<&str, Horse<'_>> {
    let mut parse_usize = nom::character::complete::u64.map(|x| x as usize);

    let (input, name) = nom::character::complete::alpha1.parse(line)?;
    let (input, _) = nom::bytes::complete::tag(" can fly ").parse(input)?;
    let (input, velocity) = parse_usize.parse(input)?;
    let (input, _) = nom::bytes::complete::tag(" km/s for ").parse(input)?;
    let (input, duration) = parse_usize.parse(input)?;
    let (input, _) = nom::bytes::complete::tag(" seconds, but then must rest for ").parse(input)?;
    let (input, rest) = parse_usize.parse(input)?;
    let (input, _) = nom::bytes::complete::tag(" seconds.").parse(input)?;

    Ok((
        input,
        Horse {
            name,
            velocity,
            duration,
            rest,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let horses = super::parse_horses(EXAMPLE).map(|x| x.1).unwrap();
        assert_eq!(super::get_winner_distance_at(&horses, 1000), 1120);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "2696");
    }

    #[test]
    fn test_part2() {
        let horses = super::parse_horses(EXAMPLE).map(|x| x.1).unwrap();
        assert_eq!(super::get_scores_at(&horses, 1000), vec![312, 689]);
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1084");
    }
}
