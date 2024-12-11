use advent_utils::{
    glam::UVec2,
    nom::{self, IResult},
};
use itertools::Itertools;

const FIRST_CODE: usize = 20151125;

// To continue, please consult the code grid in the manual.  Enter the code at row 2981, column 3075.
fn parse_coords(input: &str) -> IResult<&str, UVec2> {
    let (input, _) = nom::bytes::complete::tag(
        "To continue, please consult the code grid in the manual.  Enter the code at row ",
    )(input)?;
    let (input, row) = nom::character::complete::u32(input)?;
    let (input, _) = nom::bytes::complete::tag(", column ")(input)?;
    let (input, col) = nom::character::complete::u32(input)?;
    let (input, _) = nom::bytes::complete::tag(".")(input)?;

    Ok((input, UVec2::new(col - 1, row - 1)))
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let target = parse_coords(file_content).map(|x| x.1).unwrap();

    sequence(FIRST_CODE)
        .zip(coords())
        .find_map(|(a, b)| (b == target).then_some(a))
        .unwrap()
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    todo!("part 2 is not implemented yet: {file_content}")
}

fn sequence(start: usize) -> impl Iterator<Item = usize> {
    std::iter::successors(Some(start), |x| Some((x * 252533) % 33554393))
        .take_while_inclusive(|x| *x > 0)
}

fn row_coords(row: usize) -> impl Iterator<Item = UVec2> {
    (0..=row).map(move |i| UVec2::new(i as u32, row as u32 - i as u32))
}
fn coords() -> impl Iterator<Item = UVec2> {
    (0..).flat_map(|r| row_coords(r))
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{sequence, FIRST_CODE};

    use super::solve_part_1;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_sequence() {
        assert_eq!(
            vec![FIRST_CODE, 31916031, 18749137, 16080970],
            sequence(FIRST_CODE).take(4).collect_vec()
        );
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve_part_1(EXAMPLE), 16929656);
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "9132360");
    }
}
