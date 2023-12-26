use std::collections::VecDeque;

struct Grid {
    cells: Vec<Vec<bool>>,
    start: (usize, usize),
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (start_row, start_col) = self.start;
        for (i, row) in self.cells.iter().enumerate() {
            for (j, cell) in row.iter().enumerate() {
                if i == start_row && j == start_col {
                    write!(f, "S")?;
                } else if *cell {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn parse_grid(input: &str) -> Grid {
    let mut grid = Vec::new();
    let mut start = (0, 0);
    for (row, line) in input.lines().enumerate() {
        let mut cells = Vec::with_capacity(line.len());
        for (col, cell) in line.chars().enumerate() {
            match cell {
                '.' => cells.push(false),
                '#' => cells.push(true),
                'S' => {
                    cells.push(false);
                    start = (row, col);
                }
                _ => panic!("invalid input"),
            }
        }
        grid.push(cells);
    }
    Grid { cells: grid, start }
}

fn get_min_distances(grid: &Grid) -> Vec<Vec<u16>> {
    let mut min_distances = vec![vec![u16::MAX; grid.cells[0].len()]; grid.cells.len()];

    let mut queue = VecDeque::new();
    queue.push_back((grid.start, 0));

    while let Some((coord, dist)) = queue.pop_front() {
        if min_distances[coord.0][coord.1] <= dist {
            continue;
        }
        min_distances[coord.0][coord.1] = dist;
        if coord.0 > 0 && !grid.cells[coord.0 - 1][coord.1] {
            queue.push_back(((coord.0 - 1, coord.1), dist + 1));
        }
        if coord.0 < grid.cells.len() - 1 && !grid.cells[coord.0 + 1][coord.1] {
            queue.push_back(((coord.0 + 1, coord.1), dist + 1));
        }
        if coord.1 > 0 && !grid.cells[coord.0][coord.1 - 1] {
            queue.push_back(((coord.0, coord.1 - 1), dist + 1));
        }
        if coord.1 < grid.cells[0].len() - 1 && !grid.cells[coord.0][coord.1 + 1] {
            queue.push_back(((coord.0, coord.1 + 1), dist + 1));
        }
    }

    min_distances
}

fn print_distances(ds: &[Vec<u16>]) {
    for row in ds {
        for d in row {
            if *d == u16::MAX {
                print!("  .");
            } else {
                print!("{:3}", d);
            }
        }
        println!();
    }
}

fn solve(file_content: &str, steps: usize) -> usize {
    let grid = parse_grid(file_content);
    let min_distance = get_min_distances(&grid);
    let max_distance = min_distance
        .iter()
        .flatten()
        .copied()
        .filter(|x| *x != u16::MAX)
        .max()
        .unwrap();
    let mut counts_per_distance: Vec<u16> = Vec::new();
    println!("{:?}", grid);
    print_distances(&min_distance);
    for i in 0..=max_distance {
        let count = min_distance.iter().flatten().filter(|x| **x == i).count();
        counts_per_distance.push(count as u16);
        println!("{}: {}", i, count);
    }
    (0..=max_distance)
        .filter(|x| (*x as usize) <= steps)
        .filter(|x| (*x as usize) % 2 == steps % 2)
        .map(|x| counts_per_distance[x as usize] as usize)
        .sum()
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, 64)
}
pub fn solve_part_2(_file_content: &str) -> usize {
    todo!("part 2 is not implemented yet")
}

#[cfg(test)]
mod tests {
    use super::{solve, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve(EXAMPLE, 6)), "16");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve(ACTUAL, 64)), "3740");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
