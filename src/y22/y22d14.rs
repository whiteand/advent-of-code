use itertools::Itertools;
use std::{cell::RefCell, collections::BTreeMap, ops::RangeInclusive};

use nom::IResult;

#[derive(Debug)]
enum Unit {
    Sand,
    Wall,
}

pub fn solve_task1(file_content: &str) -> impl std::fmt::Display {
    const SOURCE: (i32, i32) = (500, 0);
    let (map, y_range) = parse_map(file_content);
    let map_ref_cell = RefCell::new(map);
    let mut i: usize = 0;
    let can_move = |x, y| !map_ref_cell.borrow().contains_key(&(x, y));
    let should_stop = |_, y| y > *y_range.end();
    loop {
        i += 1;
        match find_rest_sand_position(SOURCE, can_move, should_stop) {
            None => return i - 1,
            Some((x, y)) => {
                map_ref_cell.borrow_mut().insert((x, y), Unit::Sand);
            }
        }
    }
}

pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    const SOURCE: (i32, i32) = (500, 0);
    let (map, y_range) = parse_map(file_content);
    let map_ref_cell = RefCell::new(map);
    let mut i: usize = 0;
    let can_move = |x, y| {
        if y >= y_range.end() + 2 {
            return false;
        }
        !map_ref_cell.borrow().contains_key(&(x, y))
    };
    let should_stop = |_, _| false;
    loop {
        i += 1;
        match find_rest_sand_position(SOURCE, can_move, should_stop) {
            None => unreachable!(),
            Some((x, y)) => {
                if (x, y) == SOURCE {
                    return i;
                }
                map_ref_cell.borrow_mut().insert((x, y), Unit::Sand);
            }
        }
    }
}

fn find_rest_sand_position(
    source: (i32, i32),
    can_move: impl Fn(i32, i32) -> bool,
    should_stop: impl Fn(i32, i32) -> bool,
) -> Option<(i32, i32)> {
    let (mut x, mut y) = source;
    loop {
        let prev_y = y;
        for (next_x, next_y) in [(x, y + 1), (x - 1, y + 1), (x + 1, y + 1)] {
            if !can_move(next_x, next_y) {
                continue;
            }

            x = next_x;
            y = next_y;

            if should_stop(x, y) {
                return None;
            }
            break;
        }
        if y == prev_y {
            return Some((x, y));
        }
    }
}

fn parse_map(file_content: &str) -> (BTreeMap<(i32, i32), Unit>, RangeInclusive<i32>) {
    let wall_coordinates = parse(file_content).flat_map(|path| {
        path.into_iter()
            .tuple_windows::<(_, _)>()
            .flat_map(|((x1, y1), (x2, y2))| {
                if x1 == x2 {
                    (y1.min(y2)..=y2.max(y1))
                        .map(|y| (x1, y))
                        .collect::<Vec<_>>()
                } else {
                    (x1.min(x2)..=x1.max(x2))
                        .map(|x| (x, y1))
                        .collect::<Vec<_>>()
                }
            })
    });

    let mut map: BTreeMap<(i32, i32), Unit> = BTreeMap::new();
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;
    for (x, y) in wall_coordinates {
        map.insert((x, y), Unit::Wall);
        min_y = min_y.min(y);
        max_y = max_y.max(y);
    }
    (map, min_y..=max_y)
}

fn parse(file_content: &str) -> impl Iterator<Item = Vec<(i32, i32)>> + '_ {
    file_content.lines().map(|line| parse_path(line).unwrap().1)
}

fn parse_path(line: &str) -> IResult<&str, Vec<(i32, i32)>> {
    nom::multi::separated_list1(
        nom::bytes::complete::tag(" -> "),
        nom::sequence::separated_pair(
            nom::character::complete::i32,
            nom::bytes::complete::tag(","),
            nom::character::complete::i32,
        ),
    )(line)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";
    const ACTUAL: &str = include_str!("../../benches/y22/y22d14.txt");
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "24");
    }

    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "1406");
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "93");
    }

    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "20870");
    }
}
