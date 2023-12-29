use glam::UVec2;
use std::{
    collections::VecDeque,
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
impl std::fmt::Debug for CellPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}", self.points.first().unwrap())?;
        for point in &self.points[1..] {
            write!(f, "->{}", point)?;
        }
        write!(f, ")")
    }
}

// 551 = 47 * 71
// 20022 = 3 * 2 * 47 * 71

pub fn solve_part_1(file_content: &str) -> usize {
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
    let end = UVec2::new(
        (grid.cells[0].len() - 2)
            .try_into()
            .expect("usize should be enough to represent cell coordinates"),
        (grid.cells.len() - 1)
            .try_into()
            .expect("usize should be enough to represent cell coordinates"),
    );

    let mut paths = VecDeque::new();

    paths.push_back(CellPath {
        points: Vec::from([start]),
    });

    let mut finished_paths = Vec::new();

    'path_loop: while let Some(mut path) = paths.pop_front() {
        'points_loop: loop {
            let options = {
                let last_point = &path.points[path.points.len() - 1];

                grid.available_neighbours(last_point)
                    .into_iter()
                    .filter(|p| !path.points.iter().rev().contains(&p))
                    .collect_vec()
            };

            if options.len() == 0 {
                break;
            }
            if options.len() == 1 {
                path.points.push(options[0]);
                continue 'points_loop;
            }
            for opt in options {
                let mut new_path = path.clone();
                new_path.points.push(opt);
                paths.push_back(new_path);
            }
            continue 'path_loop;
        }
        finished_paths.push(path);
    }

    finished_paths
        .into_iter()
        .filter(|p| p.points.last().unwrap().eq(&end))
        .map(|p| p.points.len() - 1)
        .max()
        .unwrap_or_default()
}
pub fn solve_part_2(_file_content: &str) -> usize {
    todo!("part 2 is not implemented yet")
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
    #[ignore]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "154");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
