pub mod array_2d;
mod binary_search;
pub mod bitfield;
pub mod doubly_linked_list;
pub mod fixed_slice_vec;
pub mod genetic;
pub mod grid;
pub mod immutable_lists;
pub mod math;
pub mod parse;
pub mod reduces;

pub use array_2d::Array2d;
pub use binary_search::binary_search;
pub use glam;
pub use immutable_lists::*;
pub use rand;
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
