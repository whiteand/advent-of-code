use super::model::ScannerMeasurements;
use advent_utils::{
    glam::IVec3,
    winnow::{self, prelude::*},
};

fn parse_ivec3(input: &mut &str) -> winnow::Result<IVec3> {
    (
        winnow::ascii::dec_int,
        ',',
        winnow::ascii::dec_int,
        ',',
        winnow::ascii::dec_int,
    )
        .map(|(x, _, y, _, z)| IVec3::new(x, y, z))
        .parse_next(input)
}

fn parse_scanner(input: &mut &str) -> winnow::Result<ScannerMeasurements> {
    (
        winnow::combinator::delimited("--- scanner ", winnow::ascii::dec_int, " ---"),
        winnow::ascii::line_ending,
        winnow::combinator::separated(1.., parse_ivec3, winnow::ascii::line_ending),
    )
        .map(|(id, _, measurements)| ScannerMeasurements::new(id, measurements))
        .parse_next(input)
}

pub fn parse_scanners(input: &mut &str) -> winnow::Result<Vec<ScannerMeasurements>> {
    winnow::combinator::separated(
        1..,
        parse_scanner,
        (winnow::ascii::line_ending, winnow::ascii::line_ending),
    )
    .parse_next(input)
}
