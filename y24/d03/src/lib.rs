const MUL_REGEX: &str = r#"(mul)\((\d{1,3}),(\d{1,3})\)"#;
const DO_REGEX: &str = r#"(do)\(\)"#;
const DONT_REGEX: &str = r#"(don't)\(\)"#;
pub fn solve_part_1(file_content: &str) -> i32 {
    let re = regex::Regex::new(MUL_REGEX).unwrap();
    let mut total = 0i32;
    for capture in re.captures_iter(file_content) {
        let a = capture
            .get(2)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .unwrap();
        let b = capture
            .get(3)
            .map(|x| x.as_str().parse::<i32>().unwrap())
            .unwrap();
        total += a * b;
    }
    total
}
pub fn solve_part_2(file_content: &str) -> i32 {
    let re =
        regex::Regex::new(format!("({MUL_REGEX})|({DO_REGEX})|({DONT_REGEX})").as_str()).unwrap();
    let mut total = 0i32;
    let mut activated = true;
    for capture in re.captures_iter(file_content) {
        if capture.get(2).map_or("", |x| x.as_str()) == "mul" {
            if !activated {
                continue;
            }
            let a = capture
                .get(3)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .unwrap();
            let b = capture
                .get(4)
                .map(|x| x.as_str().parse::<i32>().unwrap())
                .unwrap();
            total += a * b;
            continue;
        }
        if capture.get(8).map_or("", |x| x.as_str()) == "don't" {
            activated = false;
            continue;
        }
        if capture.get(6).map_or("", |x| x.as_str()) == "do" {
            activated = true;
            continue;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "161");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "180233229");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "48");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "95411583");
    }
}
