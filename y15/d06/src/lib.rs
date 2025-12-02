use nom::Parser;

struct Coords {
    start_row: usize,
    start_col: usize,
    end_row: usize,
    end_col: usize,
}
impl Coords {
    fn iter(&self) -> impl Iterator<Item = (usize, usize)> {
        let start_col = self.start_col;
        let end_col = self.end_col;
        (self.start_row..=self.end_row)
            .flat_map(move |row| (start_col..=end_col).map(move |col| (row, col)))
    }
}

enum Command {
    On(Coords),
    Off(Coords),
    Toggle(Coords),
}

fn parse_coords(input: &str) -> nom::IResult<&str, Coords> {
    let num_parser =
        || nom::character::complete::u16::<&str, nom::error::Error<&str>>.map(|v| v as usize);
    let point_parser = || {
        nom::sequence::separated_pair(
            num_parser(),
            nom::character::complete::char(','),
            num_parser(),
        )
    };
    let mut points_parser = nom::sequence::separated_pair(
        point_parser(),
        nom::bytes::complete::tag(" through "),
        point_parser(),
    )
    .map(|((r, c), (r2, c2))| Coords {
        start_row: r,
        start_col: c,
        end_row: r2,
        end_col: c2,
    });

    points_parser.parse(input)
}

fn parse_command(input: &str) -> nom::IResult<&str, Command> {
    let parse_turn_on =
        nom::sequence::preceded(nom::bytes::complete::tag("turn on "), parse_coords)
            .map(Command::On);
    let parse_turn_off =
        nom::sequence::preceded(nom::bytes::complete::tag("turn off "), parse_coords)
            .map(Command::Off);
    let parse_toggle = nom::sequence::preceded(nom::bytes::complete::tag("toggle "), parse_coords)
        .map(Command::Toggle);

    nom::branch::alt((parse_toggle, parse_turn_off, parse_turn_on)).parse(input)
}

#[derive(Debug, Default, Clone, Copy)]
struct SimpleLight {
    is_on: bool,
}
trait Light: Default + Copy {
    fn turn_on(&mut self);
    fn turn_off(&mut self);
    fn toggle(&mut self);
}

impl Light for SimpleLight {
    fn turn_on(&mut self) {
        self.is_on = true;
    }

    fn turn_off(&mut self) {
        self.is_on = false;
    }

    fn toggle(&mut self) {
        self.is_on = !self.is_on;
    }
}

const GRID_SIZE: usize = 1000;
pub fn solve_part_1(file_content: &str) -> usize {
    solve::<SimpleLight>(file_content)
        .into_iter()
        .flat_map(|row| row.into_iter())
        .filter(|cell| cell.is_on)
        .count()
}

#[derive(Debug, Default, Clone, Copy)]
struct SmartLight {
    level: usize,
}
impl Light for SmartLight {
    fn turn_on(&mut self) {
        self.level += 1;
    }

    fn turn_off(&mut self) {
        self.level = self.level.saturating_sub(1);
    }

    fn toggle(&mut self) {
        self.level += 2;
    }
}

pub fn solve_part_2(file_content: &str) -> usize {
    solve::<SmartLight>(file_content)
        .iter()
        .flat_map(|row| row.iter())
        .map(|cell| cell.level)
        .sum()
}

fn solve<T: Light>(file_content: &str) -> Vec<Vec<T>> {
    let mut grid = vec![vec![T::default(); GRID_SIZE]; GRID_SIZE];
    for command in file_content
        .lines()
        .map(|line| parse_command(line).unwrap().1)
    {
        match command {
            Command::On(coords) => {
                for (r, c) in coords.iter() {
                    grid[r][c].turn_on();
                }
            }
            Command::Off(coords) => {
                for (r, c) in coords.iter() {
                    grid[r][c].turn_off()
                }
            }
            Command::Toggle(coords) => {
                for (r, c) in coords.iter() {
                    grid[r][c].toggle();
                }
            }
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "998996");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "400410");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "1001996");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "15343601");
    }
}
