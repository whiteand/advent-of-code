use std::ops::RangeInclusive;

use itertools::Itertools;

fn get_gcd(mut a: u128, mut b: u128) -> u128 {
    while a > 0 && b > 0 {
        if a > b {
            a %= b;
        } else {
            b %= a;
        }
    }

    a.max(b)
}
fn get_gcd_i(mut a: i128, mut b: i128) -> u128 {
    if a == 0 {
        return b.abs() as u128;
    }
    if b == 0 {
        return a.abs() as u128;
    }
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }

    get_gcd(a.try_into().unwrap(), b.try_into().unwrap())
}

fn get_lcm(a: u128, b: u128) -> u128 {
    a * (b / get_gcd(a, b))
}

#[derive(Copy, Clone)]
struct Rat {
    top: i128,
    bottom: u128,
}

impl std::iter::Sum for Rat {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut initial = Self::ZERO.clone();
        for x in iter {
            initial += x;
        }
        initial
    }
}

impl std::ops::AddAssign for Rat {
    fn add_assign(&mut self, rhs: Self) {
        let new_bottom = get_lcm(self.bottom, rhs.bottom);
        self.top = self.top * ((new_bottom / self.bottom) as i128)
            + rhs.top * ((new_bottom / rhs.bottom) as i128);
        self.bottom = new_bottom
    }
}

impl Rat {
    const ZERO: Self = Self { top: 0, bottom: 1 };
    const ONE: Self = Self { top: 1, bottom: 1 };
    const MINUS_ONE: Self = Self { top: -1, bottom: 1 };
    fn new(top: i128, bottom: u128) -> Self {
        if top == 0 {
            return Self::ZERO;
        }
        if top == bottom as i128 {
            return Self::ONE;
        }
        if top == -(bottom as i128) {
            return Self::MINUS_ONE;
        }
        let gcd = get_gcd_i(top, bottom as i128);
        Self {
            top: top / gcd as i128,
            bottom: bottom / gcd,
        }
    }
    fn reverse(&self) -> Rat {
        if self.top >= 0 {
            Self {
                top: self.bottom as i128,
                bottom: self.top as u128,
            }
        } else {
            Self {
                top: -(self.bottom as i128),
                bottom: (-self.top) as u128,
            }
        }
    }
    fn is_non_negative(&self) -> bool {
        self.top >= 0
    }
    fn is_zero(&self) -> bool {
        self.top == 0
    }

    fn in_range(&self, range: &RangeInclusive<i128>) -> bool {
        let min = *range.start();
        let max = *range.end();
        let bottom = self.bottom as i128;
        min * bottom <= self.top && self.top <= max * bottom
    }

    fn signum(&self) -> i128 {
        if self.top < 0 {
            -1
        } else if self.top > 0 {
            1
        } else {
            0
        }
    }
}

impl std::ops::Neg for Rat {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            top: -self.top,
            bottom: self.bottom,
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
        let a = self.top * other.bottom as i128;
        let b = other.top * self.bottom as i128;
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
        let new_bottom = get_lcm(self.bottom, rhs.bottom);
        let top = self.top * ((new_bottom / self.bottom) as i128)
            + rhs.top * ((new_bottom / rhs.bottom) as i128);

        Self::new(top, new_bottom)
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

impl std::ops::MulAssign for Rat {
    fn mul_assign(&mut self, rhs: Self) {
        let left_g = get_gcd_i(self.top, rhs.bottom as i128) as i128;
        let right_g = get_gcd_i(self.bottom as i128, rhs.top);
        let new_top = self.top / left_g * (rhs.top / (right_g as i128));
        let new_bottom = self.bottom / right_g * (rhs.bottom / left_g as u128);
        self.top = new_top;
        self.bottom = new_bottom;
    }
}

impl std::ops::Div for Rat {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        if rhs.top == 0 && self.top == 0 {
            return Self::ONE;
        }
        let new_sign = self.signum() * rhs.signum();

        let top = self.top.abs() * (rhs.bottom as i128);
        let bottom = self.bottom * (rhs.top.abs() as u128);

        Self::new(top * new_sign, bottom)
    }
}

impl std::ops::Sub for Rat {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let new_bottom = get_lcm(self.bottom, rhs.bottom);
        let top = self.top * ((new_bottom / self.bottom) as i128)
            - rhs.top * ((new_bottom / rhs.bottom) as i128);
        Self::new(top, new_bottom)
    }
}

impl std::ops::SubAssign for Rat {
    fn sub_assign(&mut self, rhs: Self) {
        let new_bottom = get_lcm(self.bottom, rhs.bottom);
        self.top = self.top * ((new_bottom / self.bottom) as i128)
            - rhs.top * ((new_bottom / rhs.bottom) as i128);
        self.bottom = new_bottom
    }
}

impl From<i128> for Rat {
    fn from(i: i128) -> Self {
        Self { top: i, bottom: 1 }
    }
}

impl TryFrom<Rat> for i64 {
    type Error = ();
    fn try_from(r: Rat) -> Result<Self, Self::Error> {
        if r.bottom != 1 {
            return Err(());
        }
        r.top.try_into().map_err(|_| ())
    }
}

impl std::cmp::PartialEq for Rat {
    fn eq(&self, other: &Self) -> bool {
        let new_bottom = get_lcm(self.bottom, other.bottom);
        let a = self.top * ((new_bottom / self.bottom) as i128);
        let b = other.top * ((new_bottom / other.bottom) as i128);
        a == b
    }
}
impl std::cmp::Eq for Rat {}

#[derive(Clone, PartialEq, Eq)]
struct Vec2 {
    x: Rat,
    y: Rat,
}

impl Vec2 {
    fn new<X: Into<Rat>, Y: Into<Rat>>(x: X, y: Y) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }
}

impl std::fmt::Debug for Vec2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?})", self.x, self.y)
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
    fn xy(&self) -> Vec2 {
        Vec2 {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}
impl std::ops::Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::fmt::Debug for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:?},{:?},{:?})", self.x, self.y, self.z)
    }
}

#[derive(Debug, Clone)]
struct Object {
    position: Vec3,
    velocity: Vec3,
}

fn solve_xy(
    objects: Vec<Object>,
    x_range: RangeInclusive<i128>,
    y_range: RangeInclusive<i128>,
) -> usize {
    let mut total = 0usize;
    for i in 0..objects.len() {
        let a = &objects[i];
        let a_pos = a.position.xy();
        let a_vel = a.velocity.xy();
        for j in (i + 1)..objects.len() {
            let b = &objects[j];
            let b_pos = b.position.xy();
            let b_vel = b.velocity.xy();
            if let Some(intersection) = get_intersection_xy(&a_pos, &a_vel, &b_pos, &b_vel) {
                match intersection {
                    Intersection2D::Point(p, t1, t2) => {
                        if t1.is_non_negative()
                            && t2.is_non_negative()
                            && p.x.in_range(&x_range)
                            && p.y.in_range(&y_range)
                        {
                            total += 1;
                        }
                    }
                    Intersection2D::Line(_, _) => {
                        continue;
                    }
                }
            }
        }
    }
    total
}

enum Intersection2D {
    Point(Vec2, Rat, Rat),
    Line(Vec2, Vec2),
}

fn get_intersection_xy(
    a_pos: &Vec2,
    a_vel: &Vec2,
    b_pos: &Vec2,
    b_vel: &Vec2,
) -> Option<Intersection2D> {
    if a_vel.x.is_zero() && b_vel.x.is_zero() {
        if a_pos.x == b_pos.x {
            return Some(Intersection2D::Line(a_pos.clone(), a_vel.clone()));
        }
        return None;
    }
    if b_vel.x.is_zero() {
        let t = (b_pos.x - a_pos.x) / a_vel.x;
        let y = a_pos.y + t * a_vel.y;
        let t2 = (y - b_pos.y) / b_vel.y;
        return Some(Intersection2D::Point(Vec2::new(b_pos.x, y), t, t2));
    }
    if a_vel.x.is_zero() {
        return get_intersection_xy(b_pos, b_vel, a_pos, a_vel);
    }
    if a_vel.y.is_zero() || b_vel.y.is_zero() {
        return match get_intersection_xy(
            &Vec2::new(a_pos.y, a_pos.x),
            &Vec2::new(a_vel.y, a_vel.x),
            &Vec2::new(b_pos.y, b_pos.x),
            &Vec2::new(b_vel.y, b_vel.x),
        ) {
            Some(Intersection2D::Point(Vec2 { x, y }, t1, t2)) => {
                Some(Intersection2D::Point(Vec2::new(y, x), t1, t2))
            }
            Some(Intersection2D::Line(Vec2 { x: x1, y: y1 }, Vec2 { x: x2, y: y2 })) => {
                Some(Intersection2D::Line(Vec2::new(y1, x1), Vec2::new(y2, x2)))
            }
            None => None,
        };
    }
    if a_vel.x * b_vel.y == a_vel.y * b_vel.x {
        let t1 = (a_pos.x - b_pos.x) / b_vel.x;
        let t2 = (a_pos.y - b_pos.y) / b_vel.y;

        return (t1 == t2).then_some(Intersection2D::Line(a_pos.clone(), a_vel.clone()));
    }

    let b_k = b_vel.y / b_vel.x;
    let d_x = a_pos.x - b_pos.x;
    let d_y = a_pos.y - b_pos.y;
    let bottom = a_vel.x * b_k - a_vel.y;
    let top = d_y - d_x * b_k;
    let m = top / bottom;
    let x = a_pos.x + a_vel.x * m;
    let y = a_pos.y + a_vel.y * m;
    let t = (x - b_pos.x) / b_vel.x;

    Some(Intersection2D::Point(Vec2::new(x, y), m, t))
}

fn parse(input: &str) -> Vec<Object> {
    input.lines().map(parse_object).collect_vec()
}
fn parse_object(input: &str) -> Object {
    let mut it = input
        .split_ascii_whitespace()
        .map(|x| x.trim_end_matches(','))
        .flat_map(|x| x.parse::<i128>().ok());
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

struct Equations {
    lefts: Vec<Vec<Rat>>,
    rights: Vec<Rat>,
}

impl Equations {
    fn multiply_row(&mut self, row_index: usize, k: &Rat) {
        let vars = self.lefts[0].len();
        for j in 0..vars {
            self.lefts[row_index][j] *= k.clone();
        }
        self.rights[row_index] *= k.clone();
    }

    fn sub_row(&mut self, row_index: usize, other_row_index: usize) {
        let vars = self.lefts[0].len();
        for j in 0..vars {
            let new_value = self.lefts[row_index][j] - self.lefts[other_row_index][j];
            self.lefts[row_index][j] = new_value;
        }
        self.rights[row_index] = self.rights[row_index] - self.rights[other_row_index];
    }

    fn solve(&mut self) -> Option<Vec<Rat>> {
        let vars = self.lefts[0].len();
        let eqs = self.rights.len();

        if vars > eqs {
            return None;
        }

        for var_index in 0..vars {
            let non_zero_row = self
                .lefts
                .iter()
                .enumerate()
                .skip(var_index)
                .find(|(_, row)| !row[var_index].is_zero())
                .map(|(i, _)| i)?;
            self.multiply_row(non_zero_row, &self.lefts[non_zero_row][var_index].reverse());
            self.swap_rows(var_index, non_zero_row);
            for eq_index in (var_index + 1)..eqs {
                let own_coef = self.lefts[eq_index][var_index].clone();
                if own_coef.is_zero() {
                    continue;
                }
                let k = own_coef.reverse();

                self.multiply_row(eq_index, &k);
                self.sub_row(eq_index, var_index);
            }
        }

        for var_index in (0..vars).rev() {
            for eq in (0..var_index).rev() {
                let own_coef = self.lefts[eq][var_index].clone();
                if own_coef.is_zero() {
                    continue;
                }
                let k = own_coef.reverse();
                self.multiply_row(var_index, &own_coef);
                self.sub_row(eq, var_index);
                self.multiply_row(var_index, &k);
            }
        }
        Some(self.rights.clone())
    }

    fn swap_rows(&mut self, var_index: usize, non_zero_row: usize) {
        if var_index == non_zero_row {
            return;
        }
        let vars = self.lefts[0].len();
        for j in 0..vars {
            let tmp = self.lefts[var_index][j].clone();
            self.lefts[var_index][j] = self.lefts[non_zero_row][j].clone();
            self.lefts[non_zero_row][j] = tmp;
        }
        let tmp = self.rights[var_index].clone();
        self.rights[var_index] = self.rights[non_zero_row].clone();
        self.rights[non_zero_row] = tmp;
    }
}

pub fn solve_part_2(file_content: &str) -> i64 {
    let objects = parse(file_content);
    let p0 = objects[0].position.clone();
    let p1 = objects[1].position.clone();
    let p2 = objects[2].position.clone();

    let v0 = objects[0].velocity.clone();
    let v1 = objects[1].velocity.clone();
    let v2 = objects[2].velocity.clone();

    let v0x = v0.x;
    let v0y = v0.y;
    let v0z = v0.z;

    let v1x = v1.x;
    let v1y = v1.y;
    let v1z = v1.z;

    let v2x = v2.x;
    let v2y = v2.y;
    let v2z = v2.z;

    let p0x = p0.x;
    let p0y = p0.y;
    let p0z = p0.z;

    let p1x = p1.x;
    let p1y = p1.y;
    let p1z = p1.z;

    let p2x = p2.x;
    let p2y = p2.y;
    let p2z = p2.z;

    let mut equations = Equations {
        lefts: vec![
            vec![
                v0y - v1y,
                -v0x + v1x,
                Rat::ZERO,
                -p0y + p1y,
                p0x - p1x,
                Rat::ZERO,
            ],
            vec![
                v0z - v1z,
                Rat::ZERO,
                -v0x + v1x,
                -p0z + p1z,
                Rat::ZERO,
                p0x - p1x,
            ],
            vec![
                Rat::ZERO,
                v0z - v1z,
                -v0y + v1y,
                Rat::ZERO,
                -p0z + p1z,
                p0y - p1y,
            ],
            vec![
                v0y - v2y,
                -v0x + v2x,
                Rat::ZERO,
                -p0y + p2y,
                p0x - p2x,
                Rat::ZERO,
            ],
            vec![
                v0z - v2z,
                Rat::ZERO,
                -v0x + v2x,
                -p0z + p2z,
                Rat::ZERO,
                p0x - p2x,
            ],
            vec![
                Rat::ZERO,
                v0z - v2z,
                -v0y + v2y,
                Rat::ZERO,
                -p0z + p2z,
                p0y - p2y,
            ],
        ],
        rights: vec![
            p0x * v0y - p0y * v0x - p1x * v1y + p1y * v1x,
            p0x * v0z - p0z * v0x - p1x * v1z + p1z * v1x,
            p0y * v0z - p0z * v0y - p1y * v1z + p1z * v1y,
            p0x * v0y - p0y * v0x - p2x * v2y + p2y * v2x,
            p0x * v0z - p0z * v0x - p2x * v2z + p2z * v2x,
            p0y * v0z - p0z * v0y - p2y * v2z + p2z * v2y,
        ],
    };

    equations
        .solve()
        .expect("Expected to have a solution")
        .into_iter()
        .take(3)
        .sum::<Rat>()
        .top
        .try_into()
        .expect("the result should be an integer with less than 64 bits")
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{parse, solve_part_2, solve_xy};

    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case(EXAMPLE, 7, 21, 2)]
    #[case(ACTUAL, 200000000000000, 400000000000000, 11246)]
    #[ignore]
    fn test_part1(
        #[case] input: &str,
        #[case] min_value: i128,
        #[case] max_value: i128,
        #[case] expected: usize,
    ) {
        let objects = parse(input);
        assert_eq!(
            solve_xy(objects, min_value..=max_value, min_value..=max_value),
            expected
        );
    }

    #[rstest]
    #[case(EXAMPLE, 47)]
    #[case(ACTUAL, 716_599_937_560_103)]
    fn test_part2(#[case] input: &str, #[case] expected: i64) {
        let res = solve_part_2(input);
        assert_eq!(res, expected);
    }
}
