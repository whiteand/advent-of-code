use std::collections::HashSet;

pub fn solve_task1(file_content: &str) -> impl std::fmt::Display {
    solve::<1>(file_content)
}

pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
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
    const ACTUAL: &str = include_str!("../../benches/y15/y15d03.txt");

    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(">")), "2");
        assert_eq!(format!("{}", solve_task1("^>v<")), "4");
        assert_eq!(format!("{}", solve_task1("^v^v^v^v^v")), "2");
    }

    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "2081");
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2("^v")), "3");
        assert_eq!(format!("{}", solve_task2("^>v<")), "3");
        assert_eq!(format!("{}", solve_task2("^v^v^v^v^v")), "11");
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "2341");
    }
}
