use glam::UVec2;
use std::{
    collections::{BinaryHeap, VecDeque},
    ops::{Add, Sub},
};

use itertools::Itertools;
#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
}
impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Right => write!(f, "→"),
            Self::Down => write!(f, "↓"),
        }
    }
}

#[derive(Clone, Copy)]
enum Cell {
    Empty,
    Slope(Direction),
    Wall,
}
impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "."),
            Self::Slope(arg0) => write!(f, "{:?}", arg0),
            Self::Wall => write!(f, "#"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '#' => Ok(Self::Wall),
            '>' => Ok(Self::Slope(Direction::Right)),
            'v' => Ok(Self::Slope(Direction::Down)),
            _ => Err(format!("invalid cell: {}", value)),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn get(&self, pos: &glam::UVec2) -> Cell {
        self.cells
            .get(pos.y as usize)
            .and_then(|row| row.get(pos.x as usize))
            .copied()
            .unwrap_or(Cell::Wall)
    }

    fn available_without_slopes_neighbours(&self, last_point: &glam::UVec2) -> Vec<UVec2> {
        match self.get(last_point) {
            Cell::Empty | Cell::Slope(_) => {
                let mut result = Vec::new();
                for axis in UVec2::AXES {
                    if !matches!(self.get(&(last_point.add(axis))), Cell::Wall) {
                        result.push(last_point.add(axis));
                    }
                    let prev_pos = last_point.saturating_sub(axis);
                    if !prev_pos.eq(last_point) && !matches!(self.get(&prev_pos), Cell::Wall) {
                        result.push(last_point.sub(axis));
                    }
                }
                return result;
            }
            Cell::Wall => Vec::new(),
        }
    }
    fn available_neighbours(&self, last_point: &glam::UVec2) -> Vec<UVec2> {
        match self.get(last_point) {
            Cell::Empty => {
                let mut result = Vec::new();
                for axis in UVec2::AXES {
                    if !matches!(self.get(&(last_point.add(axis))), Cell::Wall) {
                        result.push(last_point.add(axis));
                    }
                    let prev_pos = last_point.saturating_sub(axis);
                    if !prev_pos.eq(last_point) && !matches!(self.get(&prev_pos), Cell::Wall) {
                        result.push(last_point.sub(axis));
                    }
                }
                return result;
            }
            Cell::Slope(Direction::Down) => {
                let mut result = Vec::new();
                if !matches!(self.get(&(last_point.add(UVec2::Y))), Cell::Wall) {
                    result.push(last_point.add(UVec2::Y));
                }
                return result;
            }
            Cell::Slope(Direction::Right) => {
                let mut result = Vec::new();
                if !matches!(self.get(&(last_point.add(UVec2::X))), Cell::Wall) {
                    result.push(last_point.add(UVec2::X));
                }
                return result;
            }
            Cell::Wall => Vec::new(),
        }
    }
}
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid: ")?;
        for row in &self.cells {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone)]
struct CellPath {
    start: glam::UVec2,
    end: glam::UVec2,
    length: usize,
}
impl CellPath {
    fn end(&self) -> glam::UVec2 {
        self.end.clone()
    }
    fn start(&self) -> glam::UVec2 {
        self.start.clone()
    }
    fn reversed(&self) -> Self {
        Self {
            start: self.end.clone(),
            end: self.start.clone(),
            length: self.length,
        }
    }
}

impl std::fmt::Debug for CellPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -{}-> {}", self.start, self.length, self.end)
    }
}

fn get_segments(
    grid: &Grid,
    start: UVec2,
    get_neighbours: impl Fn(&Grid, &UVec2) -> Vec<UVec2>,
) -> Vec<CellPath> {
    let mut paths = VecDeque::new();

    paths.push_back(Vec::from([start]));

    let mut segments = Vec::new();
    let mut visited_multiple_opportuinities = Vec::new();

    while let Some(mut path) = paths.pop_front() {
        'points_loop: loop {
            let last_point = &path[path.len() - 1];

            if visited_multiple_opportuinities.contains(last_point) {
                break;
            }

            let options = get_neighbours(&grid, last_point)
                .into_iter()
                .filter(|p| !path.iter().rev().contains(&p))
                .collect_vec();

            if options.len() == 1 {
                path.push(options[0]);
                continue 'points_loop;
            }

            if options.len() == 0 {
                break;
            }

            visited_multiple_opportuinities.push(last_point.clone());

            for opt in options {
                paths.push_back(vec![last_point.clone(), opt]);
            }
            break;
        }
        segments.push(path);
    }

    segments
        .into_iter()
        .map(|ps| CellPath {
            start: ps.first().copied().unwrap(),
            end: ps.last().copied().unwrap(),
            length: ps.len(),
        })
        .collect_vec()
}

#[derive(Clone, Debug, Hash, PartialEq, Eq, Copy)]
struct Bitmask {
    bits: [u64; 3],
}
impl Bitmask {
    fn new() -> Self {
        Self { bits: [0; 3] }
    }
    fn get_pos(&self, ind: usize) -> (usize, usize) {
        if ind >= 64 * 3 {
            unreachable!("too big index: {}", ind);
        }
        let times = ind >> 6;
        return (times, ind & 63);
    }
    fn insert(&mut self, ind: usize) {
        let (bit, ind) = self.get_pos(ind);
        self.bits[bit] |= 1 << ind;
    }
    fn with_inserted(&self, ind: usize) -> Self {
        let mut res = self.clone();
        res.insert(ind);
        res
    }
    fn contains(&self, ind: usize) -> bool {
        let (bit, ind) = self.get_pos(ind);
        self.bits[bit] & (1 << ind) != 0
    }
    fn iter(&self) -> impl Iterator<Item = usize> + '_ {
        let n = self.bits.len() * 64;
        (0..n).filter(move |i| self.contains(*i))
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Previous {
    segments: Bitmask,
    starts: Bitmask,
}
impl Previous {
    fn with_inserted(&self, current: &Current) -> Self {
        Self {
            segments: self.segments.with_inserted(current.segment),
            starts: self.starts.with_inserted(current.start),
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Current {
    segment: usize,
    start: usize,
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct ConcatenatedSegments {
    previous: Previous,
    current: Current,
    length: usize,
}

impl ConcatenatedSegments {
    fn new(segment: usize, current_start: usize, current_length: usize) -> Self {
        Self {
            previous: Previous {
                segments: Bitmask::new(),
                starts: Bitmask::new(),
            },
            current: Current {
                segment: segment,
                start: current_start,
            },
            length: current_length,
        }
    }
    fn segments_count(&self) -> usize {
        self.previous
            .segments
            .bits
            .iter()
            .fold(0usize, |acc, s| acc + s.count_ones() as usize)
            + 1
    }
    fn push(&self, next: usize, start: usize, next_len: usize) -> Self {
        Self {
            previous: self.previous.with_inserted(&self.current),
            current: Current {
                segment: next,
                start,
            },
            length: self.length + next_len,
        }
    }
    fn contains_segment(&self, ind: usize) -> bool {
        self.previous.segments.contains(ind) || self.current.segment == ind
    }
    fn contains_start(&self, ind: usize) -> bool {
        self.previous.starts.contains(ind) || self.current.start == ind
    }
}

impl std::cmp::Ord for ConcatenatedSegments {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.length.cmp(&other.length)
    }
}
impl std::cmp::PartialOrd for ConcatenatedSegments {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn can_reach_the_finish(
    next_segments: &[Vec<usize>],
    starts: &[usize],
    ends: &[usize],
    end_index: usize,
    segs: &ConcatenatedSegments,
) -> bool {
    let mut visited = Bitmask::new();
    let mut queue = VecDeque::from([segs.current.segment]);
    while let Some(seg_id) = queue.pop_front() {
        if visited.contains(seg_id) {
            continue;
        }
        if ends[seg_id] == end_index {
            return true;
        }
        visited.insert(seg_id);
        queue.extend(next_segments[seg_id].iter().filter(|next_id| {
            !visited.contains(**next_id)
                && !segs.contains_segment(**next_id)
                && !segs.contains_start(starts[**next_id])
        }));
    }
    false
}

fn solve(file_content: &str, get_segments: impl Fn(&Grid, UVec2) -> Vec<CellPath>) -> usize {
    let grid = Grid {
        cells: file_content
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| Cell::try_from(c).unwrap())
                    .collect_vec()
            })
            .collect_vec(),
    };

    let start = UVec2::new(1, 0);
    let end = UVec2::new(grid.cells[0].len() as u32 - 2, grid.cells.len() as u32 - 1);

    let mut segments = get_segments(&grid, start);

    segments.sort_unstable_by_key(|f| f.length);
    segments.reverse();

    let points = segments
        .iter()
        .flat_map(|s| [s.start(), s.end()])
        .unique()
        .sorted_by(|a, b| a.y.cmp(&b.y).then(a.x.cmp(&b.x)))
        .collect_vec();

    let end_index = points.iter().position(|p| p.eq(&end)).unwrap();

    let starts = segments
        .iter()
        .map(|s| {
            let start = s.start();
            points.iter().position(|p| p.eq(&start)).unwrap()
        })
        .collect_vec();
    let ends = segments
        .iter()
        .map(|s| {
            let end = s.end();
            points.iter().position(|p| p.eq(&end)).unwrap()
        })
        .collect_vec();

    let next_segments = (0..segments.len())
        .map(|from_id| {
            (0..segments.len())
                .filter(|to_id| *to_id != from_id && ends[from_id] == starts[*to_id])
                .collect_vec()
        })
        .collect_vec();

    let mut ends_lists = BinaryHeap::new();

    ends_lists.extend(
        (0..segments.len())
            .filter(|id| segments[*id].start() == start)
            .map(|id| ConcatenatedSegments::new(id, starts[id], segments[id].length - 1)),
    );

    let mut max_finish_len = 0usize;
    while let Some(segs) = ends_lists.pop() {
        if segments[segs.current.segment].end.eq(&end) {
            let len = segs.length;
            if len > max_finish_len {
                max_finish_len = len;
            }
            continue;
        }
        ends_lists.extend(
            next_segments[segs.current.segment]
                .iter()
                .filter(|next_segment_id| {
                    !segs.contains_segment(**next_segment_id)
                        && !segs.contains_start(ends[**next_segment_id])
                })
                .copied()
                .map(|next_id| segs.push(next_id, starts[next_id], segments[next_id].length - 1))
                .filter(|segs| {
                    ends[segs.current.segment] == end_index
                        || can_reach_the_finish(&next_segments, &starts, &ends, end_index, segs)
                }),
        );
    }

    max_finish_len
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, |grid, start| {
        get_segments(grid, start, Grid::available_neighbours)
    })
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, |grid, start| {
        get_segments(grid, start, Grid::available_without_slopes_neighbours)
            .into_iter()
            .flat_map(|p| [p.reversed(), p].into_iter().rev())
            .collect()
    })
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "94");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "2018");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "154");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "6406");
    }
}
