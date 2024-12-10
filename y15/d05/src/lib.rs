use std::task::Poll;

use itertools::Itertools;

trait Rule {
    fn next(&mut self, byte: u8) -> Poll<bool>;
    fn last(&self) -> Poll<bool>;
}

struct ThreeVowels {
    appeared: usize,
}

impl ThreeVowels {
    fn new() -> Self {
        Self { appeared: 0 }
    }
}

impl Default for ThreeVowels {
    fn default() -> Self {
        Self::new()
    }
}

impl Rule for ThreeVowels {
    fn next(&mut self, byte: u8) -> Poll<bool> {
        match byte {
            b'a' | b'e' | b'i' | b'o' | b'u' => {
                self.appeared += 1;
            }
            _ => {}
        }
        self.last()
    }

    fn last(&self) -> Poll<bool> {
        if self.appeared >= 3 {
            Poll::Ready(true)
        } else {
            Poll::Pending
        }
    }
}

struct AppearsTwice {
    last_byte: u8,
    found: bool,
}

impl AppearsTwice {
    fn new() -> Self {
        Self {
            last_byte: 0,
            found: false,
        }
    }
}

impl Rule for AppearsTwice {
    fn next(&mut self, byte: u8) -> Poll<bool> {
        if self.found {
            return Poll::Ready(true);
        }
        if self.last_byte == byte {
            self.found = true;
        } else {
            self.last_byte = byte;
        }
        self.last()
    }

    fn last(&self) -> Poll<bool> {
        if self.found {
            Poll::Ready(true)
        } else {
            Poll::Pending
        }
    }
}

struct ForbidenSeq<'t> {
    seq: &'t [u8],
    next_ind: usize,
    found: bool,
}
impl<'t> ForbidenSeq<'t> {
    fn new(seq: &'t [u8]) -> Self {
        Self {
            seq,
            next_ind: 0,
            found: false,
        }
    }
}
impl Rule for ForbidenSeq<'_> {
    fn next(&mut self, byte: u8) -> Poll<bool> {
        if self.found {
            return Poll::Ready(false);
        }
        if self.next_ind >= self.seq.len() {
            return Poll::Pending;
        }
        let expected = self.seq.get(self.next_ind).copied().unwrap();
        if expected != byte {
            if self.next_ind == 0 {
                return Poll::Pending;
            }
            self.next_ind = 0;
            return self.next(byte);
        }
        self.next_ind += 1;
        if self.next_ind == self.seq.len() {
            self.next_ind = 0;
            self.found = true;
        }
        self.last()
    }

    fn last(&self) -> Poll<bool> {
        if self.found {
            Poll::Ready(false)
        } else {
            Poll::Pending
        }
    }
}

fn is_nice_1(line: &str) -> bool {
    let mut three_vowels = ThreeVowels::new();
    let mut appears_twice = AppearsTwice::new();
    let mut ab_rule = ForbidenSeq::new(b"ab");
    let mut cd_rule = ForbidenSeq::new(b"cd");
    let mut pq_rule = ForbidenSeq::new(b"pq");
    let mut xy_rule = ForbidenSeq::new(b"xy");
    for b in line.as_bytes().iter().copied() {
        let _ = three_vowels.next(b);
        let _ = ab_rule.next(b);
        let _ = cd_rule.next(b);
        let _ = pq_rule.next(b);
        let _ = xy_rule.next(b);
        let _ = appears_twice.next(b);
    }

    matches!(
        (
            three_vowels.last(),
            ab_rule.last(),
            cd_rule.last(),
            pq_rule.last(),
            xy_rule.last(),
            appears_twice.last(),
        ),
        (
            Poll::Ready(true),
            Poll::Pending,
            Poll::Pending,
            Poll::Pending,
            Poll::Pending,
            Poll::Ready(true),
        )
    )
}
// It contains a pair of any two letters that appears at least twice in the string without overlapping, like xyxy (xy) or aabcdefgaa (aa), but not like aaa (aa, but it overlaps).

// It contains at least one letter which repeats with exactly one letter between them, like xyx, abcdefeghi (efe), or even aaa.

// xyxy

fn is_nice_2(line: &str) -> bool {
    let bs = line.as_bytes();
    (0..(bs.len() - 3)).any(|i| {
        let pair = &bs[i..(i + 2)];
        ((i + 2)..(bs.len() - 1)).any(|j| &bs[j..(j + 2)] == pair)
    }) && bs.iter().copied().tuple_windows().any(|(a, _, c)| a == c)
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, is_nice_1)
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, is_nice_2)
}

fn solve(file_content: &str, is_nice: impl Fn(&str) -> bool) -> usize {
    file_content.lines().filter(|x| is_nice(x)).count()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const EXAMPLE2: &str = include_str!("../example2.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "2");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "255");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE2)), "2");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "55");
    }
}
