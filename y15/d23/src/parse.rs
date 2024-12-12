use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1},
    combinator,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    Parser,
};

use crate::{Instruction, Offset, Register};

fn parse_register(input: &str) -> nom::IResult<&str, Register> {
    alt((
        combinator::value(Register::A, tag("a")),
        combinator::value(Register::B, tag("b")),
    ))(input)
}
fn parse_offset(input: &str) -> nom::IResult<&str, Offset> {
    alt((
        preceded(tag("+"), complete::u64.map(|x| x as usize)).map(Offset::Forward),
        preceded(tag("-"), complete::u64.map(|x| x as usize)).map(Offset::Backward),
    ))(input)
}
fn parse_inc(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(tag("inc"), multispace1, parse_register)
        .map(|(_, r)| Instruction::Inc(r))
        .parse(input)
}
fn parse_half(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(tag("hlf"), multispace1, parse_register)
        .map(|(_, r)| Instruction::Half(r))
        .parse(input)
}
fn parse_tripple(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(tag("tpl"), multispace1, parse_register)
        .map(|(_, r)| Instruction::Tripple(r))
        .parse(input)
}
fn parse_jump(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(tag("jmp"), multispace1, parse_offset)
        .map(|(_, o)| Instruction::Jump(o))
        .parse(input)
}
fn parse_jump_if_one(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(
        tag("jio"),
        multispace1,
        nom::sequence::separated_pair(
            parse_register,
            nom::sequence::pair(tag(","), multispace1),
            parse_offset,
        )
        .map(|(r, o)| Instruction::JumpIfOne(r, o)),
    )
    .map(|(_, instruction)| instruction)
    .parse(input)
}
fn parse_jump_if_even(input: &str) -> nom::IResult<&str, Instruction> {
    separated_pair(
        tag("jie"),
        multispace1,
        nom::sequence::separated_pair(
            parse_register,
            nom::sequence::pair(tag(","), multispace1),
            parse_offset,
        )
        .map(|(r, o)| Instruction::JumpIfEven(r, o)),
    )
    .map(|(_, instruction)| instruction)
    .parse(input)
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((
        parse_inc,
        parse_jump_if_one,
        parse_tripple,
        parse_jump,
        parse_jump_if_even,
        parse_half,
    ))(input)
}
pub(super) fn parse_instructions(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    separated_list1(line_ending, parse_instruction)(input)
}
