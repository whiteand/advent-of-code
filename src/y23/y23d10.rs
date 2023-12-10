use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Cell {
    Horizontal,
    Vertical,
    TopRight,
    TopLeft,
    BottomRight,
    BottomLeft,
    None,
    Start,
}

impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Cell::*;
        match self {
            Horizontal => write!(f, "_"),
            Vertical => write!(f, "|"),
            TopRight => write!(f, "L"),
            TopLeft => write!(f, "J"),
            BottomRight => write!(f, "F"),
            BottomLeft => write!(f, "7"),
            None => write!(f, "."),
            Start => write!(f, "S"),
        }
    }
}

impl TryFrom<char> for Cell {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        use Cell::*;
        match value {
            '-' => Ok(Horizontal),
            '|' => Ok(Vertical),
            'L' => Ok(TopRight),
            'J' => Ok(TopLeft),
            'F' => Ok(BottomRight),
            '7' => Ok(BottomLeft),
            '.' => Ok(None),
            'S' => Ok(Start),
            _ => Err(()),
        }
    }
}

struct Grid {
    cells: Vec<Vec<Cell>>,
}
impl Grid {
    fn get(&self, row: usize, col: usize) -> Cell {
        self.cells[row][col]
    }
    fn start(&self) -> (usize, usize) {
        self.cells
            .iter()
            .enumerate()
            .find_map(|(row, cells)| {
                cells
                    .iter()
                    .enumerate()
                    .find_map(|(col, cell)| matches!(cell, Cell::Start).then_some((row, col)))
            })
            .unwrap()
    }
}
impl Deref for Grid {
    type Target = Vec<Vec<Cell>>;

    fn deref(&self) -> &Self::Target {
        &self.cells
    }
}
impl DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.cells
    }
}

impl FromIterator<Vec<Cell>> for Grid {
    fn from_iter<T: IntoIterator<Item = Vec<Cell>>>(iter: T) -> Self {
        let mut cells = iter.into_iter().collect::<Vec<_>>();
        for line in &mut cells {
            line.insert(0, Cell::None);
            line.push(Cell::None);
        }
        cells.insert(0, vec![Cell::None; cells[0].len()]);
        cells.push(vec![Cell::None; cells[0].len()]);
        Grid { cells }
    }
}
impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.cells {
            for cell in row {
                write!(f, "{:?}", cell)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

struct Task {
    from: (usize, usize),
    to: (usize, usize),
    steps: usize,
}

#[derive(Debug, Clone)]
struct Visited {
    row: usize,
    col: usize,
    steps: usize,
}
fn print_visited(visited: &[Vec<Option<Visited>>]) {
    for row in visited {
        for cell in row {
            if let Some(Visited { steps, .. }) = cell {
                print!("{:2} ", steps);
            } else {
                print!(".. ");
            }
        }
        println!();
    }
    println!();
}

pub fn solve_task1(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let start = grid.start();

    let (_, end) = get_visited_from(&grid, start);
    end.steps
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Left,
    Right,
    On,
}
#[derive(Debug)]
struct ColorTask {
    row: usize,
    col: usize,
    color: Color,
}

fn print_colors(colors: &[Vec<Option<Color>>]) {
    for row in colors {
        for cell in row {
            if let Some(color) = cell {
                match color {
                    Color::Left => print!("L"),
                    Color::Right => print!("R"),
                    Color::On => print!("#"),
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
    println!();
}

pub fn solve_task2(file_content: &str) -> usize {
    let grid = parse_grid(file_content);
    let start = grid.start();
    let (visited_from, end) = get_visited_from(&grid, start);
    print_visited(&visited_from);
    let mut colors: Vec<Vec<Option<Color>>> = vec![vec![None; grid[0].len()]; grid.len()];
    let mut queue: VecDeque<ColorTask> = VecDeque::new();
    colors[end.row][end.col] = Some(Color::On);
    let mut ends = vec![];
    match grid.get(end.row, end.col) {
        Cell::Horizontal => {
            ends.push(Visited {
                row: end.row,
                col: end.col - 1,
                steps: visited_from[end.row][end.col - 1].as_ref().unwrap().steps,
            });
            ends.push(Visited {
                row: end.row,
                col: end.col + 1,
                steps: visited_from[end.row][end.col + 1].as_ref().unwrap().steps,
            });
        }
        Cell::Vertical => {
            ends.push(Visited {
                row: end.row - 1,
                col: end.col,
                steps: visited_from[end.row - 1][end.col].as_ref().unwrap().steps,
            });
            ends.push(Visited {
                row: end.row + 1,
                col: end.col,
                steps: visited_from[end.row + 1][end.col].as_ref().unwrap().steps,
            });
        }
        Cell::TopRight => {
            ends.push(Visited {
                row: end.row - 1,
                col: end.col,
                steps: visited_from[end.row - 1][end.col].as_ref().unwrap().steps,
            });
            ends.push(Visited {
                row: end.row,
                col: end.col + 1,
                steps: visited_from[end.row][end.col + 1].as_ref().unwrap().steps,
            });
        }
        Cell::TopLeft => {
            ends.push(Visited {
                row: end.row - 1,
                col: end.col,
                steps: visited_from[end.row - 1][end.col].as_ref().unwrap().steps,
            });
            ends.push(Visited {
                row: end.row,
                col: end.col - 1,
                steps: visited_from[end.row][end.col - 1].as_ref().unwrap().steps,
            });
        }
        Cell::BottomRight => {
            ends.push(Visited {
                row: end.row + 1,
                col: end.col,
                steps: visited_from[end.row + 1][end.col].as_ref().unwrap().steps,
            });
            ends.push(Visited {
                row: end.row,
                col: end.col + 1,
                steps: visited_from[end.row][end.col + 1].as_ref().unwrap().steps,
            });
        }
        Cell::BottomLeft => {
            ends.push(Visited {
                row: end.row + 1,
                col: end.col,
                steps: visited_from[end.row + 1][end.col].as_ref().unwrap().steps,
            });
            ends.push(Visited {
                row: end.row,
                col: end.col - 1,
                steps: visited_from[end.row][end.col - 1].as_ref().unwrap().steps,
            });
        }
        Cell::None => unreachable!("The end of the loop is not possible to be the none cell"),
        Cell::Start => unreachable!("The end of the loop is not possible to be the start"),
    }
    while let Some(Visited { row, col, steps }) = ends.pop() {
        colors[row][col] = Some(Color::On);
        if steps == 0 {
            continue;
        }
        print_colors(&colors);
        let previuos_step = visited_from[row][col].as_ref().unwrap();
        if colors[previuos_step.row][previuos_step.col].is_none() {
            ends.push(Visited {
                row: previuos_step.row,
                col: previuos_step.col,
                steps: steps - 1,
            })
        }
    }
    print_colors(&colors);
    0
}

fn get_visited_from(grid: &Grid, start: (usize, usize)) -> (Vec<Vec<Option<Visited>>>, Visited) {
    let mut visited_from: Vec<Vec<Option<Visited>>> = vec![vec![None; grid[0].len()]; grid.len()];

    let mut queue: VecDeque<Task> = VecDeque::new();
    queue.push_back(Task {
        from: start,
        to: start,
        steps: 0,
    });
    let mut max_step_coords: Vec<Visited> = Vec::new();
    let mut max_steps = 0;
    while let Some(Task {
        from: (from_row, from_col),
        to: (to_row, to_col),
        steps,
    }) = queue.pop_front()
    {
        if visited_from[to_row][to_col].is_some() {
            let res = visited_from[to_row][to_col].clone().unwrap();
            if res.col != from_col
                && res.row != from_row
                && (res.steps == steps - 1 || res.steps == steps)
            {
                return (visited_from, res);
            }
            continue;
        }
        visited_from[to_row][to_col] = Some(Visited {
            steps,
            row: from_row,
            col: from_col,
        });
        if steps > max_steps {
            max_step_coords.clear();
            max_step_coords.push(Visited {
                row: to_row,
                col: to_col,
                steps,
            });
            max_steps = steps;
        }
        if (to_row + 1 != from_row || to_col != from_col)
            && matches!(
                grid.get(to_row + 1, to_col),
                Cell::Vertical | Cell::TopRight | Cell::TopLeft
            )
            && matches!(
                grid.get(to_row, to_col),
                Cell::Vertical | Cell::BottomLeft | Cell::BottomRight | Cell::Start
            )
        {
            queue.push_back(Task {
                from: (to_row, to_col),
                to: (to_row + 1, to_col),
                steps: steps + 1,
            });
        }
        if (to_row != from_row || to_col + 1 != from_col)
            && matches!(
                grid.get(to_row, to_col + 1),
                Cell::Horizontal | Cell::TopLeft | Cell::BottomLeft
            )
            && matches!(
                grid.get(to_row, to_col),
                Cell::Horizontal | Cell::BottomRight | Cell::TopRight | Cell::Start
            )
        {
            queue.push_back(Task {
                from: (to_row, to_col),
                to: (to_row, to_col + 1),
                steps: steps + 1,
            });
        }
        if (to_row - 1 != from_row || to_col != from_col)
            && matches!(
                grid.get(to_row - 1, to_col),
                Cell::Vertical | Cell::BottomRight | Cell::BottomLeft
            )
            && matches!(
                grid.get(to_row, to_col),
                Cell::Vertical | Cell::TopRight | Cell::TopLeft | Cell::Start
            )
        {
            queue.push_back(Task {
                from: (to_row, to_col),
                to: (to_row - 1, to_col),
                steps: steps + 1,
            });
        }
        if (to_row != from_row || to_col - 1 != from_col)
            && matches!(
                grid.get(to_row, to_col - 1),
                Cell::Horizontal | Cell::BottomRight | Cell::TopRight
            )
            && (matches!(
                grid.get(to_row, to_col),
                Cell::Horizontal | Cell::TopLeft | Cell::BottomLeft | Cell::Start
            ))
        {
            queue.push_back(Task {
                from: (to_row, to_col),
                to: (to_row, to_col - 1),
                steps: steps + 1,
            });
        }
    }
    unreachable!()
}

fn parse_grid(file_content: &str) -> Grid {
    file_content
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Cell::try_from(c).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Grid>()
}
#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("./y23d10/example.txt");
    const EXAMPLE_2: &str = include_str!("./y23d10/example2.txt");
    const EXAMPLE_3: &str = include_str!("./y23d10/example3.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d10.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(EXAMPLE)), "8");
    }

    #[test]
    fn test_task1_2() {
        assert_eq!(format!("{}", solve_task1(EXAMPLE_2)), "4");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "6820");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(EXAMPLE_3)), "4");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
