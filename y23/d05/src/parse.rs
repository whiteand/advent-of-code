use std::ops::Range;

use advent_utils::nom::{
    bytes::complete::tag,
    character::complete::{self, newline, not_line_ending, space1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult, Parser,
};

use super::{int_map::IntMap, map_pipeline::MapPipeline, range_map::RangeMap};

fn range_map(input: &str) -> IResult<&str, RangeMap> {
    let (input, (dst, src, length)) = (
        (terminated(complete::u32, space1)),
        (terminated(complete::u32, space1)),
        complete::u32,
    )
        .parse(input)?;
    Ok((
        input,
        RangeMap::new(dst as usize, src as usize, length as usize),
    ))
}

fn int_map(input: &str) -> IResult<&str, IntMap> {
    map(separated_list0(newline, range_map), IntMap::new).parse(input)
}

fn maps(input: &str) -> IResult<&str, MapPipeline> {
    let (input, maps) = separated_list0(
        (newline, newline),
        preceded((not_line_ending, newline), int_map),
    )
    .parse(input)?;

    Ok((input, MapPipeline::new(maps)))
}
pub(super) fn part_1_inputs(file_content: &str) -> IResult<&str, (Vec<usize>, MapPipeline)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, map(complete::u32, |x| x as usize)),
        (newline, newline),
    )
    .parse(file_content)?;

    let (input, maps) = maps(input)?;
    Ok((input, (seeds, maps)))
}

pub(super) fn part_2_inputs(file_content: &str) -> IResult<&str, (Vec<Range<usize>>, MapPipeline)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(
            space1,
            map(
                separated_pair(complete::u32, space1, complete::u32),
                |(x, y)| (x as usize)..((x + y) as usize),
            ),
        ),
        (newline, newline),
    )
    .parse(file_content)?;

    let (input, maps) = maps.parse(input)?;
    Ok((input, (seeds, maps)))
}
