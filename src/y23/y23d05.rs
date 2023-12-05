mod int_map;
mod map_pipeline;
mod range_map;

use int_map::IntMap;
use map_pipeline::MapPipeline;
use range_map::RangeMap;
use std::ops::Range;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete,
    character::complete::{newline, space1},
    combinator::{eof, map},
    multi::{separated_list0, separated_list1},
    sequence::{delimited, separated_pair, terminated, tuple},
    IResult,
};

pub fn solve_task1(file_content: &str) -> usize {
    let (_, (seeds, pipeline)) = task1_inputs(file_content).expect("failed to parse inputs");
    seeds
        .into_iter()
        .map(|s| pipeline.get(s))
        .min()
        .unwrap_or_default()
}

pub fn solve_task2(file_content: &str) -> usize {
    let (_, (seed_ranges, maps)) = task2_inputs(file_content).expect("failed to parse inputs");
    seed_ranges
        .into_iter()
        .flat_map(|r| maps.map_range(r))
        .min_by_key(|r| r.start)
        .map(|r| r.start)
        .expect("no location found")
}

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, (dst, src, length)) = tuple((
        (terminated(complete::u32, space1)),
        (terminated(complete::u32, space1)),
        complete::u32,
    ))(input)?;
    Ok((
        input,
        RangeMap::new(dst as usize, src as usize, length as usize),
    ))
}

fn int_map<'a, 'b>(prefix: &'a str, input: &'b str) -> IResult<&'b str, IntMap> {
    map(
        delimited(
            tuple((tag(prefix), newline)),
            separated_list0(newline, range_map),
            alt((map(tuple((newline, newline)), |_| ()), map(eof, |_| ()))),
        ),
        |ranges| IntMap::new(ranges),
    )(input)
}

fn maps(input: &str) -> IResult<&str, MapPipeline> {
    let (input, seed_to_soil) = int_map("seed-to-soil map:", input)?;
    let (input, soil_to_fertilizer) = int_map("soil-to-fertilizer map:", input)?;
    let (input, fertilizer_to_water) = int_map("fertilizer-to-water map:", input)?;
    let (input, water_to_light) = int_map("water-to-light map:", input)?;
    let (input, light_to_temperature) = int_map("light-to-temperature map:", input)?;
    let (input, temperature_to_humidity) = int_map("temperature-to-humidity map:", input)?;
    let (input, humidity_to_location) = int_map("humidity-to-location map:", input)?;
    Ok((
        input,
        MapPipeline::new(vec![
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        ]),
    ))
}
fn task1_inputs(file_content: &str) -> IResult<&str, (Vec<usize>, MapPipeline)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, map(complete::u32, |x| x as usize)),
        tuple((newline, newline)),
    )(file_content)?;

    let (input, maps) = maps(input)?;
    Ok((input, (seeds, maps)))
}

fn task2_inputs(file_content: &str) -> IResult<&str, (Vec<Range<usize>>, MapPipeline)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(
            space1,
            map(
                separated_pair(complete::u32, space1, complete::u32),
                |(x, y)| (x as usize)..((x + y) as usize),
            ),
        ),
        tuple((newline, newline)),
    )(file_content)?;

    let (input, maps) = maps(input)?;
    Ok((input, (seeds, maps)))
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d05/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d05.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "35");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "510109797");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "46");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "9622622");
    }
}
