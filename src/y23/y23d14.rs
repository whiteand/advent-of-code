pub fn solve_task1(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);
    let mut coords = grid.round_rocks_coords();
    grid.tilt::<North>(&mut coords);
    grid.get_value(&coords)
}

pub fn solve_task2<const N: usize>(file_content: &str) -> usize {
    let mut grid = parse_grid(file_content);
    let mut coords = grid.round_rocks_coords();
    let mut visited: Vec<(Vec<(usize, usize)>, usize)> = Vec::new();
    let mut results: Vec<usize> = Vec::new();
    let mut first_duplication = 0;
    let mut loop_start = 0;
    'tilting: for i in 0..N {
        grid.tilt::<North>(&mut coords);
        grid.tilt::<West>(&mut coords);
        grid.tilt::<South>(&mut coords);
        grid.tilt::<East>(&mut coords);
        for (c, it) in visited.iter() {
            if c == &coords {
                first_duplication = i;
                loop_start = *it;
                break 'tilting;
            }
        }
        visited.push((coords.clone(), i));
        results.push(grid.get_value(&coords));
    }
    let loop_len = first_duplication - loop_start;
    let ind = (N - 1 - loop_start) % loop_len + loop_start;
    results[ind]
}

fn parse_grid(file_content: &str) -> Grid {
    let map = file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(move |char| match char {
                    '#' => Some(Rock::Square),
                    'O' => Some(Rock::Round),
                    '.' => None,
                    _ => panic!("Invalid char"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = map.len();
    let cols = map[0].len();
    Grid { map, rows, cols }
}

struct Grid {
    map: Vec<Vec<Option<Rock>>>,
    rows: usize,
    cols: usize,
}

impl Grid {
    fn round_rocks_coords(&self) -> Vec<(usize, usize)> {
        self.map
            .iter()
            .enumerate()
            .flat_map(|(row, line)| {
                line.iter().enumerate().filter_map(move |(col, rock)| {
                    rock.as_ref()
                        .and_then(|r| matches!(r, Rock::Round).then_some((row, col)))
                })
            })
            .collect::<Vec<_>>()
    }
    fn tilt<D: TiltDirection>(&mut self, coords: &mut [(usize, usize)]) {
        coords.sort_unstable_by(D::cmp);
        for coord in coords {
            let next_position = D::next_pos(self, coord);
            if let Some((r, c)) = next_position {
                let x = self.map[coord.0][coord.1].take();
                self.map[r][c] = x;
                *coord = (r, c);
            }
        }
    }
    fn get_value(&self, coords: &[(usize, usize)]) -> usize {
        self.rows * coords.len() - coords.iter().map(|(r, _)| r).sum::<usize>()
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Rock {
    Round,
    Square,
}

trait TiltDirection {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering;
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)>;
}

struct North;

impl TiltDirection for North {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.cmp(c2)
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        (0..row)
            .rev()
            .map(|r| (r, col))
            .take_while(|c| grid.map[c.0][c.1].is_none())
            .last()
    }
}
struct West;

impl TiltDirection for West {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.1.cmp(&c2.1)
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        (0..col)
            .rev()
            .map(|c| (row, c))
            .take_while(|c| grid.map[c.0][c.1].is_none())
            .last()
    }
}
struct South;

impl TiltDirection for South {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.0.cmp(&c2.0).reverse()
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        ((row + 1)..grid.rows)
            .map(|r| (r, col))
            .take_while(|c| grid.map[c.0][c.1].is_none())
            .last()
    }
}
struct East;

impl TiltDirection for East {
    fn cmp(c1: &(usize, usize), c2: &(usize, usize)) -> std::cmp::Ordering {
        c1.1.cmp(&c2.1).reverse()
    }
    fn next_pos(grid: &Grid, coord: &(usize, usize)) -> Option<(usize, usize)> {
        let (row, col) = *coord;
        ((col + 1)..grid.cols)
            .map(|c| (row, c))
            .take_while(|c| grid.map[c.0][c.1].is_none())
            .last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d14/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d14.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "136");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "106997");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2::<1_000_000_000>(INPUT)), "64");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2::<1_000_000_000>(ACTUAL)), "99641");
    }
}
