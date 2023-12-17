use std::collections::{BinaryHeap, VecDeque};

use itertools::Itertools;

struct Grid {
    loses: Vec<Vec<usize>>,
}

impl Grid {
    fn get(&self, row: usize, col: usize) -> usize {
        if row >= self.rows() {
            return usize::MAX;
        }
        if col >= self.cols() {
            return usize::MAX;
        }
        self.loses[row][col]
    }
    fn cols(&self) -> usize {
        self.loses[0].len()
    }
    fn rows(&self) -> usize {
        self.loses.len()
    }
    fn dimensions(&self) -> (usize, usize) {
        (self.rows(), self.cols())
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.loses {
            for col in row {
                write!(f, "{}", col)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Visited {
    visited: Vec<Vec<[usize; 4]>>,
}
impl Visited {
    fn new(rows: usize, cols: usize) -> Self {
        let visited = vec![vec![[usize::MAX; 4]; cols]; rows];
        Self { visited }
    }

    fn set_min(&mut self, row: usize, col: usize, direction: Direction, cost: usize) -> bool {
        if cost <= self.get(row, col, direction) {
            self.visited[row][col][usize::from(direction)] = cost;
            true
        } else {
            false
        }
    }

    fn get_min(&mut self, row: usize, col: usize) -> usize {
        self.visited[row][col]
            .iter()
            .min()
            .unwrap_or(&usize::MAX)
            .to_owned()
    }

    fn get(&mut self, row: usize, col: usize, direction: Direction) -> usize {
        self.visited[row][col][usize::from(direction)]
    }
}

impl std::fmt::Debug for Visited {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = f.width().unwrap_or(3);
        for row in &self.visited {
            'cell: for cell in row {
                let m = *cell.iter().min().unwrap_or(&usize::MAX);
                for d in Direction::iter() {
                    if cell[usize::from(d)] == m {
                        if m == usize::MAX {
                            write!(f, " {:<w$}", "X")?;
                        } else {
                            write!(f, "{}{:<w$}", d, m)?;
                        }
                        continue 'cell;
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Direction::Up => write!(f, "↑"),
            Direction::Right => write!(f, "→"),
            Direction::Down => write!(f, "↓"),
            Direction::Left => write!(f, "←"),
        }
    }
}

impl Direction {
    fn iter() -> impl Iterator<Item = Self> {
        [
            Direction::Up,
            Direction::Right,
            Direction::Down,
            Direction::Left,
        ]
        .into_iter()
    }
}

impl From<Direction> for usize {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 0,
            Direction::Right => 1,
            Direction::Down => 2,
            Direction::Left => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step {
    row: usize,
    col: usize,
    cost: usize,
    h: usize,
    direction: Direction,
}

impl std::cmp::Ord for Step {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let sum = self.cost + self.h;
        let other_sum = other.cost + other.h;
        other_sum.cmp(&sum)
    }
}
impl std::cmp::PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn print_steps<'t>(steps: impl Iterator<Item = &'t Step>, rows: usize, col: usize) {
    let mut visited = Visited::new(rows, col);
    for step in steps {
        visited.set_min(step.row, step.col, step.direction, step.cost);
        for d in Direction::iter() {}
    }
    println!("{:3?}", visited);
}

fn manhattan(row: usize, col: usize, target_row: usize, target_col: usize) -> usize {
    let mut res = 0;
    if row > target_row {
        res += row - target_row;
    } else {
        res += target_row - row;
    }

    if col > target_col {
        res += col - target_col;
    } else {
        res += target_col - col;
    }

    res
}

pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let (rows, cols) = grid.dimensions();

    let mut visited = Visited::new(rows, cols);

    let start_row = 0;
    let start_col = 0;
    let target_row = rows - 1;
    let target_col = cols - 1;

    let mut discovered = 0;
    let target_path_points = [
        (0, 1, Direction::Right, 4),
        (0, 2, Direction::Right, 5),
        (1, 2, Direction::Down, 6),
        (1, 3, Direction::Right, 11),
        (1, 4, Direction::Right, 15),
        (1, 5, Direction::Right, 20),
        (0, 5, Direction::Up, 23),
        (0, 6, Direction::Right, 25),
        (0, 7, Direction::Right, 28),
        (0, 8, Direction::Right, 29),
        (1, 8, Direction::Down, 32),
        (2, 8, Direction::Down, 37),
        (2, 9, Direction::Right, 41),
        (2, 10, Direction::Right, 43),
        (3, 10, Direction::Down, 47),
        (4, 10, Direction::Down, 52),
        (4, 11, Direction::Right, 55),
        (5, 11, Direction::Down, 60),
        (6, 11, Direction::Down, 66),
        (7, 11, Direction::Down, 71),
        (7, 12, Direction::Right, 74),
        (8, 12, Direction::Down, 81),
        (9, 12, Direction::Down, 84),
        (10, 12, Direction::Down, 87),
        (10, 11, Direction::Left, 93),
        (11, 11, Direction::Down, 96),
        (12, 11, Direction::Down, 99),
        (12, 12, Direction::Right, 102),
    ];

    for d in Direction::iter() {
        visited.set_min(start_row, start_col, d, grid.get(start_row, start_col));
    }

    let mut steps = BinaryHeap::new();

    for d in 1..=3 {
        // steps to the right
        steps.push(Step {
            row: start_row,
            col: start_col + d,
            cost: (0..=(start_col + d)).map(|c| grid.get(start_row, c)).sum(),
            direction: Direction::Right,
            h: manhattan(start_row, start_col + d, target_row, target_col),
        });
        // steps to the bottom
        steps.push(Step {
            row: start_row + d,
            col: start_col,
            cost: (0..=(start_row + d)).map(|r| grid.get(r, start_col)).sum(),
            direction: Direction::Down,
            h: manhattan(start_row + d, start_col, target_row, target_col),
        });
    }

    println!("{:#?}", steps);

    while let Some(Step {
        row,
        col,
        cost,
        direction,
        ..
    }) = steps.pop()
    {
        if target_path_points.contains(&(row, col, direction)) {
            println!(
                "Found target at {row},{col}, last move {:?}. Cost = {cost}",
                direction
            );
            println!();
        }

        if !visited.set_min(row, col, direction, cost) {
            continue;
        }

        if !matches!(direction, Direction::Right) {
            let mut collected_cost = cost;
            for c in col + 1..cols.min(col + 4) {
                collected_cost = collected_cost.saturating_add(grid.get(row, c));
                steps.push(Step {
                    row,
                    col: c,
                    cost: collected_cost,
                    h: manhattan(row, c, target_row, target_col),
                    direction: Direction::Right,
                })
            }
        }

        if !matches!(direction, Direction::Left) {
            let mut collected_cost = cost;
            for c in (col.saturating_sub(3)..col).rev() {
                collected_cost = collected_cost.saturating_add(grid.get(row, c));
                steps.push(Step {
                    row,
                    col: c,
                    cost: collected_cost,
                    h: manhattan(row, c, target_row, target_col),
                    direction: Direction::Left,
                })
            }
        }
        if !matches!(direction, Direction::Down) {
            let mut collected_cost = cost;
            for r in (row + 1)..rows.min(row + 4) {
                collected_cost = collected_cost.saturating_add(grid.get(r, col));
                steps.push(Step {
                    row: r,
                    col,
                    h: manhattan(r, col, target_row, target_col),
                    cost: collected_cost,
                    direction: Direction::Down,
                })
            }
        }

        if !matches!(direction, Direction::Up) {
            let mut collected_cost = cost;
            for r in (row.saturating_sub(3)..row).rev() {
                collected_cost = collected_cost.saturating_add(grid.get(r, col));
                steps.push(Step {
                    row: r,
                    col,
                    h: manhattan(r, col, target_row, target_col),
                    cost: collected_cost,
                    direction: Direction::Up,
                })
            }
        }

        if target_path_points.contains(&(row, col, direction)) {
            println!(
                "Found target at {},{}, last move {:?}. Cost = {cost}",
                row, col, direction
            );
            println!("Visited:");
            print!("{:3?}", &visited);
            println!("Steps:");
            print_steps(steps.iter(), rows, cols);
            println!();
        }
    }
    visited.get_min(rows - 1, cols - 1)
}
pub fn solve_task2(_file_content: &str) -> usize {
    0
}

fn parse_grid(input: &str) -> Grid {
    let loses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as usize)
                .collect::<Vec<_>>()
        })
        .collect_vec();

    Grid { loses }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d17/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d17.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "102");
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
