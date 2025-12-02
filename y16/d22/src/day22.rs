use advent_utils::{
    glam::IVec2,
    grid::{Grid, NonDiagonal},
    nom::{
        self,
        bytes::complete::tag,
        character::complete::{self, line_ending, multispace1, not_line_ending},
        combinator::all_consuming,
        multi::separated_list1,
        parse_usize,
        sequence::preceded,
        Parser,
    },
};
use itertools::iproduct;
use tracing::info;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let nodes = parse_nodes(file_content).map(|x| x.1).unwrap();
    iproduct!(nodes.iter(), nodes.iter())
        .filter(|(a, b)| a.0 != b.0 && a.1.used > 0 && a.1.used <= b.1.avail())
        .count()
}
fn include(dst: IVec2, d: IVec2, grid: &mut Grid<Node>) {
    let src_pos = dst + d;
    let src_node = *grid.get(src_pos).unwrap();
    let dst_node = *grid.get(dst).unwrap();
    if !dst_node.can_include(&src_node) {
        unreachable!();
    }
    grid.set(
        dst,
        Node {
            used: dst_node.used + src_node.used,
            ..dst_node
        },
    );
    grid.set(
        src_pos,
        Node {
            used: 0,
            ..src_node
        },
    );
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let nodes = parse_nodes(file_content).map(|x| x.1).unwrap();
    let max_pos = nodes
        .iter()
        .map(|(pos, _)| *pos)
        .fold(IVec2::new(0, 0), |a, b| a.max(b));
    info!(?max_pos);
    let mut grid = Grid::new(max_pos + IVec2::splat(1), Node { size: 0, used: 0 });
    for (pos, node) in nodes {
        grid.set(pos, node);
    }
    let mut goal = IVec2::new(max_pos.x, 0);

    // These steps are found by hand using vizualization below
    let steps = std::iter::repeat_n(IVec2::NEG_Y, 9)
        .chain(std::iter::repeat_n(IVec2::NEG_X, 3))
        .chain(std::iter::repeat_n(IVec2::NEG_Y, 8))
        .chain(std::iter::repeat_n(IVec2::X, 32))
        .chain(
            [IVec2::Y, IVec2::NEG_X, IVec2::NEG_X, IVec2::NEG_Y, IVec2::X]
                .into_iter()
                .cycle()
                .take(175),
        );

    let mut pos = IVec2::new(7, 17);
    let mut steps_len = 0;
    for x in steps {
        steps_len += 1;
        let next_pos = pos + x;
        include(pos, x, &mut grid);
        goal = if next_pos == goal { pos } else { goal };
        pos = next_pos;
    }
    let empty = grid
        .coords()
        .find(|x| grid.get(*x).unwrap().used == 0)
        .unwrap();
    let empty_node = grid.get(empty).unwrap();
    for y in 0..grid.rows_len() {
        let row = grid.row(y).unwrap();
        for x in 0..row.len() {
            let pos = IVec2::new(x as i32, y as i32);
            let node = row[x];
            if node.used == 0 {
                print!("_");
                continue;
            }
            if node.used > empty_node.size {
                print!("x");
                continue;
            }
            if pos == goal {
                print!("G");
                continue;
            }
            let n = grid
                .neighbours(pos, NonDiagonal)
                .filter(|(_, n)| n.can_include(&node))
                .count();
            match n {
                0 => print!("#"),
                x => print!("{x}"),
            }
        }
        println!();
    }
    assert_eq!(goal, IVec2::ZERO);
    steps_len
}

fn parse_nodes(input: &str) -> nom::IResult<&str, Vec<(IVec2, Node)>> {
    all_consuming(preceded(
        (not_line_ending, line_ending, not_line_ending, line_ending),
        separated_list1(line_ending, parse_pos_and_node),
    ))
    .parse(input.trim())
}
fn parse_pos_and_node(input: &str) -> nom::IResult<&str, (IVec2, Node)> {
    // /dev/grid/node-x0-y0     87T   71T    16T   81%
    let (input, _) = tag("/dev/grid/node-x").parse(input)?;
    let (input, x) = complete::i32(input)?;
    let (input, _) = tag("-y").parse(input)?;
    let (input, y) = complete::i32(input)?;
    let pos = IVec2::new(x, y);
    let (input, _) = multispace1(input)?;
    let (input, size) = parse_usize(input)?;
    let (input, _) = (complete::char('T'), multispace1).parse(input)?;
    let (input, used) = parse_usize(input)?;
    let (input, _) = not_line_ending(input)?;
    Ok((input, (pos, Node { size, used })))
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    size: usize,
    used: usize,
}
impl Node {
    fn avail(&self) -> usize {
        self.size - self.used
    }
    fn can_include(&self, other: &Self) -> bool {
        self.avail() >= other.used
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
    #[ignore]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(ACTUAL, "227")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
