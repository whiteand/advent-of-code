use std::ops::RangeInclusive;

use itertools::Itertools;

fn get_gcd(mut a: u64, mut b: u64) -> u64 {
    while a > 0 && b > 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    a.max(b)
}
fn get_gcd_i(mut a: i64, mut b: i64) -> u64 {
    if a == 0 {
        return b.abs() as u64;
    }
    if b == 0 {
        return a.abs() as u64;
    }
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }

    get_gcd(a.try_into().unwrap(), b.try_into().unwrap())
}

#[derive(Copy, Clone, PartialEq, Eq)]
struct Rat {
    top: i64,
    bottom: u64,
}

impl Rat {
    fn new(top: i64, bottom: u64) -> Self {
        let gcd = get_gcd_i(top, bottom as i64);
        Self {
            top: top / gcd as i64,
            bottom: bottom / gcd,
        }
    }
    fn times(&self, k: i64) -> Self {
        Self::new(self.top * k, self.bottom)
    }
    fn signum(&self) -> i64 {
        if self.top < 0 {
            -1
        } else if self.top > 0 {
            1
        } else {
            0
        }
    }
    fn abs(&self) -> Self {
        if self.top < 0 {
            Self {
                top: -self.top,
                bottom: self.bottom,
            }
        } else {
            self.clone()
        }
    }
}

impl std::cmp::PartialOrd for Rat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl std::cmp::Ord for Rat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = self.top * other.bottom as i64;
        let b = other.top * self.bottom as i64;
        a.cmp(&b)
    }
}

impl std::fmt::Debug for Rat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.bottom == 1 {
            return write!(f, "{}", self.top);
        }
        write!(f, "{}/{}", self.top, self.bottom)
    }
}

impl std::ops::Add for Rat {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let bottom = self.bottom * rhs.bottom;
        let top = self.top * rhs.bottom as i64 + rhs.top * self.bottom as i64;

        Self::new(top, bottom)
    }
}
impl std::ops::Mul for Rat {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let bottom = self.bottom * rhs.bottom;
        let top = self.top * rhs.top;
        Self::new(top, bottom)
    }
}
impl std::ops::Div for Rat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        let new_sign = self.signum() * rhs.signum();

        let top = self.top.abs() * (rhs.bottom as i64);
        let bottom = self.bottom * (rhs.top.abs() as u64);

        Self::new(top * new_sign, bottom)
    }
}

impl From<i64> for Rat {
    fn from(i: i64) -> Self {
        Self { top: i, bottom: 1 }
    }
}

#[derive(Clone)]
struct Vec3 {
    x: Rat,
    y: Rat,
    z: Rat,
}

impl Vec3 {
    fn new<X: Into<Rat>, Y: Into<Rat>, Z: Into<Rat>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Object {
    position: Vec3,
    velocity: Vec3,
}

fn solve_xy(
    objects: Vec<Object>,
    x_range: RangeInclusive<isize>,
    y_range: RangeInclusive<isize>,
) -> usize {
    dbg!(objects);
    0
}

fn parse(input: &str) -> Vec<Object> {
    input.lines().map(parse_object).collect_vec()
}
fn parse_object(input: &str) -> Object {
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.trim_end_matches(','))
        .flat_map(|x| x.parse::<i64>().ok());
    let px = it.next().expect("Expected px");
    let py = it.next().expect("Expected py");
    let pz = it.next().expect("Expected pz");
    let vx = it.next().expect("Expected vx");
    let vy = it.next().expect("Expected vy");
    let vz = it.next().expect("Expected vz");
    Object {
        position: Vec3::new(px, py, pz),
        velocity: Vec3::new(vx, vy, vz),
    }
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve_xy(
        parse(file_content),
        200000000000000..=400000000000000,
        200000000000000..=400000000000000,
    )
}

pub fn solve_part_2(_file_content: &str) -> usize {
    todo!("part 2 is not implemented yet")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{parse, solve_part_2, solve_xy};

    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case(EXAMPLE, 7, 21)]
    // #[case(ACTUAL, 200000000000000, 400000000000000)]
    fn test_part1(#[case] input: &str, #[case] min_value: isize, #[case] max_value: isize) {
        let objects = parse(input);
        assert_eq!(
            solve_xy(objects, min_value..=max_value, min_value..=max_value),
            2
        );
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
