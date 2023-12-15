use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Hasher {
    current: u8,
}

impl Hasher {
    fn new() -> Self {
        Self { current: 0 }
    }
    fn write_byte(&mut self, b: u8) {
        self.current = self.current.wrapping_add(b);
        self.current = self.current.wrapping_mul(17)
    }
}

// Determine the ASCII code for the current character of the string.
// Increase the current value by the ASCII code you just determined.
// Set the current value to itself multiplied by 17.
// Set the current value to the remainder of dividing itself by 256.

impl Write for Hasher {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        for b in s.bytes() {
            self.write_byte(b)
        }
        Ok(())
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    file_content
        .split(',')
        .map(|x| x.trim_matches('\n'))
        .map(|line| {
            let mut hasher = Hasher::new();

            write!(hasher, "{}", line).unwrap();
            hasher.current as usize
        })
        .sum()
}
pub fn solve_task2(_file_content: &str) -> impl std::fmt::Display {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d15/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d15.txt");
    #[test]
    fn test_hasher() {
        let mut hasher = Hasher::new();
        hasher.write_byte(b'H');
        assert_eq!(hasher.current, 200);
        hasher.write_byte(b'A');
        assert_eq!(hasher.current, 153);
        hasher.write_byte(b'S');
        assert_eq!(hasher.current, 172);
        hasher.write_byte(b'H');
        assert_eq!(hasher.current, 52);
    }

    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "1320");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "515495");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "0");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
