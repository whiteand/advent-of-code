pub mod doubly_linked_list;
pub mod grid;
pub mod math;
pub mod parse;
pub mod reduces;

pub use glam;
pub mod nom {
    pub use nom::*;
    pub fn parse_usize(input: &str) -> nom::IResult<&str, usize> {
        nom::character::complete::u64
            .map(|x| x as usize)
            .parse(input)
    }
    pub fn parse_isize(input: &str) -> nom::IResult<&str, isize> {
        nom::character::complete::i64
            .map(|x| x as isize)
            .parse(input)
    }
}
