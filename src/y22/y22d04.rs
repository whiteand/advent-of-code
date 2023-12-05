use std::str::FromStr;

use nom::IResult;

fn parse_next<'a, T, U>(x: &mut T) -> U
where
    T: Iterator<Item = &'a str>,
    U: FromStr,
    U::Err: std::fmt::Debug,
{
    x.next().unwrap().parse().unwrap()
}

struct Range {
    start: u32,
    end: u32,
}

impl Range {
    #[inline]
    fn fully_contains(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    #[inline]
    fn overlaps(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl FromStr for Range {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split('-');
        Ok(Range {
            start: parse_next(&mut iter),
            end: parse_next(&mut iter),
        })
    }
}

struct Pair(Range, Range);

impl Pair {
    #[inline]
    fn one_contains_other(&self) -> bool {
        self.0.fully_contains(&self.1) || self.1.fully_contains(&self.0)
    }

    #[inline]
    fn has_overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

impl FromStr for Pair {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.split(',');
        Ok(Self(parse_next(&mut iter), parse_next(&mut iter)))
    }
}

fn parse_range(line: &str) -> IResult<&str, Range> {
    let (input, start) = nom::character::complete::u32(line)?;
    let (input, _) = nom::bytes::complete::tag("-")(input)?;
    let (input, end) = nom::character::complete::u32(input)?;
    Ok((input, Range { start, end }))
}

fn parse_pair(line: &str) -> IResult<&str, Pair> {
    let (input, first) = parse_range(line)?;
    let (input, _) = nom::bytes::complete::tag(",")(input)?;
    let (input, second) = parse_range(input)?;

    Ok((input, Pair(first, second)))
}

fn solve(file_content: &str, predicate: impl Fn(&Pair) -> bool) -> usize {
    file_content
        .lines()
        .map(|line| parse_pair(line).unwrap().1)
        .filter(predicate)
        .count()
}

pub fn solve_task1(file_content: &str) -> usize {
    solve(file_content, Pair::one_contains_other)
}

pub fn solve_task2(file_content: &str) -> usize {
    solve(file_content, Pair::has_overlaps)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "2");
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "4");
    }
}
