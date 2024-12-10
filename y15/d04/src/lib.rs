use md5::{Digest, Md5};

pub fn solve_part_1(file_content: &str) -> u32 {
    (1..u32::MAX)
        .find(|x| {
            let x = format!("{file_content}{}", x);
            let mut hasher = Md5::new();
            hasher.update(x);
            let mut digest = hasher.finalize().into_iter();
            let first = digest.next().unwrap();
            let second = digest.next().unwrap();
            let third = digest.next().unwrap();
            first == 0 && second == 0 && third < 16
        })
        .unwrap_or_default()
}
pub fn solve_part_2(file_content: &str) -> u32 {
    (1..u32::MAX)
        .find(|x| {
            let x = format!("{file_content}{}", x);
            let mut hasher = Md5::new();
            hasher.update(x);
            let mut digest = hasher.finalize().into_iter();
            let first = digest.next().unwrap();
            let second = digest.next().unwrap();
            let third = digest.next().unwrap();
            first == 0 && second == 0 && third == 0
        })
        .unwrap_or_default()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "abcdef";
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "609043");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL.trim())), "346386");
    }

    #[test]
    #[ignore] // long
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "6742839");
    }

    #[test]
    #[ignore] // long
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL.trim())), "9958218");
    }
}
