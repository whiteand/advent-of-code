use advent_utils::nom::{bytes::complete, character, multi::separated_list1, IResult, Parser};

pub fn solve<F>(file_content: &str, f: F) -> u32
where
    F: Fn((u32, u32, u32)) -> u32,
{
    file_content
        .lines()
        .map(|line| parse_dimensions(line).unwrap().1)
        .map(f)
        .sum()
}

pub fn solve_part_1(file_content: &str) -> u32 {
    solve(file_content, |(l, w, h)| {
        2 * l * w + 2 * w * h + 2 * l * h + l * w
    })
}
pub fn solve_part_2(file_content: &str) -> u32 {
    solve(file_content, |(l, w, h)| 2 * (l + w) + l * w * h)
}

fn parse_dimensions(line: &str) -> IResult<&str, (u32, u32, u32)> {
    let sep = complete::tag("x");
    let dimension = character::complete::u32;
    let (input, mut v) = separated_list1(sep, dimension).parse(line)?;
    v.sort();
    Ok((input, (v[0], v[1], v[2])))
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2x3x4
1x1x10";
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "101");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1606483");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "48");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "3842356");
    }
}
