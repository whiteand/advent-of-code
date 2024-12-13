use core::str;

use advent_utils::nom::{
    self, branch::alt, bytes::complete::tag, character::complete::alpha1, multi::many1, IResult,
    Parser,
};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, supports_tls)
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, supports_ssl)
}
fn solve(file_content: &str, f: impl Fn(&str) -> bool) -> usize {
    file_content.trim().lines().filter(|x| f(x)).count()
}
enum Seq<'t> {
    Hypernet(&'t str),
    Supernet(&'t str),
}

fn parse_ip(input: &str) -> IResult<&str, Vec<Seq<'_>>> {
    many1(alt((
        nom::sequence::delimited(tag("["), alpha1, tag("]")).map(Seq::Hypernet),
        alpha1.map(Seq::Supernet),
    )))(input)
}
fn supports_tls(input: &str) -> bool {
    let values = parse_ip(input).map(|x| x.1).unwrap();
    values
        .iter()
        .any(|x| matches!(x, Seq::Supernet(s) if has_abba(s)))
        && values.iter().all(|x| match x {
            Seq::Supernet(_) => true,
            Seq::Hypernet(s) => !has_abba(s),
        })
}
fn supports_ssl(input: &str) -> bool {
    let values = parse_ip(input).map(|x| x.1).unwrap();
    values
        .iter()
        .filter_map(|x| match x {
            Seq::Supernet(s) => Some(s),
            _ => None,
        })
        .flat_map(|part| {
            part.as_bytes()
                .windows(3)
                .filter(|w| w[0] == w[2] && w[1] != w[0])
        })
        .any(|window| {
            values
                .iter()
                .filter_map(|x| match x {
                    Seq::Hypernet(s) => Some(s),
                    _ => None,
                })
                .any(|s| s.contains(str::from_utf8(&[window[1], window[0], window[1]]).unwrap()))
        })
}

fn has_abba(part: &str) -> bool {
    part.as_bytes()
        .windows(4)
        .any(|w| w[0] == w[3] && w[1] == w[2] && w[0] != w[1])
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case("abba[mnop]qrst", true)]
    #[case("abcd[bddb]xyyx", false)]
    #[case("aaaa[qwer]tyui", false)]
    #[case("ioxxoj[asdfgh]zxcvbn", true)]
    fn test_supports_tls(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(super::supports_tls(input), expected);
    }
    #[rstest]
    #[case("aba[bab]xyz", true)]
    #[case("xyx[xyx]xyx", false)]
    #[case("aaa[kek]eke", true)]
    #[case("zazbz[bzb]cdb", true)]
    fn test_supports_ssl(#[case] input: &str, #[case] expected: bool) {
        if expected {
            if !super::supports_ssl(input) {
                panic!("{input} supports ssl")
            }
        } else if super::supports_ssl(input) {
            panic!("{input} do not support ssl")
        }
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "105");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "258");
    }
}
