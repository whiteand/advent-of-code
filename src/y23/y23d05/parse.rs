use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{self, newline, not_line_ending, space1},
    combinator::map,
    multi::{separated_list0, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated, tuple},
    IResult,
};

use super::{int_map::IntMap, map_pipeline::MapPipeline, range_map::RangeMap};

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

fn int_map<'b>(input: &'b str) -> IResult<&'b str, IntMap> {
    map(separated_list0(newline, range_map), IntMap::new)(input)
}

fn maps(input: &str) -> IResult<&str, MapPipeline> {
    let (input, maps) = separated_list1(
        tuple((newline, newline)),
        preceded(tuple((not_line_ending, newline)), int_map),
    )(input)?;

    Ok((input, MapPipeline::new(maps)))
}
pub(super) fn task1_inputs(file_content: &str) -> IResult<&str, (Vec<usize>, MapPipeline)> {
    let (input, seeds) = delimited(
        tag("seeds: "),
        separated_list1(space1, map(complete::u32, |x| x as usize)),
        tuple((newline, newline)),
    )(file_content)?;

    let (input, maps) = maps(input)?;
    Ok((input, (seeds, maps)))
}

pub(super) fn task2_inputs(file_content: &str) -> IResult<&str, (Vec<Range<usize>>, MapPipeline)> {
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
