use std::num;

#[derive(Debug, Eq, Clone, PartialEq, PartialOrd, Ord)]
struct IVec2(isize, isize);
impl std::fmt::Display for IVec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}
impl IVec2 {
    fn set(&mut self, other: &IVec2) -> &mut Self {
        self.0 = other.0;
        self.1 = other.1;
        self
    }

    fn add_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 += other.0;
        self.1 += other.1;
        self
    }
    fn sub_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 -= other.0;
        self.1 -= other.1;
        self
    }
    fn right(&self) -> Self {
        if self.0 == 0 {
            if self.1 == 0 {
                IVec2(0, 1)
            } else if self.1 > 0 {
                IVec2(-1, 0)
            } else {
                IVec2(1, 0)
            }
        } else if self.1 == 0 {
            if self.0 > 0 {
                IVec2(0, 1)
            } else {
                IVec2(0, -1)
            }
        } else {
            panic!("invalid direction")
        }
    }
    fn negate(&self) -> Self {
        IVec2(-self.0, -self.1)
    }
    fn mul_mut(&mut self, distance: usize) -> &mut Self {
        self.0 *= distance as isize;
        self.1 *= distance as isize;
        self
    }
    fn max_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 = self.0.max(other.0);
        self.1 = self.1.max(other.1);
        self
    }
    fn min_mut(&mut self, other: &IVec2) -> &mut Self {
        self.0 = self.0.min(other.0);
        self.1 = self.1.min(other.1);
        self
    }
}

struct Edge {
    start: IVec2,
    end: IVec2,
    color: Color,
}

impl Edge {
    fn contains(&self, pos: &IVec2) -> bool {
        if pos.0 == self.start.0 && pos.0 == self.end.0 {
            pos.1 >= self.start.1.min(self.end.1) && pos.1 <= self.start.1.max(self.end.1)
        } else if pos.1 == self.start.1 && pos.1 == self.end.1 {
            pos.0 >= self.start.0.min(self.end.0) && pos.0 <= self.start.0.max(self.end.0)
        } else {
            false
        }
    }
    fn dir(&self) -> IVec2 {
        if self.is_horizontal() {
            if self.start.1 > self.end.1 {
                IVec2(0, -1)
            } else {
                IVec2(0, 1)
            }
        } else {
            if self.start.0 > self.end.0 {
                IVec2(-1, 0)
            } else {
                IVec2(1, 0)
            }
        }
    }
    fn len(&self) -> usize {
        if self.is_horizontal() {
            (self.start.1 - self.end.1).abs() as usize + 1
        } else {
            (self.start.0 - self.end.0).abs() as usize + 1
        }
    }
    fn iter(&self) -> impl Iterator<Item = IVec2> {
        let mut p = self.start.clone();
        let e = self.end.clone();
        let e2 = e.clone();
        let dp = self.dir();
        std::iter::from_fn(move || {
            if &p == &e {
                None
            } else {
                p.add_mut(&dp);
                Some(p.clone())
            }
        })
        .chain(std::iter::once(e2))
    }
    fn is_horizontal(&self) -> bool {
        self.start.0 == self.end.0
    }
    fn is_end(&self, pos: &IVec2) -> bool {
        &self.start == pos || &self.end == pos
    }
}

impl std::fmt::Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} -> {}: {}", self.start, self.end, self.color)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Side {
    Left,
    Right,
    None,
}

struct Grid<T> {
    min: IVec2,
    max: IVec2,
    map: Vec<Vec<Option<T>>>,
}

impl<T: Clone> Grid<T> {
    fn new(min: IVec2, max: IVec2) -> Self {
        let mut map: Vec<Vec<Option<T>>> = Vec::new();

        let mut d = max.clone();
        d.sub_mut(&min);

        map.resize((d.0 + 3) as usize, Vec::new());
        for row in map.iter_mut() {
            row.resize((d.1 + 3) as usize, None);
        }

        Self { min, max, map }
    }
}
impl<T> Grid<T> {
    fn get(&self, pos: &IVec2) -> Option<&T> {
        if pos.0 < self.min.0 - 1 || pos.0 > self.max.0 + 1 {
            return None;
        }
        if pos.1 < self.min.1 - 1 || pos.1 > self.max.1 + 1 {
            return None;
        }
        let (row, col) = self.to_indices(pos);
        self.map[row][col].as_ref()
    }
    fn to_indices(&self, pos: &IVec2) -> (usize, usize) {
        let row = (pos.0 - self.min.0 + 1) as usize;
        let col = (pos.1 - self.min.1 + 1) as usize;
        (row, col)
    }
    fn set(&mut self, pos: &IVec2, value: T) -> bool {
        if pos.0 < self.min.0 - 1 || pos.0 > self.max.0 + 1 {
            return false;
        }
        if pos.1 < self.min.1 - 1 || pos.1 > self.max.1 + 1 {
            return false;
        }
        let (row, col) = self.to_indices(pos);
        self.map[row][col] = Some(value);
        true
    }

    fn iter(&self) -> impl Iterator<Item = (IVec2, Option<&T>)> + '_ {
        self.map.iter().enumerate().flat_map(move |(row, r)| {
            r.iter().enumerate().map(move |(col, c)| {
                (
                    IVec2(row as isize + self.min.0, col as isize + self.min.1),
                    c.as_ref(),
                )
            })
        })
    }
}

fn get_doubled_area(a: &IVec2, b: &IVec2) -> isize {
    a.0 * b.1 - a.1 * b.0
}

fn get_gcd(mut a: isize, mut b: isize) -> isize {
    if a == 0 {
        return b;
    }
    if b == 0 {
        return a;
    }
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }
    while a > 0 && b > 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    a.max(b)
}

fn get_line_internal(a: &IVec2, b: &IVec2) -> isize {
    let dx = (b.0 - a.0).abs();
    let dy = (b.1 - a.1).abs();
    let g = get_gcd(dx, dy);
    2 + g - 1
}

fn get_internal_dots(a: &IVec2, b: &IVec2, c: &IVec2) -> isize {
    let mut ab = b.clone();
    ab.sub_mut(&a);
    let mut ac = c.clone();
    ac.sub_mut(&a);

    let s = get_doubled_area(&ab, &ac);
    let double_internal =
        s.abs() - get_line_internal(&a, &b) - get_line_internal(&b, &c) - get_line_internal(&a, &c)
            + 3
            + 2;

    // S = internal + external / 2 - 1
    // 2 * S =  + external - 2
    // internal * 2 = 2S - external + 2

    debug_assert!(double_internal % 2 == 0);

    double_internal / 2 * s.signum()
}

fn solve(instructions: impl Iterator<Item = Instruction>) -> usize {
    let edges = instructions
        .scan(IVec2(0isize, 0isize), |pos, instruction| {
            let prev_pos = pos.clone();
            pos.add_mut(&IVec2::from(instruction.direction).mul_mut(instruction.distance));

            let edge = Edge {
                start: prev_pos,
                end: pos.clone(),
                color: instruction.color,
            };

            Some(edge)
        })
        .collect::<Vec<_>>();

    let start = edges[0].start.clone();
    let mut res = 0isize;
    for edge in edges.iter().skip(1) {
        res += get_internal_dots(&start, &edge.start, &edge.end);
    }
    let mut res: usize = res.abs() as usize;
    for edge in edges.iter() {
        res += edge.len() - 1;
    }
    res as usize
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve(parse_instructions(file_content))
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(
        parse_instructions(file_content).map(|instruction| Instruction {
            color: Color(instruction.distance),
            distance: instruction.color.0,
            direction: instruction.direction,
        }),
    )
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

    use rstest::rstest;

    use super::{get_doubled_area, solve_part_1, solve_part_2, IVec2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case(IVec2(0, 4), IVec2(3, 0), -12)]
    #[case(IVec2(3, 0), IVec2(0, 4), 12)]
    #[case(IVec2(1, 3), IVec2(3, 1), -8)]
    #[case(IVec2(0, 1), IVec2(1, 0), -1)]
    #[case(IVec2(1, 0), IVec2(0, 1), 1)]

    fn test_get_area(#[case] a: IVec2, #[case] b: IVec2, #[case] expected: isize) {
        assert_eq!(get_doubled_area(&a, &b), expected);
    }

    #[rstest]
    #[case(IVec2(0, 0), IVec2(3, 0), IVec2(0, 4), 3)]
    #[case(IVec2(0, 0), IVec2(0, 4), IVec2(3, 0), -3)]
    fn test_get_internal_dots(
        #[case] a: IVec2,
        #[case] b: IVec2,
        #[case] c: IVec2,
        #[case] expected: isize,
    ) {
        assert_eq!(super::get_internal_dots(&a, &b, &c), expected);
    }

    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "62");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        assert_eq!(solve_part_1(ACTUAL), 56923);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "952408144115");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
