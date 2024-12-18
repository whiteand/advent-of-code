use itertools::Itertools;

#[inline]
fn find_sum_of_interpolations(file_content: &str, extrapolate: impl Fn(Vec<i64>) -> i64) -> i64 {
    file_content
        .lines()
        .map(move |line| {
            extrapolate(
                line.split_ascii_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .sum()
}

fn extrapolate(values: Vec<i64>) -> i64 {
    let mut previous = Vec::new();
    let mut current = values;
    let mut lasts = Vec::new();
    loop {
        (previous, current) = (current, previous);
        current.clear();
        let mut has_no_zero = false;
        let mut last = 0;
        for (a, b) in previous.iter().tuple_windows() {
            last = *b;
            let v = b - a;
            if v != 0 {
                has_no_zero = true;
            }
            current.push(v)
        }
        lasts.push(last);
        if !has_no_zero {
            break;
        }
    }
    lasts.into_iter().sum()
}

fn extrapolate_back(mut values: Vec<i64>) -> i64 {
    values.reverse();
    extrapolate(values)
}

pub fn solve_part_1(file_content: &str) -> i64 {
    find_sum_of_interpolations(file_content, extrapolate)
}
pub fn solve_part_2(file_content: &str) -> i64 {
    find_sum_of_interpolations(file_content, extrapolate_back)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "114");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1708206096");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "2");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1050");
    }
}
