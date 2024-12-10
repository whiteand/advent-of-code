use core::str;
use std::{convert::Infallible, fmt::Write, str::FromStr};

use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> String {
    solve(file_content, 0)
}
pub fn solve_part_2(file_content: &str) -> String {
    solve(file_content, 1)
}
fn solve(file_content: &str, skip: usize) -> String {
    let password: Password = file_content.parse().unwrap();
    let next_pass = password.filter(is_valid_password).nth(skip).unwrap();
    format!("{}", next_pass)
}

const PASSWORD_SIZE: usize = 8;
const FORBIDDEN: [u8; 3] = [b'i', b'o', b'l'];

#[derive(Clone, Eq, PartialEq)]
struct Password {
    letters: [u8; PASSWORD_SIZE],
}
impl std::fmt::Display for Password {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for letter in &self.letters {
            f.write_char(*letter as char)?;
        }
        Ok(())
    }
}
impl FromStr for Password {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = [0; PASSWORD_SIZE];
        res.copy_from_slice(&s.as_bytes()[..PASSWORD_SIZE]);
        Ok(Password { letters: res })
    }
}

impl Iterator for Password {
    type Item = Password;

    fn next(&mut self) -> Option<Self::Item> {
        for ptr in (0..self.letters.len()).rev() {
            if self.letters[ptr] < b'z' {
                self.letters[ptr] += 1;
                while FORBIDDEN.contains(&self.letters[ptr]) {
                    self.letters[ptr] += 1;
                }
                return Some(self.clone());
            }
            self.letters[ptr] = b'a';
        }
        None
    }
}

fn is_valid_password(password: &Password) -> bool {
    if password.letters.iter().any(|ch| FORBIDDEN.contains(ch)) {
        return false;
    }
    // Passwords must include one increasing straight of at least three letters, like abc, bcd, cde, and so on, up to xyz. They cannot skip letters; abd doesn't count.
    let has_three_inc = password
        .letters
        .iter()
        .tuple_windows()
        .any(|(a, b, c)| *a + 1 == *b && *b + 1 == *c);

    if !has_three_inc {
        return false;
    }

    let non_overlapping_pairs = password
        .letters
        .iter()
        .tuple_windows()
        .enumerate()
        .map(|(a, (b, c))| (a, b, c))
        .filter(|(_, b, c)| b == c)
        .tuple_windows()
        .filter(|(a, b)| a.0 + 1 != b.0)
        .count()
        + 1;

    if non_overlapping_pairs < 2 {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "abcdffaa");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "hepxxyzz");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "abcdffbb");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "heqaabcc");
    }
}
