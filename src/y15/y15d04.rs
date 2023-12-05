use md5::{Digest, Md5};

pub fn solve_task1(file_content: &str) -> u32 {
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
pub fn solve_task2(file_content: &str) -> u32 {
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
    const ACTUAL: &str = include_str!("../../benches/y15/y15d04.txt");
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "609043");
    }

    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "346386");
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "6742839");
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "9958218");
    }
}
