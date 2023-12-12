use std::{cmp::Ordering, ops::Range};

#[derive(Clone, Copy, PartialEq, Eq)]
struct Bitmask {
    bits: usize,
    len: usize,
}
impl std::ops::Deref for Bitmask {
    type Target = usize;
    fn deref(&self) -> &Self::Target {
        &self.bits
    }
}
impl std::fmt::Debug for Bitmask {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len {
            if self.is_set(i as usize) {
                write!(f, "1")?;
            } else {
                write!(f, "0")?;
            }
            if i % 4 == 3 {
                write!(f, " ")?;
            }
        }
        Ok(())
    }
}

impl Bitmask {
    fn new(len: usize) -> Self {
        Self { bits: 0, len }
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
    fn slice(&self, from: usize, to: usize) -> Self {
        Self {
            bits: (self.bits >> from) & (1 << (to - from)) - 1,
            len: to - from,
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
        for i in 0..len {
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
        res.operational.set(i);
        res
    }
    fn with_damaged(&self, i: usize) -> Self {
        let mut res = self.clone();
        res.damaged.set(i);
        res
    }
    fn super_position(&self, i: usize) -> (Self, Self) {
        (self.with_operational(i), self.with_damaged(i))
    }
}

#[derive(Debug)]
struct Task {
    states_range: Range<usize>,
    ranges_range: Range<usize>,
    is_broken: Bitmask,
    is_operational: Bitmask,
}

fn get_arangements_number(states: Vec<State>, damaged_ranges: Vec<usize>) -> usize {
    let mut tasks = vec![Task {
        states_range: 0..states.len(),
        ranges_range: 0..damaged_ranges.len(),
        is_broken: Bitmask::new(states.len()),
        is_operational: Bitmask::new(states.len()),
    }];
    let mut res = 0;

    'to_next_task: while let Some(Task {
        states_range,
        ranges_range,
        mut is_broken,
        mut is_operational,
    }) = tasks.pop()
    {
        // This macro takes formatted
        macro_rules! not_finished {
            ($($arg:tt)*) => {
                todo!(
                    "\n{:?}\n{:?}\n{:#?}\n  Cannot handle state: {:?}",
                    states,
                    damaged_ranges,
                    Task {
                        states_range: states_range.clone(),
                        ranges_range: ranges_range.clone(),
                        is_broken: is_broken,
                        is_operational: is_operational,
                    },
                    format!($($arg)*)
                )
            };
        }
        macro_rules! pr {
            ($($arg:tt)*) => {
                println!(
                    "\n{:?}\n{:?}\n{:?}\n{:#?}\n  {:?}",
                    states.clone().into_iter().enumerate().filter(|(i, _)| states_range.contains(i))
                        .map(|(_, s)| s).collect::<Vec<_>>(),
                    states.clone().into_iter()
                        .enumerate().filter(|(i, _)| states_range.contains(i)).map(|(i, v)| if is_broken.is_set(i) {
                            State::Damaged
                        } else if is_operational.is_set(i) {
                            State::Operational
                        } else {
                            v
                        }).collect::<Vec<_>>(),
                    damaged_ranges.clone().into_iter().enumerate().filter(|(i, _)| ranges_range.contains(i)).map(|(_, v)| v).collect::<Vec<_>>(),
                    Task {
                        states_range: states_range.clone(),
                        ranges_range: ranges_range.clone(),
                        is_broken: is_broken,
                        is_operational: is_operational,
                    },
                    format!($($arg)*)
                )
            };
        }
        pr!("New task");
        if states_range.len() == 0 && ranges_range.len() == 0 {
            res += 1;
            continue;
        } else if states_range.len() == 0 {
            continue;
        } else if ranges_range.len() == 0 {
            not_finished!("ranges_range.len() == 0")
        }
        let mut current_streak = 0;
        let current_damage_range = damaged_ranges.get(ranges_range.start);
        for i in states_range.clone() {
            let state = match (is_broken.is_set(i), is_operational.is_set(i), states[i]) {
                (true, true, State::Unknown) => unreachable!(),
                (true, false, State::Unknown) => State::Damaged,
                (false, true, State::Unknown) => State::Operational,
                (false, false, State::Unknown) => State::Unknown,
                (_, true, State::Damaged) => continue 'to_next_task,
                (true, _, State::Operational) => unreachable!(),
                (_, _, v) => v,
            };
            println!("State {state:?}. Current Streak: {current_streak}. Current Range: {current_damage_range:?}");
            match state {
                State::Operational => {
                    if current_streak > 0 {
                        match current_damage_range {
                            Some(&r) => {
                                if current_streak == r {
                                    tasks.push(Task {
                                        states_range: (i + 1)..states_range.end,
                                        ranges_range: (ranges_range.start + 1)..ranges_range.end,
                                        is_broken: is_broken,
                                        is_operational: is_operational,
                                    });
                                    continue 'to_next_task;
                                }
                                if current_streak < r {
                                    continue 'to_next_task;
                                }
                                not_finished!(
                                    "i: {i}\ncurrent_streak > 0: {current_streak}\n  with current range {r}"
                                )
                            }
                            None => continue 'to_next_task,
                        }
                    }
                    current_streak = 0;
                }
                State::Damaged => match current_damage_range {
                    Some(r) => {
                        if *r <= current_streak {
                            continue 'to_next_task;
                        }
                        current_streak += 1;
                    }
                    None => {
                        continue 'to_next_task;
                    }
                },
                State::Unknown => match current_damage_range {
                    Some(r) => match current_streak.cmp(r) {
                        Ordering::Less => {
                            if current_streak > 0 {
                                is_broken.set(i);
                                continue;
                            }
                            tasks.push(Task {
                                states_range: states_range.clone(),
                                ranges_range: ranges_range.clone(),
                                is_broken: is_broken,
                                is_operational: is_operational.with_set(i),
                            });
                            tasks.push(Task {
                                states_range: states_range.clone(),
                                ranges_range: ranges_range.clone(),
                                is_broken: is_broken.with_set(i),
                                is_operational: is_operational,
                            });
                            continue 'to_next_task;
                        }
                        Ordering::Equal => {
                            tasks.push(Task {
                                states_range: i..states_range.end,
                                ranges_range: (ranges_range.start + 1)..ranges_range.end,
                                is_broken: is_broken,
                                is_operational: is_operational.with_set(i),
                            });
                            continue 'to_next_task;
                        }
                        Ordering::Greater => {
                            not_finished!("current_streak > current_damage_range",)
                        }
                    },
                    None => {
                        not_finished!("Current damage range does not exists")
                    }
                },
            }
        }
        match (current_streak.cmp(&0), current_damage_range) {
            (Ordering::Greater, Some(&r)) if r == current_streak => {
                res += 1;
                continue 'to_next_task;
            }
            (Ordering::Greater, Some(&r)) if r > current_streak => {
                continue 'to_next_task;
            }
            v => not_finished!("current streak: {current_streak}\n{v:?}"),
        }
    }
    res
}

fn parse_line(line: &str) -> (Vec<State>, Vec<usize>) {
    let (left, right) = line.split_once(' ').unwrap();
    let states = left
        .chars()
        .map(|c| match c {
            '#' => State::Damaged,
            '.' => State::Operational,
            '?' => State::Unknown,
            v => unreachable!("There is no such state as {v}"),
        })
        .collect::<Vec<_>>();

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
        .map(|(states, ranges)| get_arangements_number(states, ranges))
        .sum()
}
pub fn solve_task2(_file_content: &str) -> usize {
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d12/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d12.txt");

    #[test]
    fn test_task1() {
        let (states, ranges) = parse_line("?#?#?#?#?#?#?#? 1,3,1,6");
        let calculated = get_arangements_number(states, ranges);
        assert_eq!(calculated, 1);
        // for (line, expected) in INPUT.lines().zip([1, 4, 1, 1, 4, 10usize]) {
        //     let (states, ranges) = parse_line(line);
        //     let calculated = get_arangements_number(states, ranges);
        //     assert_eq!(calculated, expected, "Line:\n  {line}")
        // }
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "0");
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
