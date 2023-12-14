use std::{collections::BTreeMap, fmt::Write};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}
impl From<Rock> for char {
    fn from(rock: Rock) -> Self {
        match rock {
            Rock::Round => 'O',
            Rock::Square => '#',
        }
    }
}
impl std::fmt::Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(char::from(*self))
    }
}
struct Grid {
    map: BTreeMap<(usize, usize), Rock>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn new(rows: usize, cols: usize, map: BTreeMap<(usize, usize), Rock>) -> Self {
        Self { rows, cols, map }
    }
}

impl FromIterator<(usize, usize, Rock)> for Grid {
    fn from_iter<T: IntoIterator<Item = (usize, usize, Rock)>>(iter: T) -> Self {
        let mut map = BTreeMap::new();
        let mut rows = 0;
        let mut cols = 0;
        for (row, col, rock) in iter {
            if row >= rows {
                rows = row + 1
            }
            if col >= cols {
                cols = cols + 1;
            }
            map.insert((row, col), rock);
        }
        Self::new(rows, cols, map)
    }
}

impl std::ops::Deref for Grid {
    type Target = BTreeMap<(usize, usize), Rock>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.cols {
                let ch = self.get(&(row, col)).map(|r| char::from(*r)).unwrap_or('.');
                f.write_char(ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Grid({}x{})", self.rows, self.cols)?;
        writeln!(f, "{}", self)
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    dbg!(grid);
    0
}
pub fn solve_task2(_file_content: &str) -> usize {
    0
}
fn parse_grid(file_content: &str) -> Grid {
    file_content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(col, char)| match char {
                    '#' => Some((row, col, Rock::Square)),
                    'O' => Some((row, col, Rock::Round)),
                    '.' => None,
                    _ => panic!("Invalid char"),
                })
        })
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d14/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d14.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "0");
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
