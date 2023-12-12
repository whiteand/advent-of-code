use std::{cmp::Ordering, ops::Range};

use itertools::Itertools;

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bitmask {
    bits: u128,
}
impl std::ops::Deref for Bitmask {
    type Target = u128;
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}

impl Bitmask {
    fn new() -> Self {
        Self { bits: 0 }
    }
    fn set(&mut self, bit: usize) {
        self.bits |= 1 << bit;
    }
    fn unset(&mut self, bit: usize) {
        self.bits &= !(1 << bit);
    }
    fn is_set(&self, bit: usize) -> bool {
        self.bits & (1 << bit) != 0
    }
    fn count(&self) -> usize {
        self.bits.count_ones() as usize
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.bits.count_ones() == 0
    }
    fn slice(&self, from: usize, to: usize) -> Self {
        Self {
            bits: (self.bits >> from) & (1 << (to - from)) - 1,
        }
    }
    fn with_set(&self, bit: usize) -> Self {
        let mut res = *self;
        res.set(bit);
        res
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Row {
    operational: Bitmask,
    damaged: Bitmask,
    len: usize,
}

impl std::fmt::Debug for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            if self.operational.is_set(i) {
                write!(f, ".")?;
            } else if self.damaged.is_set(i) {
                write!(f, "#")?;
            } else {
                write!(f, "?")?;
            }
        }
        Ok(())
    }
}

impl Row {
    fn with_operational(&self, i: usize) -> Self {
        let mut res = self.clone();
        res.len = res.len.max(i + 1);
        res.operational.set(i);
        res
    }
    fn with_damaged(&self, i: usize) -> Self {
        let mut res = self.clone();
        res.len = res.len.max(i + 1);
        res.damaged.set(i);
        res
    }
    fn with_unknown(&self, i: usize) -> Self {
        let mut res = self.clone();
        res.len = res.len.max(i + 1);
        res.damaged.unset(i);
        res.operational.unset(i);
        res
    }

    #[inline]
    fn has_damaged(&self) -> bool {
        !self.damaged.is_empty()
    }

    fn is_operational(&self, i: usize) -> Option<bool> {
        debug_assert!(i < self.len, "There is no pipe with index {i}");
        if self.operational.is_set(i) {
            return Some(true);
        }
        if self.damaged.is_set(i) {
            return Some(false);
        }
        return None;
    }

    fn with_super_position(&self, i: usize) -> (Self, Self) {
        (self.with_operational(i), self.with_damaged(i))
    }
    fn skip(&self, from: usize) -> Self {
        Self {
            operational: self.operational.slice(from, self.len),
            damaged: self.damaged.slice(from, self.len),
            len: self.len - from,
        }
    }

    fn new() -> Self {
        Self {
            len: 0,
            operational: Bitmask::new(),
            damaged: Bitmask::new(),
        }
    }
}

fn get_arrangements_number(states: Row, damaged_ranges: Vec<usize>) -> usize {
    let mut tasks = vec![(states, 0)];
    let mut res = 0;
    'to_next_task: while let Some((mut row, damaged_range_index)) = tasks.pop() {
        if damaged_range_index >= damaged_ranges.len() {
            if row.has_damaged() {
                continue 'to_next_task;
            }
            res += 1;
            continue 'to_next_task;
        }

        let mut damaged_count = 0;
        let mut expected_len = damaged_ranges[damaged_range_index];
        let min_len = &damaged_ranges[damaged_range_index..]
            .into_iter()
            .map(|v| v + 1)
            .sum::<usize>()
            - 1;

        if row.len < min_len {
            continue 'to_next_task;
        }

        for i in 0..row.len {
            match row.is_operational(i) {
                Some(true) => {
                    if damaged_count == 0 {
                        tasks.push((row.skip(i + 1), damaged_range_index));
                        continue 'to_next_task;
                    }
                    if damaged_count != expected_len {
                        continue 'to_next_task;
                    }
                    tasks.push((row.skip(i + 1), damaged_range_index + 1));
                    continue 'to_next_task;
                }
                Some(false) => {
                    if damaged_count >= expected_len {
                        continue 'to_next_task;
                    }
                    damaged_count += 1;
                    continue;
                }
                None => {
                    if damaged_count > expected_len {
                        continue 'to_next_task;
                    }
                    if damaged_count == expected_len {
                        tasks.push((row.skip(i + 1), damaged_range_index + 1));
                        continue 'to_next_task;
                    }
                    if damaged_count > 0 {
                        row = row.with_damaged(i);
                        damaged_count += 1;
                        continue;
                    }
                    let (operational, damaged) = row.with_super_position(i);
                    tasks.push((operational, damaged_range_index));
                    tasks.push((damaged, damaged_range_index));
                    continue 'to_next_task;
                }
            }
        }
        if damaged_count != expected_len {
            continue 'to_next_task;
        }
        res += 1;
    }
    res
}

fn parse_line(line: &str) -> (Row, Vec<usize>) {
    let (left, right) = line.split_once(' ').unwrap();
    let states = left.chars().enumerate().fold(Row::new(), |r, (i, c)| {
        debug_assert!(i < 128, "bit mask is too short to store all states");
        match c {
            '#' => r.with_damaged(i),
            '.' => r.with_operational(i),
            '?' => r.with_unknown(i),
            v => unreachable!("There is no such state as {v}"),
        }
    });

    let ranges = right
        .split(',')
        .map(|r| r.parse::<usize>().unwrap())
        .collect();

    (states, ranges)
}

pub fn solve_task1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(parse_line)
        .map(|(states, ranges)| get_arrangements_number(states, ranges))
        .sum()
}

pub fn unfold_line(line: &str) -> String {
    let (states, ranges) = line.split_once(' ').unwrap();
    format!(
        "{} {}",
        (0..5).map(|f| states).join("?"),
        (0..5).map(|f| ranges).join(",")
    )
}

pub fn solve_task2(file_content: &str) -> usize {
    file_content
        .lines()
        .map(unfold_line)
        .map(|s| parse_line(&s))
        .enumerate()
        .map(|(i, (states, ranges))| {
            println!("{i}");
            get_arrangements_number(states, ranges)
        })
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d12/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d12.txt");

    #[test]
    fn test_task1() {
        for (line, expected) in INPUT.lines().zip([1, 4, 1, 1, 4, 10usize]) {
            let (row, ranges) = parse_line(line);
            let calculated = get_arrangements_number(row, ranges);
            assert_eq!(calculated, expected, "Line:\n  {line}")
        }
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "7599");
    }

    #[test]
    fn test_task2() {
        for (line, expected) in INPUT.lines().zip([1, 16384, 1, 16, 2500, 506250usize]) {
            let unfolded = unfold_line(line);
            let (row, ranges) = parse_line(&unfolded);
            let calculated = get_arrangements_number(row, ranges);
            assert_eq!(calculated, expected, "Line:\n  {line}")
        }
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
