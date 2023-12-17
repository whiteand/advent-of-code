use pathfinding::prelude::astar;
pub fn solve(file_content: &str, min_steps: usize, max_steps: usize) -> usize {
    let grid = parse_grid(file_content);
    let (rows, cols) = grid.dimensions();

    let res = astar(
        &Step {
            col: 0,
            direction: None,
            row: 0,
        },
        |Step {
             row,
             col,
             direction,
             ..
         }| {
            let mut steps = Vec::new();
            let row = *row;
            let col = *col;
            let direction = *direction;

            if !matches!(direction, Some(Direction::Right | Direction::Left)) {
                let mut collected_cost: usize = 0;
                for c in col + 1..cols.min(col + max_steps + 1) {
                    collected_cost = collected_cost.saturating_add(grid.get(row, c));
                    if c - col < min_steps {
                        continue;
                    }
                    steps.push((
                        Step {
                            row,
                            col: c,
                            direction: Some(Direction::Right),
                        },
                        collected_cost,
                    ))
                }

                let mut collected_cost = 0usize;
                for c in (col.saturating_sub(max_steps)..col).rev() {
                    collected_cost = collected_cost.saturating_add(grid.get(row, c));
                    if col - c < min_steps {
                        continue;
                    }
                    steps.push((
                        Step {
                            row,
                            col: c,
                            direction: Some(Direction::Left),
                        },
                        collected_cost,
                    ))
                }
            }

            if !matches!(direction, Some(Direction::Down | Direction::Up)) {
                let mut collected_cost = 0usize;
                for r in (row + 1)..rows.min(row + max_steps + 1) {
                    collected_cost = collected_cost.saturating_add(grid.get(r, col));
                    if r - row < min_steps {
                        continue;
                    }
                    steps.push((
                        Step {
                            row: r,
                            col,
                            direction: Some(Direction::Down),
                        },
                        collected_cost,
                    ))
                }
                let mut collected_cost = 0usize;
                for r in (row.saturating_sub(max_steps)..row).rev() {
                    collected_cost = collected_cost.saturating_add(grid.get(r, col));
                    if row - r < min_steps {
                        continue;
                    }
                    steps.push((
                        Step {
                            row: r,
                            col,
                            direction: Some(Direction::Up),
                        },
                        collected_cost,
                    ))
                }
            }
            steps
        },
        |s| manhattan(s.row, s.col, rows - 1, cols - 1),
        |s| s.row == rows - 1 && s.col == cols - 1,
    );

    res.map(|(_, cost)| cost)
        .expect("There should be a solution")
}

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

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Step {
    row: usize,
    col: usize,
    direction: Option<Direction>,
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

fn parse_grid(input: &str) -> Grid {
    let loses = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Invalid digit") as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    Grid { loses }
}

#[cfg(test)]
mod tests {
    use super::solve;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve(INPUT, 1, 3)), "102");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve(ACTUAL, 1, 3)), "967");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve(INPUT, 4, 10)), "94");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve(ACTUAL, 4, 10)), "1101");
    }
}
