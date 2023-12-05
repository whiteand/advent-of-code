use std::ops::RangeInclusive;
use std::{cmp::Ordering::*, collections::BTreeSet};

use nom::{
    bytes::complete::tag,
    character,
    combinator::map,
    sequence::{preceded, separated_pair},
    IResult,
};

#[derive(Eq, PartialEq, Debug, Clone)]
struct Measurement {
    sensor: (i32, i32),
    beacon: (i32, i32),
}

impl Measurement {
    fn distance(&self) -> i32 {
        let Measurement {
            sensor: (sx, sy),
            beacon: (bx, by),
        } = self;

        let mut distance = 0;

        if *sx > *bx {
            distance += *sx - *bx
        } else {
            distance += *bx - *sx
        }

        if *sy > *by {
            distance += *sy - *by
        } else {
            distance += *by - *sy
        }

        distance
    }

    fn restricted_range(&self, y: i32) -> Option<RangeInclusive<i32>> {
        let measurement_distance = self.distance();
        let distance_to_y = (self.sensor.1 - y).abs();

        if distance_to_y > measurement_distance {
            None
        } else if distance_to_y == measurement_distance {
            if self.beacon.0 == self.sensor.0 && self.beacon.1 == y {
                None
            } else {
                Some(self.sensor.0..=self.sensor.0)
            }
        } else if self.beacon.1 == y {
            let start = self.sensor.0 - measurement_distance + distance_to_y;
            let end = self.sensor.0 + measurement_distance - distance_to_y;
            if start == self.beacon.0 {
                Some((start + 1)..=end)
            } else if end == self.beacon.0 {
                Some(start..=(end - 1))
            } else {
                unreachable!()
            }
        } else {
            let start = self.sensor.0 - measurement_distance + distance_to_y;
            let end = self.sensor.0 + measurement_distance - distance_to_y;
            Some(start..=end)
        }
    }
}

pub fn solve_task1(file_content: &str, row: i32) -> usize {
    let measurements: Vec<_> = parse_measurements(file_content).collect();
    let restricted_ranges = get_restricted_ranges(&measurements, row);
    restricted_ranges.into_iter().map(|r| r.count()).sum()
}

pub fn solve_task2(
    file_content: &str,
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
) -> u64 {
    let measurements: Vec<_> = parse_measurements(file_content).collect();
    if let Some((x, y)) = find_beacon(&measurements, x_range, y_range) {
        (x as u64) * 4000000 + (y as u64)
    } else {
        0
    }
}
fn find_beacon(
    measurements: &[Measurement],
    x_range: RangeInclusive<i32>,
    y_range: RangeInclusive<i32>,
) -> Option<(i32, i32)> {
    let beacons = measurements
        .iter()
        .map(|m| m.beacon)
        .collect::<BTreeSet<_>>();

    for y in y_range {
        let restricted_ranges = get_restricted_ranges(measurements, y)
            .iter()
            .flat_map(|r| r.intersection_with(&x_range))
            .collect::<Vec<_>>();
        let mut x = *x_range.start();
        let mut current_range_index = 0;
        let max_x = *x_range.end();
        while x <= max_x {
            let p = (x, y);
            if beacons.contains(&p) {
                x += 1;
                continue;
            }
            if restricted_ranges
                .get(current_range_index)
                .map(|r| r.contains(&x))
                .unwrap_or(false)
            {
                x = *restricted_ranges[current_range_index].end() + 1;
                current_range_index += 1;
                continue;
            }
            return Some(p);
        }
    }
    None
}

fn get_restricted_ranges(measurements: &[Measurement], row: i32) -> Vec<RangeInclusive<i32>> {
    let mut not_intersecting_restricted_ranges: Vec<_> = measurements
        .iter()
        .filter_map(|m| m.restricted_range(row))
        .collect();

    not_intersecting_restricted_ranges.sort_by(|a, b| {
        let a_start = *a.start();
        let b_start = *b.start();

        match a_start.cmp(&b_start) {
            Equal => a.len().cmp(&b.len()),
            x => x,
        }
    });

    let mut merged_ranges = Vec::new();
    for range in not_intersecting_restricted_ranges {
        let last_merged = merged_ranges.pop();
        match last_merged {
            None => {
                merged_ranges.push(range);
            }
            Some(last_merged) => match last_merged.merge_with(&range) {
                Some(new_range) => merged_ranges.push(new_range),
                None => {
                    merged_ranges.push(last_merged);
                    merged_ranges.push(range);
                }
            },
        }
    }
    merged_ranges
}

fn parse_measurements(file_content: &str) -> impl Iterator<Item = Measurement> + '_ {
    file_content
        .lines()
        .map(|line| parse_measurement(line).unwrap().1)
}
fn parse_measurement(input: &str) -> IResult<&str, Measurement> {
    let mut parse_measurement = map(
        separated_pair(
            preceded(tag("Sensor at "), parse_point),
            tag(": closest beacon is at "),
            parse_point,
        ),
        |(sensor, beacon)| Measurement { sensor, beacon },
    );
    parse_measurement(input)
}
fn parse_point(input: &str) -> IResult<&str, (i32, i32)> {
    let x = preceded(tag("x="), character::complete::i32);
    let y = preceded(tag("y="), character::complete::i32);
    let mut parse_point = separated_pair(x, tag(", "), y);
    parse_point(input)
}

trait RangeOperations: Sized {
    fn merge_with(&self, other: &Self) -> Option<Self>;
    fn intersection_with(&self, other: &Self) -> Option<Self>;
    fn len(&self) -> usize;
}

impl RangeOperations for RangeInclusive<i32> {
    fn intersection_with(&self, other: &Self) -> Option<Self> {
        if self.start().gt(other.end()) || self.end().lt(other.start()) {
            None
        } else {
            let start = *self.start().max(other.start());
            let end = *self.end().min(other.end());
            Some(start..=end)
        }
    }
    fn merge_with(&self, other: &Self) -> Option<Self> {
        if *other.start() == *self.end() + 1 {
            let start = *self.start();
            let end = *other.end();
            Some(start..=end)
        } else if *other.end() + 1 == *self.start() {
            let start = *other.start();
            let end = *self.end();
            Some(start..=end)
        } else if self.start().gt(other.end()) || self.end().lt(other.start()) {
            None
        } else {
            let start = (*self.start()).min(*other.start());
            let end = (*self.end()).max(*other.end());
            Some(start..=end)
        }
    }
    fn len(&self) -> usize {
        return (*self.end() - *self.start() + 1) as usize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y22d15/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y22/y22d15.txt");
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT, 10)), "26");
    }

    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL, 2000000)), "4907780");
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(
            format!("{}", solve_task2(INPUT, 0..=20, 0..=20)),
            "56000011"
        );
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(
            format!("{}", solve_task2(ACTUAL, 0..=4000000, 0..=4000000)),
            "13639962836448"
        );
    }
}
