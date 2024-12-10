use std::collections::HashSet;

pub fn solve_part_1(file_content: &str) -> impl std::fmt::Display {
    solve::<1>(file_content)
}

pub fn solve_part_2(file_content: &str) -> impl std::fmt::Display {
    solve::<2>(file_content)
}

pub fn solve<const N: usize>(file_content: &str) -> usize {
    let mut v = [(0, 0); N];
    let mut current = 0;
    let mut s = HashSet::new();
    s.insert((0, 0));
    for c in file_content.chars() {
        match c {
            '^' => {
                v[current].1 += 1;
            }
            'v' => {
                v[current].1 -= 1;
            }
            '>' => {
                v[current].0 += 1;
            }
            '<' => {
                v[current].0 -= 1;
            }
            _ => {
                continue;
            }
        }
        s.insert(v[current]);
        current += 1;
        current %= v.len();
    }
    s.len()
}
#[cfg(test)]
mod tests {
    use super::*;
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(">")), "2");
        assert_eq!(format!("{}", solve_part_1("^>v<")), "4");
        assert_eq!(format!("{}", solve_part_1("^v^v^v^v^v")), "2");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "2081");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2("^v")), "3");
        assert_eq!(format!("{}", solve_part_2("^>v<")), "3");
        assert_eq!(format!("{}", solve_part_2("^v^v^v^v^v")), "11");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "2341");
    }
}
