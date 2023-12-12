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

    fn skip(&mut self, from: usize) {
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

    fn get_max_damaged_slice_len(&self) -> usize {
        let mut res = 0;
        let mut cur = 0;
        for i in 0..self.len {
            if let Some(false) = self.is_operational(i) {
                cur += 1;
                if cur > res {
                    res = cur;
                }
            } else {
                cur = 0;
            }
        }

        res
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
    let rest_min_len = get_min_len(&ranges[1..]) + 1;

    let free_positions = row_len + 1 - rest_min_len - ranges[0] + 1;

    return (0..free_positions)
        .map(|i| count_all_arrangements(row_len - i - 1 - ranges[0], &ranges[1..]))
        .sum();
}

// fn get_arrangements_number_fast(row: Row, damaged_ranges: &[usize]) -> usize {
//     let res = _get_arrangements_number_fast(row, damaged_ranges);
//     let expected = get_arrangements_number(row, damaged_ranges);
//     assert_eq!(
//         res, expected,
//         "Fast and slow algorithms should match:n=\n  \"{row:?}\", {damaged_ranges:?}  \n"
//     );
//     // println!(
//     //     "get_arrangements_number_fast({:?}, {:?}) = {}",
//     //     row, damaged_ranges, res
//     // );
//     res
// }
fn get_arrangements_number_fast(mut row: Row, mut damaged_ranges: &[usize]) -> usize {
    loop {
        if damaged_ranges.is_empty() {
            if row.has_damaged() {
                return 0;
            } else {
                return 1;
            }
        }

        while row.len > 0 && row.is_operational(0).unwrap_or(false) {
            row.skip(1)
        }
        while row.len > 0 && row.is_operational(row.len - 1).unwrap_or(false) {
            row.skip_last(1)
        }

        let min_len = damaged_ranges.iter().map(|r| r + 1).sum::<usize>() - 1;
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

        if let Some(false) = row.is_operational(0) {
            let expected = damaged_ranges[0];
            if !row.starts_with_damaged(expected) {
                return 0;
            }
            damaged_ranges = &damaged_ranges[1..];
            row.skip(expected);

            if damaged_ranges.is_empty() {
                if row.has_damaged() {
                    return 0;
                } else {
                    return 1;
                }
            }

            match row.is_operational(0) {
                Some(false) => {
                    return 0;
                }
                _ => {
                    row.skip(1);
                }
            }
        }

        if row.len > 0 {
            if let Some(false) = row.is_operational(row.len - 1) {
                let expected = damaged_ranges[damaged_ranges.len() - 1];
                if !row.ends_with_damaged(expected) {
                    return 0;
                }
                damaged_ranges = &damaged_ranges[..damaged_ranges.len() - 1];
                row.skip_last(expected);

                if damaged_ranges.is_empty() {
                    if row.has_damaged() {
                        return 0;
                    } else {
                        return 1;
                    }
                }

                match row.is_operational(row.len - 1) {
                    Some(false) => {
                        return 0;
                    }
                    _ => {
                        row.skip_last(1);
                    }
                }
            }
        }

        if row.is_operational(0).is_none() {
            break;
        }
    }

    // println!("r = {:?}, d = {:?}", row, damaged_ranges);

    let min_len = damaged_ranges.iter().map(|r| r + 1).sum::<usize>() - 1;
    if row.len < min_len {
        return 0;
    }

    let unknown_cnt = (0..(row.len))
        .into_iter()
        .take_while(|i| row.is_operational(*i).is_none())
        .count();

    debug_assert!(
        unknown_cnt > 0,
        "at least the first element should be unknown"
    );

    if unknown_cnt == 1 {
        let (operational, damaged) = row.with_super_position(0);

        return get_arrangements_number_fast(operational, damaged_ranges)
            + get_arrangements_number_fast(damaged, damaged_ranges);
    }

    if unknown_cnt == row.len {
        let res = count_all_arrangements(unknown_cnt, damaged_ranges);
        return res;
    }

    let next_is_operational = row.is_operational(unknown_cnt).unwrap_or(false);
    if next_is_operational {
        let mut res = 0;
        for i in 0..=damaged_ranges.len() {
            let arrangements = count_all_arrangements(unknown_cnt, &damaged_ranges[..i]);
            if arrangements == 0 {
                break;
            }
            // println!(
            //     "{:?} in {}  arrangements = {}",
            //     &damaged_ranges[..i],
            //     "?".repeat(unknown_cnt),
            //     arrangements
            // );
            res += arrangements
                * get_arrangements_number_fast(
                    row.with_skip(unknown_cnt + 1),
                    &damaged_ranges[i..],
                );
        }
        return res;
    }

    let max_r = damaged_ranges.iter().max().copied().unwrap();
    let max_damaged = row.get_max_damaged_slice_len();
    if max_damaged > max_r {
        return 0;
    }

    let (operational, damaged) = row.with_super_position(unknown_cnt - 1);
    return get_arrangements_number_fast(operational, &damaged_ranges)
        + get_arrangements_number_fast(damaged, &damaged_ranges);
}

fn get_arrangements_number(states: Row, damaged_ranges: &[usize]) -> usize {
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
        let expected_len = damaged_ranges[damaged_range_index];
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
                        tasks.push((row.with_skip(i + 1), damaged_range_index));
                        continue 'to_next_task;
                    }
                    if damaged_count != expected_len {
                        continue 'to_next_task;
                    }
                    tasks.push((row.with_skip(i + 1), damaged_range_index + 1));
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
                        tasks.push((row.with_skip(i + 1), damaged_range_index + 1));
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
        .map(|(states, ranges)| get_arrangements_number_fast(states, &ranges))
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
        .enumerate()
        .map(|(i, (states, ranges))| {
            println!("{i}");
            get_arrangements_number_fast(states, &ranges)
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
            let calculated = get_arrangements_number(row, &ranges);
            let calculated_fast = get_arrangements_number_fast(row, &ranges);
            assert_eq!(calculated_fast, expected, "Line:\n  {line}");
            assert_eq!(calculated, expected, "Line:\n  {line}")
        }
    }

    #[test]
    fn temporal_test() {
        let line = "..##????#? 4,1,1";
        let (row, ranges) = parse_line(line);
        let calculated = get_arrangements_number(row, &ranges);
        let calculated_fast = get_arrangements_number_fast(row, &ranges);
        assert_eq!(calculated, calculated_fast, "Line:\n  {line}");
    }

    #[test]
    fn test_get_all_arrangements() {
        assert_eq!(count_all_arrangements(6, &[1, 2, 1]), 1)
    }
    #[test]
    fn test_task1_actual() {
        for line in ACTUAL.lines() {
            println!("\n{line}\n");
            let (row, ranges) = parse_line(line);
            let calculated = get_arrangements_number(row, &ranges);
            let calculated_fast = get_arrangements_number_fast(row, &ranges);
            assert_eq!(calculated_fast, calculated, "Line:\n  {line}");
        }
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "7599");
    }

    #[test]
    fn test_task2() {
        for (line, expected) in INPUT.lines().zip([1, 16384, 1, 16, 2500, 506250usize]) {
            let unfolded = unfold_line(line);
            let (row, ranges) = parse_line(&unfolded);
            let calculated = get_arrangements_number(row, &ranges);
            let calculated_fast = get_arrangements_number_fast(row, &ranges);
            assert_eq!(calculated_fast, expected, "Line:\n  {line}");
            assert_eq!(calculated, expected, "Line:\n  {line}");
        }
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
