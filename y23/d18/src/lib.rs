use itertools::Itertools;

#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Ord)]
struct IVec2(isize, isize);
impl std::fmt::Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
impl IVec2 {
    fn add_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 += other.0;
        self.1 += other.1;
        self
    }
    fn mul_mut(&mut self, distance: usize) -> &mut Self {
        self.0 *= distance as isize;
        self.1 *= distance as isize;
        self
    }
}

fn solve(instructions: impl Iterator<Item = Instruction>) -> isize {
    let points = instructions
        .scan(IVec2(0isize, 0isize), |pos, instruction| {
            pos.add_mut(IVec2::from(instruction.direction).mul_mut(instruction.distance));

            Some(pos.clone())
        })
        .collect::<Vec<_>>();

    let area: isize = std::iter::once(&IVec2(0, 0))
        .chain(points.iter())
        .tuple_windows()
        .map(|(a, b)| a.0 * b.1 - a.1 * b.0)
        .sum::<isize>()
        .abs();
    let perimeter: isize = std::iter::once(&IVec2(0, 0))
        .chain(points.iter())
        .tuple_windows()
        .map(|(a, b)| (b.0 - a.0).abs() + (b.1 - a.1).abs())
        .sum();
    (area + perimeter) / 2 + 1
}

pub fn solve_part_1(file_content: &str) -> isize {
    solve(parse_instructions(file_content))
}
pub fn solve_part_2(file_content: &str) -> isize {
    solve(parse_instructions(file_content).map(|instruction| {
        let direction = match instruction.color.0 & 0b1111 {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => Direction::Up,
        };
        let distance = instruction.color.0 >> 4;
        Instruction {
            color: Color(instruction.distance),
            distance,
            direction,
        }
    }))
}

#[derive(Debug, Eq, Hash, PartialEq, Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl From<Direction> for IVec2 {
    fn from(direction: Direction) -> Self {
        use Direction::*;
        match direction {
            Up => IVec2(-1, 0),
            Right => IVec2(0, 1),
            Down => IVec2(1, 0),
            Left => IVec2(0, -1),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        match self {
            Up => write!(f, "U"),
            Right => write!(f, "R"),
            Down => write!(f, "D"),
            Left => write!(f, "L"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Color(usize);

impl std::fmt::Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color(color) = self;
        write!(f, "#{:06x}", color)
    }
}
impl std::fmt::Debug for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Color(color) = self;
        write!(f, "#{:06x}", color)
    }
}

struct Instruction {
    direction: Direction,
    distance: usize,
    color: Color,
}
impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.direction, self.distance, self.color)
    }
}

fn parse_instructions(file_content: &str) -> impl Iterator<Item = Instruction> + '_ {
    file_content.lines().map(|line| {
        let mut parts = line.split_whitespace();
        let direction = match parts.next().unwrap() {
            "U" => Direction::Up,
            "R" => Direction::Right,
            "D" => Direction::Down,
            "L" => Direction::Left,
            _ => panic!("invalid direction"),
        };
        let distance = parts.next().unwrap().parse().unwrap();
        let color_str = parts.next().unwrap();
        let color = usize::from_str_radix(&color_str[2..color_str.len() - 1], 16)
            .map(Color)
            .unwrap();
        Instruction {
            color,
            distance,
            direction,
        }
    })
}

#[cfg(test)]
mod tests {

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "62");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(solve_part_1(ACTUAL), 56923);
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "952408144115");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "66296566363189");
    }
}
