use advent_utils::{
    glam::IVec2,
    grid::Grid,
    nom::{
        self,
        bytes::complete::tag,
        character::complete::{self, line_ending, multispace1, not_line_ending},
        combinator::all_consuming,
        multi::separated_list1,
        parse_usize,
        sequence::{preceded, tuple},
    },
};
use itertools::iproduct;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let nodes = parse_nodes(file_content).map(|x| x.1).unwrap();
    iproduct!(nodes.iter(), nodes.iter())
        .filter(|(a, b)| a.0 != b.0 && a.1.used > 0 && a.1.used <= b.1.avail())
        .count()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let nodes = parse_nodes(file_content).map(|x| x.1).unwrap();
    let max_pos = nodes
        .iter()
        .map(|(pos, _)| *pos)
        .fold(IVec2::new(0, 0), |a, b| a.max(b));
    let mut grid = Grid::new(max_pos + IVec2::splat(1), Node { size: 0, used: 0 });
    for (pos, node) in nodes {
        grid.set(pos, node);
    }

    0
}

fn parse_nodes(input: &str) -> nom::IResult<&str, Vec<(IVec2, Node)>> {
    all_consuming(preceded(
        tuple((not_line_ending, line_ending, not_line_ending, line_ending)),
        separated_list1(line_ending, parse_pos_and_node),
    ))(input.trim())
}
fn parse_pos_and_node(input: &str) -> nom::IResult<&str, (IVec2, Node)> {
    // /dev/grid/node-x0-y0     87T   71T    16T   81%
    let (input, _) = tag("/dev/grid/node-x")(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag("-y")(input)?;
    let (input, y) = complete::i32(input)?;
    let pos = IVec2::new(x, y);
    let (input, _) = multispace1(input)?;
    let (input, size) = parse_usize(input)?;
    let (input, _) = tuple((complete::char('T'), multispace1))(input)?;
    let (input, used) = parse_usize(input)?;
    let (input, _) = not_line_ending(input)?;
    Ok((input, (pos, Node { size, used })))
}

#[derive(Clone, Copy)]
struct Node {
    size: usize,
    used: usize,
}
impl Node {
    fn avail(&self) -> usize {
        self.size - self.used
    }
}

impl std::fmt::Debug for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}/{})", self.used, self.size)
    }
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "7")]
    #[case::actual(ACTUAL, "892")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "7")]
    // #[case::actual(ACTUAL, "0")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
