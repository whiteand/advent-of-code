use std::{convert::Infallible, fmt::Write, str::FromStr};

use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> usize {
    let mut grid = Grid::from_str(file_content).unwrap();
    for _ in 0..100 {
        grid.next()
    }
    grid.coords().filter(|(r, c)| grid.get(*r, *c)).count()
}

pub fn solve_part_2(file_content: &str) -> usize {
    let mut grid = Grid::from_str(file_content).unwrap();
    *grid.lights.first_mut().unwrap().first_mut().unwrap() = true;
    *grid.lights.first_mut().unwrap().last_mut().unwrap() = true;
    *grid.lights.last_mut().unwrap().first_mut().unwrap() = true;
    *grid.lights.last_mut().unwrap().last_mut().unwrap() = true;
    for _ in 0..100 {
        grid.next();
        *grid.lights.first_mut().unwrap().first_mut().unwrap() = true;
        *grid.lights.first_mut().unwrap().last_mut().unwrap() = true;
        *grid.lights.last_mut().unwrap().first_mut().unwrap() = true;
        *grid.lights.last_mut().unwrap().last_mut().unwrap() = true;
    }
    grid.coords().filter(|(r, c)| grid.get(*r, *c)).count()
}

#[derive(Clone)]
struct Grid {
    lights: Vec<Vec<bool>>,
    next_lights: Vec<Vec<bool>>,
}

impl Grid {
    fn get(&self, row: i64, col: i64) -> bool {
        if row < 0 || row >= self.lights.len() as i64 {
            return false;
        }
        if col < 0 || col >= self.lights[0].len() as i64 {
            return false;
        }
        self.lights[row as usize][col as usize]
    }
    fn coords(&self) -> impl Iterator<Item = (i64, i64)> {
        let rows = self.lights.len() as i64;
        let cols = self.lights[0].len() as i64;
        (0..rows).cartesian_product(0..cols)
    }
    fn neighbours(row: i64, col: i64) -> impl Iterator<Item = (i64, i64)> {
        ((row - 1)..=row + 1)
            .cartesian_product((col - 1)..=col + 1)
            .filter(move |(r, c)| *r != row || *c != col)
    }
    fn next(&mut self) {
        for (r, c) in self.coords() {
            let is_on = self.get(r, c);
            let on_neighbours = Grid::neighbours(r, c)
                .map(|(r, c)| self.get(r, c))
                .filter(|x| *x)
                .count();
            self.next_lights[r as usize][c as usize] = if is_on {
                (2..4).contains(&on_neighbours)
            } else {
                on_neighbours == 3
            }
        }
        std::mem::swap(&mut self.next_lights, &mut self.lights);
    }
}

impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char('⎡')?;
        f.write_str(&"-".repeat(self.lights[0].len()))?;
        f.write_char('⎤')?;
        f.write_char('\n')?;
        for row in self.lights.iter() {
            f.write_char('|')?;
            for light in row.iter() {
                f.write_char(if *light { '#' } else { '.' })?;
            }
            f.write_char('|')?;
            f.write_char('\n')?;
        }
        f.write_char('⎣')?;
        f.write_str(&"-".repeat(self.lights[0].len()))?;
        f.write_char('⎦')?;
        Ok(())
    }
}

impl FromStr for Grid {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lights = s
            .lines()
            .map(|x| {
                x.as_bytes()
                    .iter()
                    .copied()
                    .map(|x| if x == b'#' { true } else { false })
                    .collect_vec()
            })
            .collect_vec();

        Ok(Grid {
            next_lights: lights.clone(),
            lights,
        })
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::Grid;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let mut grid = Grid::from_str(EXAMPLE).unwrap();
        for _ in 0..4 {
            grid.next()
        }
        assert_eq!(grid.coords().filter(|(r, c)| grid.get(*r, *c)).count(), 4)
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "814");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "924");
    }
}
