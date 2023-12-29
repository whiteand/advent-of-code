use glam::UVec2;
use std::{
    collections::{HashMap, VecDeque},
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
    points: Vec<glam::UVec2>,
}
impl CellPath {
    fn last(&self) -> glam::UVec2 {
        self.points.last().cloned().unwrap()
    }
    fn first(&self) -> glam::UVec2 {
        self.points.first().cloned().unwrap()
    }
    fn reversed(&self) -> Self {
        Self {
            points: self.points.iter().rev().cloned().collect_vec(),
        }
    }
}

impl std::fmt::Debug for CellPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.points.first().unwrap())?;
        for point in &self.points[1..] {
            write!(f, "->{}", point)?;
        }
        write!(f, ")")
    }
}

fn get_no_choicers(
    grid: &Grid,
    start: UVec2,
    get_neighbours: impl Fn(&Grid, &UVec2) -> Vec<UVec2>,
) -> Vec<CellPath> {
    let mut paths = VecDeque::new();

    paths.push_back(CellPath {
        points: Vec::from([start]),
    });

    let mut no_choicers = Vec::new();
    let mut visited_multiple_opportuinities = Vec::new();

    while let Some(mut path) = paths.pop_front() {
        'points_loop: loop {
            let last_point = &path.points[path.points.len() - 1];

            if visited_multiple_opportuinities.contains(last_point) {
                break;
            }

            let options = get_neighbours(&grid, last_point)
                .into_iter()
                .filter(|p| !path.points.iter().rev().contains(&p))
                .collect_vec();

            if options.len() == 1 {
                path.points.push(options[0]);
                continue 'points_loop;
            }

            if options.len() == 0 {
                break;
            }

            visited_multiple_opportuinities.push(last_point.clone());

            for opt in options {
                let new_path = CellPath {
                    points: vec![last_point.clone(), opt],
                };
                paths.push_back(new_path);
            }
            break;
        }
        no_choicers.push(path);
    }

    no_choicers
}

fn solve(file_content: &str, get_no_choicers: impl Fn(&Grid, UVec2) -> Vec<CellPath>) -> usize {
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

    let mut no_choicers: HashMap<UVec2, Vec<CellPath>> = HashMap::new();

    for no_choicer in get_no_choicers(&grid, start) {
        let start = no_choicer.first();
        no_choicers.entry(start).or_default().push(no_choicer);
    }

    let mut ends_lists = VecDeque::new();

    for p in no_choicers.get(&start).into_iter().flatten() {
        ends_lists.push_back(vec![(p.first(), p.last())])
    }

    let mut finished_paths = Vec::new();

    let no_choicers_len = no_choicers.values().flatten().count();
    println!("No choicers len: {}", no_choicers_len);
    for no_choicer in no_choicers.values().flatten() {
        println!("{:?}", &no_choicer.points.len());
    }

    while let Some(ends_list) = ends_lists.pop_front() {
        let last_end = ends_list.last().cloned().unwrap().1;

        for next in no_choicers
            .get(&last_end)
            .into_iter()
            .flatten()
            .filter(|p| !ends_list.iter().map(|p| p.0).contains(&p.last()))
        {
            let new_pair = (last_end, next.last());
            let new_ends_list = ends_list
                .iter()
                .cloned()
                .chain(std::iter::once(new_pair))
                .collect_vec();
            ends_lists.push_back(new_ends_list);
        }

        if last_end == end {
            finished_paths.push(ends_list);
        }
    }

    finished_paths
        .into_iter()
        .map(|p| {
            p.into_iter()
                .map(|(s, e)| {
                    let no_choicer = no_choicers
                        .get(&s)
                        .unwrap()
                        .iter()
                        .find(|p| p.last() == e)
                        .unwrap();
                    no_choicer.points.len() - 1
                })
                .sum::<usize>()
        })
        .max()
        .unwrap_or_default()
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, |grid, start| {
        get_no_choicers(grid, start, Grid::available_neighbours)
    })
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, |grid, start| {
        get_no_choicers(grid, start, Grid::available_without_slopes_neighbours)
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
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
