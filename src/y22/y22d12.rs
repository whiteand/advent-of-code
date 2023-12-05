use itertools::Itertools;
use std::{collections::VecDeque, ops::RangeInclusive};

pub fn solve_task1(file_content: &str) -> usize {
    let (grid, start, end) = parse_grid(file_content);

    let minimal_distances = calculate_min_distances(&grid, start, |height| 0..=height + 1);

    minimal_distances[end.0][end.1]
}

pub fn solve_task2(file_content: &str) -> usize {
    let (grid, _, start) = parse_grid(file_content);
    let rows = grid.len();
    let cols = grid[0].len();

    let minimal_distances = calculate_min_distances(&grid, start, |current_height| {
        current_height.saturating_sub(1)..=END_VALUE
    });

    (0..rows)
        .cartesian_product(0..cols)
        .filter(|(a, b)| grid[*a][*b] == START_VALUE)
        .map(|(a, b)| minimal_distances[a][b])
        .min()
        .unwrap_or(usize::MAX)
}

pub fn calculate_min_distances(
    grid: &[Vec<usize>],
    start: (usize, usize),
    get_destination_height_range: impl Fn(usize) -> RangeInclusive<usize>,
) -> Vec<Vec<usize>> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut visited = grid
        .iter()
        .map(|v| vec![false; v.len()])
        .collect::<Vec<_>>();
    let mut minimal_distance = grid
        .iter()
        .map(|v| vec![usize::MAX; v.len()])
        .collect::<Vec<_>>();
    let mut tasks: VecDeque<(usize, usize)> = VecDeque::new();

    // invariants:
    //   visited[i] is true if the node was already visited and minimal distance was calculated to all neighbours
    //   minimal_distance[i] - contains minimal distance to the node if the node was already visited
    //   tasks - contains a list of visited nodes which neighbors were potentially not visited.

    minimal_distance[start.0][start.1] = 0;

    tasks.push_back(start);

    while !tasks.is_empty() {
        let Some((row, col)) = tasks.pop_front() else {
            unreachable!();
        };
        if visited[row][col] {
            continue;
        }
        let current_height = grid[row][col];
        let available_range = get_destination_height_range(current_height);
        for (r, c) in get_neighbours(rows, cols, row, col)
            .into_iter()
            .filter(|(r, c)| !visited[*r][*c])
            .filter(|(r, c)| available_range.contains(&grid[*r][*c]))
        {
            let min_distance =
                minimal_distance[r][c].min(minimal_distance[row][col].saturating_add(1));
            minimal_distance[r][c] = min_distance;
            tasks.push_back((r, c));
        }
        visited[row][col] = true;
    }
    minimal_distance
}

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyz";
const END_VALUE: usize = ALPHABET.len() - 1;
const START_VALUE: usize = 0;

type Grid = Vec<Vec<usize>>;
type Coords = (usize, usize);

fn parse_grid(file_content: &str) -> (Grid, Coords, Coords) {
    let mut res = Vec::new();
    let mut start: Option<(usize, usize)> = None;
    let mut end: Option<(usize, usize)> = None;
    for (row, line) in file_content.lines().enumerate() {
        let mut new_line = Vec::with_capacity(line.len());
        for (col, ch) in line.chars().enumerate() {
            let v = if ch == 'S' {
                start = Some((row, col));
                START_VALUE
            } else if ch == 'E' {
                end = Some((row, col));
                END_VALUE
            } else {
                ALPHABET
                    .chars()
                    .enumerate()
                    .find_map(|(i, c)| if c == ch { Some(i) } else { None })
                    .unwrap_or_default()
            };
            new_line.push(v);
        }
        res.push(new_line);
    }
    (res, start.unwrap(), end.unwrap())
}

pub fn get_neighbours(rows: usize, cols: usize, row: usize, col: usize) -> Vec<(usize, usize)> {
    let min_row = row.saturating_sub(1);
    let max_row = (row + 1).min(rows.saturating_sub(1));
    let min_col = col.saturating_sub(1);
    let max_col = (col + 1).min(cols.saturating_sub(1));
    let mut res = Vec::new();
    for r in min_row..=max_row {
        for c in min_col..=max_col {
            if r == row && c == col {
                continue;
            }
            if r != row && c != col {
                continue;
            }
            res.push((r, c))
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";
    const ACTUAL: &str = include_str!("../../benches/y22/y22d12.txt");
    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "31");
    }
    #[test]
    #[ignore]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "484");
    }
    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "29");
    }
    #[test]
    #[ignore]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "478");
    }
}
