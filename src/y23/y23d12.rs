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
    fn unset(&mut self, bit: usize) {
        self.bits &= !(1 << bit);
    }
    fn is_set(&self, bit: usize) -> bool {
        self.bits & (1 << bit) != 0
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
        if i >= self.len {
            return Some(false);
        }
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
    fn with_skip(&self, i: usize) -> Self {
        let mut res = self.clone();
        res.skip(i);
        res
    }
    fn with_skip_last(&self, i: usize) -> Self {
        let mut res = self.clone();
        res.skip_last(i);
        res
    }

    fn skip(&mut self, from: usize) {
        if from == 0 {
            return;
        }
        if from >= self.len {
            self.operational = Bitmask::new();
            self.damaged = Bitmask::new();
            self.len = 0;
            return;
        }
        self.operational = self.operational.slice(from, self.len);
        self.damaged = self.damaged.slice(from, self.len);
        self.len -= from;
    }
    fn skip_last(&mut self, n: usize) {
        if n == 0 {
            return;
        }
        if n >= self.len {
            self.operational = Bitmask::new();
            self.damaged = Bitmask::new();
            self.len = 0;
            return;
        }
        self.operational = self.operational.slice(0, self.len - n);
        self.damaged = self.damaged.slice(0, self.len - n);
        self.len -= n;
    }

    fn starts_with_damaged(&self, n: usize) -> bool {
        self.operational.slice(0, n).is_empty()
    }
    fn ends_with_damaged(&self, n: usize) -> bool {
        if n > self.len {
            return false;
        }
        self.operational.slice(self.len - n, self.len).is_empty()
    }

    fn new() -> Self {
        Self {
            len: 0,
            operational: Bitmask::new(),
            damaged: Bitmask::new(),
        }
    }

    fn get_damaged_prefix_len(&self) -> usize {
        (0..(self.len))
            .take_while(|i| self.damaged.is_set(*i))
            .count()
    }

    fn get_unknown_prefix_len(&self) -> usize {
        (0..(self.len))
            .take_while(|i| !self.operational.is_set(*i) && !self.damaged.is_set(*i))
            .count()
    }
    fn get_unknown_sufix_len(&self) -> usize {
        (0..(self.len))
            .rev()
            .take_while(|i| !self.operational.is_set(*i) && !self.damaged.is_set(*i))
            .count()
    }

    fn get_damaged_suffix_len(&self) -> usize {
        (0..(self.len))
            .rev()
            .take_while(|i| self.damaged.is_set(*i))
            .count()
    }
}

fn get_min_len(ranges: &[usize]) -> usize {
    if ranges.is_empty() {
        return 0;
    }
    ranges.iter().map(|r| r + 1).sum::<usize>() - 1
}

fn count_all_arrangements(row_len: usize, ranges: &[usize]) -> usize {
    if ranges.is_empty() {
        return 1;
    }
    if get_min_len(ranges) > row_len {
        return 0;
    }
    if ranges.len() == 1 {
        return row_len - ranges[0] + 1;
    }
    let rs = ranges.len();

    // Invariant:
    // arr[K][L] = number of arrangement of K ranges over the L length
    let mut solution = vec![vec![99; row_len + 1]; ranges.len() + 1];
    solution[0][0] = 1;
    for l in 0..=row_len {
        solution[0][l] = 1;
    }
    for k in 1..=rs {
        solution[k][0] = 0;
    }
    for k in 1..=rs {
        let taken_elements = ranges[..k].iter().sum::<usize>();
        for l in 1..=row_len {
            if l < taken_elements + k - 1 {
                solution[k][l] = 0;
                continue;
            }
            if l == taken_elements + k - 1 {
                solution[k][l] = 1;
                continue;
            }
            if k == 1 {
                solution[k][l] = l - ranges[0] + 1;
                continue;
            }
            solution[k][l] = solution[k][l - 1] + solution[k - 1][l - ranges[k - 1] - 1];
        }
    }

    solution[ranges.len()][row_len]
}

fn get_arrangements_number(mut row: Row, mut damaged_ranges: &[usize]) -> usize {
    let mut min_len = get_min_len(&damaged_ranges);
    loop {
        if damaged_ranges.is_empty() {
            return if row.has_damaged() { 0 } else { 1 };
        }
        let n = (0..(row.len))
            .take_while(|i| row.operational.is_set(*i))
            .count();
        row.skip(n);
        let n = (0..(row.len))
            .rev()
            .take_while(|i| row.operational.is_set(*i))
            .count();
        row.skip_last(n);

        if row.len < min_len {
            return 0;
        }

        if row.len == min_len {
            while row.len > 0 && !damaged_ranges.is_empty() {
                let expected = damaged_ranges[0];
                if !row.starts_with_damaged(expected) {
                    return 0;
                }
                damaged_ranges = &damaged_ranges[1..];
                row.skip(expected);
                if damaged_ranges.is_empty() && row.len <= 0 {
                    break;
                }
                if let Some(false) = row.is_operational(0) {
                    return 0;
                } else {
                    row.skip(1);
                }
            }
            debug_assert_eq!(row.len, 0, "Full row should be consumed");
            debug_assert_eq!(damaged_ranges.len(), 0, "Full row should be consumed");
            return 1;
        }

        let damaged_prefix_len = row.get_damaged_prefix_len();

        if damaged_prefix_len > 0 {
            let first_range = damaged_ranges[0];
            if damaged_prefix_len > first_range {
                return 0;
            }
            if !row.starts_with_damaged(first_range) {
                return 0;
            }
            if let Some(false) = row.is_operational(first_range) {
                return 0;
            }
            row.skip(first_range + 1);
            damaged_ranges = &damaged_ranges[1..];
            min_len = get_min_len(damaged_ranges);
            if damaged_ranges.is_empty() {
                return if row.has_damaged() { 0 } else { 1 };
            }
        }

        let damaged_suffix_len = row.get_damaged_suffix_len();
        if damaged_suffix_len > 0 {
            let last_range = damaged_ranges[damaged_ranges.len() - 1];
            if damaged_suffix_len > last_range {
                return 0;
            }
            if !row.ends_with_damaged(last_range) {
                return 0;
            }
            if let Some(false) = row.is_operational(row.len - last_range - 1) {
                return 0;
            }
            row.skip_last(last_range + 1);
            damaged_ranges = &damaged_ranges[..damaged_ranges.len() - 1];
            min_len = get_min_len(damaged_ranges);
            if damaged_ranges.is_empty() {
                return if row.has_damaged() { 0 } else { 1 };
            }
        }

        if row.is_operational(0).is_none() && row.is_operational(row.len - 1).is_none() {
            break;
        }
    }

    if damaged_ranges.is_empty() {
        return if row.has_damaged() { 0 } else { 1 };
    }

    if damaged_ranges.len() == 1 {
        if row.has_damaged() {
            let range = damaged_ranges[0];
            if row.len < range {
                return 0;
            }

            let mut first_broken = (0..(row.len))
                .find(|i| row.is_operational(*i) == Some(false))
                .unwrap();
            let initial_last = (0..(row.len))
                .rev()
                .find(|i| row.is_operational(*i) == Some(false))
                .unwrap();

            if initial_last - first_broken + 1 > range {
                return 0;
            }

            if (first_broken..=initial_last).any(|i| row.operational.is_set(i)) {
                return 0;
            }
            let mut last = initial_last;

            while last < row.len - 1
                && !row.operational.is_set(last + 1)
                && last - first_broken + 1 < range
            {
                last += 1;
            }
            while first_broken > 0
                && !row.operational.is_set(first_broken - 1)
                && last - first_broken + 1 < range
            {
                first_broken -= 1;
            }

            if last - first_broken + 1 != range {
                return 0;
            }
            let mut res = 1;
            while first_broken > 0
                && !row.operational.is_set(first_broken - 1)
                && last - 1 >= initial_last
            {
                first_broken -= 1;
                last -= 1;
                res += 1;
            }
            return res;
        }
    }

    let min_len = damaged_ranges.iter().map(|r| r + 1).sum::<usize>() - 1;
    if row.len < min_len {
        return 0;
    }

    let prefix_unknown = row.get_unknown_prefix_len();

    debug_assert!(
        prefix_unknown > 0,
        "at least the first element should be unknown"
    );

    if prefix_unknown == 1 {
        let (operational, damaged) = row.with_super_position(0);

        return get_arrangements_number(operational, damaged_ranges)
            + get_arrangements_number(damaged, damaged_ranges);
    }

    if prefix_unknown == row.len {
        let res = count_all_arrangements(prefix_unknown, damaged_ranges);
        return res;
    }

    let suffix_unknown = row.get_unknown_sufix_len();

    if suffix_unknown == 1 {
        let (operational, damaged) = row.with_super_position(row.len - 1);

        return get_arrangements_number(operational, damaged_ranges)
            + get_arrangements_number(damaged, damaged_ranges);
    }

    let after_prefix_damaged = row.with_skip(prefix_unknown).get_damaged_prefix_len();
    if after_prefix_damaged == 0 {
        let mut res = 0;
        for i in 0..=damaged_ranges.len() {
            let arrangements = count_all_arrangements(prefix_unknown, &damaged_ranges[..i]);
            if arrangements == 0 {
                break;
            }
            res += arrangements
                * get_arrangements_number(row.with_skip(prefix_unknown + 1), &damaged_ranges[i..]);
        }
        return res;
    }

    let before_suffix_damaged = row.with_skip_last(suffix_unknown).get_damaged_suffix_len();
    if before_suffix_damaged == 0 {
        let mut res = 0;
        for i in (0..=damaged_ranges.len()).rev() {
            let arrangements = count_all_arrangements(suffix_unknown, &damaged_ranges[i..]);
            if arrangements == 0 {
                break;
            }
            res += arrangements
                * get_arrangements_number(
                    row.with_skip_last(suffix_unknown + 1),
                    &damaged_ranges[..i],
                );
        }
        return res;
    }

    let mut res = 0;
    for i in 0..damaged_ranges.len() {
        let range = damaged_ranges[i];
        if range < after_prefix_damaged {
            continue;
        }

        let prev_min_len = if i > 0 {
            get_min_len(&damaged_ranges[..i])
        } else {
            0
        };

        'position_loop: for j in (0..=prefix_unknown).rev() {
            if j < prev_min_len {
                continue;
            }
            if range - 1 + j < prefix_unknown {
                continue;
            }
            for k in j..(j + range) {
                if k >= row.len {
                    continue 'position_loop;
                }
                if row.operational.is_set(k) {
                    continue 'position_loop;
                }
            }
            if row.damaged.is_set(j + range) {
                continue;
            }
            let mut dbg_row = row.clone();
            for k in 0..range {
                dbg_row.damaged.set(j + k)
            }

            let arrangements = if j > 0 {
                count_all_arrangements(j - 1, &damaged_ranges[..i])
            } else {
                1
            };
            if arrangements == 0 {
                break;
            }
            res += arrangements
                * get_arrangements_number(row.with_skip(j + range + 1), &damaged_ranges[(i + 1)..])
        }
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
            assert_eq!(calculated, expected, "Line:\n  {line}")
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
    fn test_get_all_arrangements() {
        assert_eq!(count_all_arrangements(6, &[1, 2, 1]), 1)
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
            let calculated_fast = get_arrangements_number(row, &ranges);
            assert_eq!(calculated_fast, expected, "Line:\n  {line}");
        }
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "15454556629917");
    }
}
