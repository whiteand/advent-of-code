use advent_utils::nom::{
    self,
    bytes::complete::tag,
    character::complete,
    sequence::{delimited, separated_pair},
    Parser,
};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    expand(file_content.split_ascii_whitespace().join("").as_str()).len()
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    get_expanded_len(file_content.split_ascii_whitespace().join("").as_str())
}

fn get_expanded_len(mut input: &str) -> usize {
    let mut res = 0;
    while !input.is_empty() {
        let Ok((rest, (len, times))) = parse_group(input) else {
            res += 1;
            input = &input[1..];
            continue;
        };
        let (section, rest) = rest.split_at(len);
        res += times * get_expanded_len(section);
        input = rest;
    }
    res
}
fn expand(mut input: &str) -> String {
    let mut res = String::new();
    while !input.is_empty() {
        let Ok((rest, (len, times))) = parse_group(input) else {
            res.push_str(&input[0..1]);
            input = &input[1..];
            continue;
        };
        let (section, rest) = rest.split_at(len);
        for _ in 0..times {
            res.push_str(section);
        }
        input = rest;
    }
    res
}

fn parse_group(input: &str) -> nom::IResult<&str, (usize, usize)> {
    delimited(
        tag("("),
        separated_pair(
            complete::u32.map(|x| x as usize),
            tag("x"),
            complete::u32.map(|x| x as usize),
        ),
        tag(")"),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{solve_part_1, solve_part_2};
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case("ADVENT", "ADVENT")]
    #[case("A(1x5)BC", "ABBBBBC")]
    #[case("(3x3)XYZ", "XYZXYZXYZ")]
    #[case("A(2x2)BCD(2x2)EFG", "ABCBCDEFEFG")]
    #[case("(6x1)(1x3)A", "(1x3)A")]
    #[case("X(8x2)(3x3)ABCY", "X(3x3)ABC(3x3)ABCY")]

    fn test_expand(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(super::expand(input).as_str(), expected)
    }

    #[rstest]
    #[case("(3x3)XYZ", 9)]
    #[case("X(8x2)(3x3)ABCY", "XABCABCABCABCABCABCY".len())]
    fn test_get_count_expand(#[case] input: &str, #[case] expected: usize) {
        assert_eq!(super::get_expanded_len(input), expected);
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "115118");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "11107527530");
    }
}
