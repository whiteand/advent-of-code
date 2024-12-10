pub fn solve_part_1(file_content: &str) -> i32 {
    file_content
        .chars()
        .scan(0, |s, c| match c {
            '(' => {
                *s += 1;
                Some(*s)
            }
            ')' => {
                *s -= 1;
                Some(*s)
            }
            _ => Some(*s),
        })
        .last()
        .unwrap_or_default()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let mut balance = 0;
    let mut i = 0;
    let mut chars = file_content.chars();
    loop {
        i += 1;
        match chars.next() {
            Some('(') => {
                balance += 1;
            }
            Some(')') => {
                balance -= 1;
                if balance < 0 {
                    return i;
                }
            }
            None => return i,
            _ => continue,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1("(())")), "0");
        assert_eq!(format!("{}", solve_part_1("()()")), "0");
        assert_eq!(format!("{}", solve_part_1("(((")), "3");
        assert_eq!(format!("{}", solve_part_1("(()(()(")), "3");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "280");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(")")), "1");
        assert_eq!(format!("{}", solve_part_2("()())")), "5");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1797");
    }
}
