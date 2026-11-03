use std::ops::RangeInclusive;

use advent_utils::{
    glam::IVec2,
    grid::Grid,
    nom::{self, Parser},
};
use itertools::Itertools;

// target area: x=20..30, y=-10..-5

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

    fn contains(&self, p: IVec2) -> bool {
        self.x_range.contains(&p.x) && self.y_range.contains(&p.y)
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

fn get_position_x_at(vx0: i32, t: i32) -> i32 {
    if t >= vx0 {
        vx0 * vx0 - vx0 * (vx0 - 1) / 2
        // x = vx0 * vx0 - vx0 * (vx0 - 1) / 2
        // 2x = 2vx0 * vx0 - vx0 * (vx0 - 1)
        // 2x = vx0 * (2 * vx0 - (vx0 - 1))
    } else {
        t * vx0 - t * (t - 1) / 2
    }
}

fn get_vx(px: i32, t: i32) -> impl Iterator<Item = i32> {
    let mut res = Vec::new();

    if (2 * px + t * (t - 1)) % (2 * t) == 0 {
        let vx = (2 * px + t * (t - 1)) / (2 * t);
        if t < vx {
            res.push(vx)
        }
    }
    for pvx in divisors(px * 2) {
        if t < pvx {
            continue;
        }
        if 2 * px == pvx * (2 * pvx - (pvx - 1)) {
            res.push(pvx)
        }
    }

    return res.into_iter();
}

fn get_position_y_at(vy0: i32, t: i32) -> i32 {
    t * vy0 - t * (t - 1) / 2
    // 2y = 2 * t * vy0 - t * (t - 1)
    // 2y = t * (2 * vy0 - (t - 1)) =>   2y is divisible by t
    // 2y / t + t - 1 = 2 * vy0 =>       2y / t + t - 1 is divisible by 2

    // if t is even:
    // 2y / t = 2m + 1
    // 2y = 2m * t + t
    // if t is odd:
    // 2y / t = 2m

    // 2y / t = 2m + 1
    // t = 2k
    // or
    // 2y / t = 2m
    // t = 2k + 1
    //
    //
}

fn get_position_at(vx0: i32, vy0: i32, t: i32) -> IVec2 {
    IVec2 {
        x: get_position_x_at(vx0, t),
        y: get_position_y_at(vy0, t),
    }
}

fn iterate_trajectory(vx0: i32, vy0: i32) -> impl Iterator<Item = IVec2> {
    return (0..).map(move |t| get_position_at(vx0, vy0, t));
}

struct Display {
    origin: IVec2,
    dx: IVec2,
    dy: IVec2,
    grid: Grid<u8>,
}

impl Display {
    fn new(origin: IVec2, size: IVec2) -> Self {
        Self {
            origin,
            dx: IVec2::X,
            dy: IVec2::NEG_Y,
            grid: Grid::new(size, b'.'),
        }
    }

    fn set(&mut self, p: IVec2, value: u8) {
        let dp = p.x * self.dx + p.y * self.dy + self.origin;
        if let Some(r) = self.grid.get_mut(dp) {
            *r = value;
        }
    }
}

impl std::fmt::Display for Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.grid.render_ascii())
    }
}

fn divisors(n: i32) -> impl Iterator<Item = i32> {
    (1..=n.abs()).filter(move |x| n % x == 0)
}

fn get_vy0(py: i32, t: i32) -> Option<i32> {
    let top = 2 * py / t + t - 1;
    if top % 2 != 0 { None } else { Some(top / 2) }
}

fn get_max_y(vy: i32) -> i32 {
    vy * (vy + 1) / 2
}

#[tracing::instrument()]
fn get_initial_velocities(p: IVec2) -> impl Iterator<Item = IVec2> {
    divisors(2 * p.y)
        .flat_map(move |t| {
            tracing::info!("Found t = {t}");
            get_vy0(p.y, t).into_iter().flat_map(move |vy| {
                tracing::info!("Calculated vy={t}");

                get_vx(p.x, t)
                    .map(move |vx| IVec2::new(vx, vy))
                    .filter(move |v| {
                        assert_eq!(get_position_at(v.x, v.y, t), p);
                        true
                    })
            })
        })
        .collect_vec()
        .into_iter()
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> i32 {
    let area = parse_area(file_content.trim()).map(|x| x.1).unwrap();

    area.positions()
        .map(|p| {
            get_initial_velocities(p)
                .map(|v| get_max_y(v.y))
                .max()
                .unwrap_or(i32::MIN)
        })
        .max()
        .unwrap_or_default()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let area = parse_area(file_content.trim()).map(|x| x.1).unwrap();

    area.positions()
        .flat_map(|p| get_initial_velocities(p))
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
    #[case::actual(ACTUAL, "0")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
