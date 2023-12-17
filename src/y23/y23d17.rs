use std::collections::BinaryHeap;

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

    for d in Direction::iter() {
        visited.set_min(start_row, start_col, d, 0);
    }

    let mut steps = BinaryHeap::new();

    for d in 1..=3 {
        // steps to the right
        steps.push(Step {
            row: start_row,
            col: start_col + d,
            cost: (1..=(start_col + d)).map(|c| grid.get(start_row, c)).sum(),
            direction: Direction::Right,
            h: manhattan(start_row, start_col + d, target_row, target_col),
        });
        // steps to the bottom
        steps.push(Step {
            row: start_row + d,
            col: start_col,
            cost: (1..=(start_row + d)).map(|r| grid.get(r, start_col)).sum(),
            direction: Direction::Down,
            h: manhattan(start_row + d, start_col, target_row, target_col),
        });
    }

    print_steps(steps.iter(), rows, cols);

    while let Some(Step {
        row,
        col,
        cost,
        direction,
        ..
    }) = steps.pop()
    {
        if !visited.set_min(row, col, direction, cost) {
            continue;
        }

        if row == target_row && col == target_col {
            break;
        }

        if !matches!(direction, Direction::Right | Direction::Left) {
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

        if !matches!(direction, Direction::Left | Direction::Right) {
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
        if !matches!(direction, Direction::Down | Direction::Up) {
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

        if !matches!(direction, Direction::Up | Direction::Down) {
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
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "967");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "94");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
