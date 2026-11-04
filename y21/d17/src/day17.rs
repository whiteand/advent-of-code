use std::ops::RangeInclusive;

use advent_utils::{
    glam::IVec2,
    nom::{self, Parser},
};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Area {
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
}

impl Area {
    fn positions(&self) -> impl Iterator<Item = IVec2> {
        let y_range = self.y_range.clone();
        let x_range = self.x_range.clone();

        y_range.flat_map(move |y| x_range.clone().map(move |x| IVec2::new(x, y)))
    }
}

fn parse_area(input: &str) -> nom::IResult<&str, Area> {
    let (input, _) = nom::bytes::tag("target area: x=").parse(input)?;
    let (input, x_start) = nom::character::complete::i32.parse(input)?;
    let (input, _) = nom::bytes::tag("..").parse(input)?;
    let (input, x_end) = nom::character::complete::i32.parse(input)?;
    let x_range = x_start..=x_end;
    let (input, _) = nom::bytes::tag(", y=").parse(input)?;
    let (input, y_start) = nom::character::complete::i32.parse(input)?;
    let (input, _) = nom::bytes::tag("..").parse(input)?;
    let (input, y_end) = nom::character::complete::i32.parse(input)?;
    let y_range = y_start..=y_end;
    let (_, _) = nom::combinator::eof.parse(input)?;

    Ok(("", Area { x_range, y_range }))
}

fn get_vx(px: i32, t: i32) -> impl Iterator<Item = i32> {
    ((2 * px + t * (t - 1)) % (2 * t) == 0)
        .then(move || (2 * px + t * (t - 1)) / (2 * t))
        .filter(|vx| t < *vx)
        .into_iter()
        .chain(
            divisors(px * 2)
                .filter(move |pvx| t >= *pvx && 2 * px == *pvx * (2 * *pvx - (*pvx - 1))),
        )
}

fn divisors(n: i32) -> impl Iterator<Item = i32> {
    (1..=n.abs()).filter(move |x| n % x == 0)
}

fn get_vy0(py: i32, t: i32) -> Option<i32> {
    let top = 2 * py / t + t - 1;
    if top % 2 != 0 { None } else { Some(top / 2) }
}

fn get_initial_velocities(p: IVec2) -> impl Iterator<Item = IVec2> {
    divisors(2 * p.y).flat_map(move |t| {
        get_vy0(p.y, t)
            .into_iter()
            .flat_map(move |vy| get_vx(p.x, t).map(move |vx| IVec2::new(vx, vy)))
    })
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> i32 {
    let area = parse_area(file_content.trim()).map(|x| x.1).unwrap();

    area.positions()
        .flat_map(get_initial_velocities)
        .map(|v| v.y * (v.y + 1) / 2)
        .max()
        .unwrap_or_default()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let area = parse_area(file_content.trim()).map(|x| x.1).unwrap();

    area.positions()
        .flat_map(get_initial_velocities)
        .unique()
        .count()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "45")]
    #[case::actual(ACTUAL, "19503")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "112")]
    #[case::actual(ACTUAL, "5200")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
