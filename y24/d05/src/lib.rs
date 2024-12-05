use std::cmp::Ordering;

use nom::{
    bytes::complete::tag, character::complete, multi::separated_list1, sequence::separated_pair,
    IResult, Parser,
};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let (_, (comparisons, sequences)) = parse(file_content).expect("should be valid");
    let mut buf = Vec::with_capacity(20);
    let mut total = 0usize;
    for seq in sequences {
        buf.clear();
        buf.extend_from_slice(&seq);
        sort_by(&comparisons, &mut buf);
        if seq.iter().enumerate().all(|(i, x)| *x == buf[i]) {
            let middle = seq.len() / 2;
            total += seq[middle];
        }
    }
    total
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let (_, (comparisons, sequences)) = parse(file_content).expect("should be valid");
    let mut buf = Vec::with_capacity(20);
    let mut total = 0usize;
    for seq in sequences {
        buf.clear();
        buf.extend_from_slice(&seq);
        sort_by(&comparisons, &mut buf);
        if seq.iter().enumerate().any(|(i, x)| *x != buf[i]) {
            let middle = buf.len() / 2;
            total += buf[middle];
        }
    }
    total
}

fn sort_by(comparisons: &[(usize, usize)], list: &mut [usize]) {
    list.sort_unstable_by(|a, b| {
        let a = *a;
        let b = *b;
        for (x, y) in comparisons.iter().copied() {
            if x == a && y == b {
                return Ordering::Less;
            }
            if x == b && y == a {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    });
}

fn parse(input: &str) -> IResult<&str, (Vec<(usize, usize)>, Vec<Vec<usize>>)> {
    separated_pair(parse_ordering, tag("\n\n"), parse_lists)(input)
}
fn parse_ordering(input: &str) -> IResult<&str, Vec<(usize, usize)>> {
    separated_list1(
        tag("\n"),
        separated_pair(
            complete::u64.map(|x| x as usize),
            tag("|"),
            complete::u64.map(|x| x as usize),
        ),
    )(input)
}
fn parse_lists(input: &str) -> IResult<&str, Vec<Vec<usize>>> {
    separated_list1(
        tag("\n"),
        separated_list1(tag(","), complete::u64.map(|x| x as usize)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "143");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "4766");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "123");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(tracing_subscriber::FmtSubscriber::new());
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "6257");
    }
}
