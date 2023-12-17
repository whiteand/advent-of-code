use itertools::Itertools;

use crate::y22::y22d16::parse::parse;

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

#[derive(Debug)]
struct CostMap {
    costs: Vec<Vec<Vec<usize>>>,
}

impl CostMap {
    fn new(max_steps: usize, rows: usize, cols: usize) -> Self {
        let n = rows * cols;
        let max_steps = rows * cols;
        let costs = (0..max_steps)
            .map(|_| vec![vec![usize::MAX; cols + 1]; rows + 1])
            .collect::<Vec<_>>();
        Self { costs }
    }

    fn get(&self, available_steps: usize, row: usize, col: usize) -> usize {
        if available_steps >= self.max_steps() {
            return usize::MAX;
        }
        if row >= self.rows() {
            return usize::MAX;
        }
        if col >= self.cols() {
            return usize::MAX;
        }
        self.costs[available_steps][row][col]
    }

    fn set(&mut self, available_steps: usize, row: usize, col: usize, cost: usize) {
        if available_steps >= self.costs.len() {
            return;
        }
        if row >= self.costs[available_steps].len() {
            return;
        }
        if col >= self.costs[available_steps][row].len() {
            return;
        }
        self.costs[available_steps][row][col] = cost;
    }

    fn max_steps(&self) -> usize {
        self.costs.len()
    }

    fn cols(&self) -> usize {
        self.costs[0][0].len()
    }
    fn rows(&self) -> usize {
        self.costs[0].len()
    }
}

impl std::fmt::Display for CostMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ks = f.precision().unwrap_or(self.max_steps());
        dbg!(ks);

        for k in 0..ks {
            let s = self.max_steps() - ks + k;
            writeln!(f, "Available steps: {}", s)?;
            for row in 0..self.rows() {
                for col in 0..self.cols() {
                    let c = self.get(s, row, col);
                    if c == usize::MAX {
                        write!(f, "X")?;
                    } else {
                        write!(f, "{}", c)?;
                    }
                }
                writeln!(f)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let (rows, cols) = grid.dimensions();

    let max_steps = rows * cols;
    dbg!(max_steps, rows, cols);
    let mut cost = CostMap::new(max_steps, rows, cols);
    let start_row = 0;
    let start_col = 0;

    for k in 0..max_steps {
        cost.set(k, start_row, start_col, grid.get(start_row, start_col));
    }
    for step in 1..max_steps {
        for row in 0..rows {
            for col in 0..cols {
                let mut min_cost = cost.get(step, row, col);
                min_cost = min_cost.min(cost.get(step - 1, row, col));

                let current_value = grid.get(row, col);

                for d in 1..=2 {
                    // from left
                    if col >= d {
                        let prev_cost = cost.get(step - 1, row, col - d);
                        let new_cost = prev_cost.saturating_add(
                            (1..=d)
                                .map(|i| grid.get(row, col - d + i))
                                .fold(0usize, |a, b| a.saturating_add(b)),
                        );
                        min_cost = min_cost.min(new_cost);
                    }
                    // from top
                    if row >= d {
                        let prev_cost = cost.get(step - 1, row - d, col);
                        let new_cost = prev_cost.saturating_add(
                            (1..=d)
                                .map(|i| grid.get(row - d + i, col))
                                .fold(0usize, |a, b| a.saturating_add(b)),
                        );
                        min_cost = min_cost.min(new_cost);
                    }
                    // from right
                    if col + d < cols {
                        let prev_cost = cost.get(step - 1, row, col + d);
                        let new_cost = prev_cost.saturating_add(
                            (1..=d)
                                .map(|i| grid.get(row, col + d - i))
                                .fold(0usize, |a, b| a.saturating_add(b)),
                        );
                        min_cost = min_cost.min(new_cost);
                    }
                    // from bottom
                    if row + d < rows {
                        let prev_cost = cost.get(step - 1, row + d, col);
                        let new_cost = prev_cost.saturating_add(
                            (1..=d)
                                .map(|i| grid.get(row + d - i, col))
                                .fold(0usize, |a, b| a.saturating_add(b)),
                        );
                        min_cost = min_cost.min(new_cost);
                    }
                }

                cost.set(step, row, col, min_cost);
            }
        }
    }

    cost.get(max_steps - 1, rows - 1, cols - 1)
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
