use itertools::Itertools;

pub fn solve_task1(file_content: &str) -> usize {
    file_content
        .lines()
        .map(parse_line)
        .map(|(states, ranges)| get_arrangements_number(states, &ranges))
        .sum()
}

pub fn unfold_line(line: &str) -> String {
    let (states, ranges) = line.split_once(' ').unwrap();
    format!(
        "{} {}",
        (0..5).map(|_| states).join("?"),
        (0..5).map(|_| ranges).join(",")
    )
}

pub fn solve_task2(file_content: &str) -> usize {
    file_content
        .lines()
        .map(unfold_line)
        .map(|s| parse_line(&s))
        .map(|(states, ranges)| get_arrangements_number(states, &ranges))
        .sum()
}

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
    fn is_set(&self, bit: usize) -> bool {
        self.bits & (1 << bit) != 0
    }
    #[inline]
    fn is_empty(&self) -> bool {
        self.bits == 0
    }
    fn slice(&self, from: usize, to: usize) -> Self {
        Self {
            bits: (self.bits >> from) & ((1 << (to - from)) - 1),
        }
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
        let mut res = *self;
        res.len = res.len.max(i + 1);
        res.operational.set(i);
        res
    }
    fn with_damaged(&self, i: usize) -> Self {
        let mut res = *self;
        res.len = res.len.max(i + 1);
        res.damaged.set(i);
        res
    }
    fn with_unknown(&self, i: usize) -> Self {
        let mut res = *self;
        res.len = res.len.max(i + 1);
        res
    }

    fn is_operational(&self, i: usize) -> Option<bool> {
        if i >= self.len {
            return Some(false);
        }
        if self.operational.is_set(i) {
            return Some(true);
        }
        if self.damaged.is_set(i) {
            return Some(false);
        }
        None
    }

    fn new() -> Self {
        Self {
            len: 0,
            operational: Bitmask::new(),
            damaged: Bitmask::new(),
        }
    }
}

fn get_min_len(ranges: &[usize]) -> usize {
    if ranges.is_empty() {
        return 0;
    }
    ranges.iter().map(|r| r + 1).sum::<usize>() - 1
}

fn get_arrangements_number(row: Row, damaged_ranges: &[usize]) -> usize {
    let ranges_number = damaged_ranges.len();
    let len = row.len;
    let mut s = vec![vec![usize::MAX; len + 1]; ranges_number + 1];
    for i in 0..=len {
        s[0][i] = if row.damaged.slice(0, i).is_empty() {
            1
        } else {
            0
        };
    }
    for r in 1..=ranges_number {
        let min_len = get_min_len(&damaged_ranges[..r]);
        for i in 0..min_len {
            s[r][i] = 0;
        }

        let mask = &damaged_ranges[..r]
            .iter()
            .rev()
            .fold(0, |acc, r| (acc << (r + 1)) | ((1 << r) - 1));
        if row.operational.slice(0, min_len).bits & mask != 0 {
            s[r][min_len] = 0;
            continue;
        }
        if row.damaged.slice(0, min_len).bits & !mask != 0 {
            s[r][min_len] = 0;
            continue;
        }
        s[r][min_len] = 1;
    }

    for ranges_to_place in 1..=ranges_number {
        for l in 1..=len {
            if s[ranges_to_place][l] != usize::MAX {
                continue;
            }
            let last_range = damaged_ranges[ranges_to_place - 1];
            let new_pos_is_damaged = row.is_operational(l - 1);
            match new_pos_is_damaged {
                None => {
                    let mut r = s[ranges_to_place][l - 1];

                    if row.operational.slice(l - last_range, l).is_empty()
                        && !row.damaged.is_set(l - last_range - 1)
                    {
                        r += s[ranges_to_place - 1][l - last_range - 1];
                    }

                    s[ranges_to_place][l] = r;
                    continue;
                }
                Some(true) => {
                    s[ranges_to_place][l] = s[ranges_to_place][l - 1];
                    continue;
                }
                Some(false) => {
                    let mut r = 0;
                    if row.operational.slice(l - last_range, l).is_empty()
                        && !row.damaged.is_set(l - last_range - 1)
                    {
                        r += s[ranges_to_place - 1][l - last_range - 1];
                    }
                    s[ranges_to_place][l] = r;
                    continue;
                }
            }
        }
    }

    s[ranges_number][len]
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

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d12/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d12.txt");

    #[test]
    fn test_task1() {
        for (line, expected) in INPUT.lines().zip([1, 4, 1, 1, 4, 10usize]) {
            let (row, ranges) = parse_line(line);
            let calculated = get_arrangements_number(row, &ranges);

            assert_eq!(calculated, expected, "Line:\n  {line}");
        }
    }

    #[test]
    fn temporal_test() {
        let line = "????????????????????????????????????????????????????????????????????????????????????????? 5,1,1,5,1,1,5,1,1,5,1,1,5,1,1";
        let (row, ranges) = parse_line(line);
        let res = get_arrangements_number(row, &ranges);
        assert_eq!(11899700525790, res, "Line:\n  {line}");
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
            let calculated = get_arrangements_number(row, &ranges);
            assert_eq!(calculated, expected, "Line:\n  {line}");
        }
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "15454556629917");
    }
}
