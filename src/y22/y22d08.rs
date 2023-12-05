fn parse_grid(file_content: &str) -> Vec<Vec<u8>> {
    file_content
        .lines()
        .map(|line| line.chars().map(|c| c as u8 - b'0').collect())
        .collect()
}

fn nullify(arr: &[Vec<u8>]) -> Vec<Vec<u8>> {
    arr.iter().map(|line| vec![0; line.len()]).collect()
}

// 1705
pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut top: Vec<Vec<u8>> = nullify(&grid);
    let mut right: Vec<Vec<u8>> = nullify(&grid);
    let mut bottom: Vec<Vec<u8>> = nullify(&grid);
    let mut left: Vec<Vec<u8>> = nullify(&grid);

    for i in (0..(rows - 1)).rev() {
        for j in 0..cols {
            top[i][j] = top[i + 1][j].max(grid[i + 1][j]);
        }
    }

    for i in 1..rows {
        for j in 0..cols {
            bottom[i][j] = bottom[i - 1][j].max(grid[i - 1][j]);
        }
    }

    for j in (0..(cols - 1)).rev() {
        for i in 0..rows {
            right[i][j] = right[i][j + 1].max(grid[i][j + 1]);
        }
    }
    for j in 1..cols {
        for i in 0..rows {
            left[i][j] = left[i][j - 1].max(grid[i][j - 1]);
        }
    }

    let mut res = 0;
    for (row, line) in grid.iter().enumerate() {
        for col in 0..line.len() {
            if row == 0 || col == 0 || row == rows - 1 || col == cols - 1 {
                res += 1;
                continue;
            }
            let v = grid[row][col];
            if top[row][col] == 0
                || right[row][col] == 0
                || bottom[row][col] == 0
                || left[row][col] == 0
            {
                if v > 0 {
                    res += 1;
                }
                continue;
            }

            if top[row][col] < v {
                res += 1;
                continue;
            }
            if right[row][col] < v {
                res += 1;
                continue;
            }
            if bottom[row][col] < v {
                res += 1;
                continue;
            }
            if left[row][col] < v {
                res += 1;
                continue;
            }
        }
    }
    res
}

struct TakeWhileInclusiveIter<T, P>
where
    T: Iterator,
    P: Fn(&T::Item) -> bool,
{
    iter: T,
    finished: bool,
    predicate: P,
}

trait TakeWhileInclusive: Iterator + Sized {
    fn take_while_inclusive<P>(self, predicate: P) -> TakeWhileInclusiveIter<Self, P>
    where
        P: Fn(&Self::Item) -> bool,
    {
        TakeWhileInclusiveIter {
            iter: self,
            finished: false,
            predicate,
        }
    }
}

impl<T> TakeWhileInclusive for T where T: Iterator {}

impl<T, P> Iterator for TakeWhileInclusiveIter<T, P>
where
    T: Iterator,
    P: Fn(&T::Item) -> bool,
{
    type Item = T::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        match self.iter.next() {
            Some(v) => {
                if (self.predicate)(&v) {
                    return Some(v);
                }
                self.finished = true;
                Some(v)
            }
            None => None,
        }
    }
}

fn get_score(grid: &Vec<Vec<u8>>, row: usize, col: usize) -> usize {
    let cols = grid[0].len();
    let top = (0..row)
        .rev()
        .map(|ind| grid[ind][col])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    let right = (col + 1..cols)
        .map(|ind| grid[row][ind])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    let bottom = ((row + 1)..grid.len())
        .map(|ind| grid[ind][col])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    let left = (0..col)
        .rev()
        .map(|ind| grid[row][ind])
        .take_while_inclusive(|v| *v < grid[row][col])
        .count();
    top * right * bottom * left
}
// 371200
pub fn solve_task2(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let mut res = 0;
    for (row, line) in grid.iter().enumerate() {
        for col in 0..line.len() {
            let score = get_score(&grid, row, col);
            res = res.max(score);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;
    const INPUT: &str = "30373
25512
65332
33549
35390";

    #[ignore]
    #[test]
    fn test_actual() {
        let str = fs::read_to_string("benches/y22/y22d8.txt").unwrap_or_default();
        let res = solve_task1(&str);
        assert_eq!(res, 1705);
    }

    #[ignore]
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT.trim())), "21");
    }

    #[ignore]
    #[test]
    fn test_score_1() {
        let grid = parse_grid(INPUT);
        let score = get_score(&grid, 1, 2);
        assert_eq!(score, 4)
    }
    #[ignore]
    #[test]
    fn test_score_2() {
        let grid = parse_grid(INPUT);
        let score = get_score(&grid, 3, 2);
        assert_eq!(score, 8)
    }
    #[ignore]
    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "8");
    }
}
