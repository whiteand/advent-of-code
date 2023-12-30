use rand::prelude::*;

use std::{array::IntoIter, ops::RangeInclusive};

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

    fn ceil(&self) -> Rat {
        let sign = self.signum();
        let mut top = self.top.abs() as u128;
        top += top % self.bottom;
        let value = top / self.bottom;
        Rat::new((value as i128) * sign, 1)
    }

    fn as_f64(&self) -> f64 {
        self.top as f64 / self.bottom as f64
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
        write!(f, "({:?}, {:?})", self.x, self.y)
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
    fn times(&self, k: impl Into<Rat>) -> Self {
        let k = k.into();
        Self {
            x: self.x.clone() * k,
            y: self.y.clone() * k,
            z: self.z.clone() * k,
        }
    }
    fn as_f64(&self) -> Vec3F64 {
        Vec3F64 {
            x: self.x.as_f64(),
            y: self.y.as_f64(),
            z: self.z.as_f64(),
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
        write!(f, "({:?}, {:?}, {:?})", self.x, self.y, self.z)
    }
}

#[derive(Debug)]
struct Object {
    position: Vec3,
    velocity: Vec3,
}

impl Object {
    fn position_at(&self, t: impl Into<Rat>) -> Vec3 {
        let t: Rat = t.into();
        Vec3 {
            x: self.position.x + self.velocity.x * t,
            y: self.position.y + self.velocity.y * t,
            z: self.position.z + self.velocity.z * t,
        }
    }
    fn position_at_f64(&self, t: f64) -> Vec3F64 {
        let x = self.position.x.as_f64();
        let y = self.position.y.as_f64();
        let z = self.position.z.as_f64();

        let vx = self.velocity.x.as_f64();
        let vy = self.velocity.y.as_f64();
        let vz = self.velocity.z.as_f64();

        let x = x + vx * t;
        let y = y + vy * t;
        let z = z + vz * t;

        Vec3F64::new(x, y, z)
    }
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

fn get_intersection_3d(
    a_pos: &Vec3,
    a_vel: &Vec3,
    b_pos: &Vec3,
    b_vel: &Vec3,
) -> Option<(Rat, Rat)> {
    let mut a_pos_2 = Vec2::new(a_pos.x, a_pos.y);
    let mut a_vel_2 = Vec2::new(a_vel.x.clone(), a_vel.y.clone());
    let mut b_pos_2 = Vec2::new(b_pos.x.clone(), b_pos.y.clone());
    let mut b_vel_2 = Vec2::new(b_vel.x.clone(), b_vel.y.clone());
    if let Some(Intersection2D::Point(_, t1, t2)) =
        get_intersection_xy(&a_pos_2, &a_vel_2, &b_pos_2, &b_vel_2)
    {
        if t1.is_non_negative() && t2.is_non_negative() {
            return Some((t1, t2));
        }
    }

    a_pos_2.x = a_pos.x;
    a_pos_2.y = a_pos.z;

    a_vel_2.x = a_vel.x.clone();
    a_vel_2.y = a_vel.z.clone();

    b_pos_2.x = b_pos.x.clone();
    b_pos_2.y = b_pos.z.clone();

    b_vel_2.x = b_vel.x.clone();
    b_vel_2.y = b_vel.z.clone();

    if let Some(Intersection2D::Point(_, t1, t2)) =
        get_intersection_xy(&a_pos_2, &a_vel_2, &b_pos_2, &b_vel_2)
    {
        if t1.is_non_negative() && t2.is_non_negative() {
            return Some((t1, t2));
        }
    }

    a_pos_2.x = a_pos.y;
    a_pos_2.y = a_pos.z;

    a_vel_2.x = a_vel.y.clone();
    a_vel_2.y = a_vel.z.clone();

    b_pos_2.x = b_pos.y.clone();
    b_pos_2.y = b_pos.z.clone();

    b_vel_2.x = b_vel.y.clone();
    b_vel_2.y = b_vel.z.clone();

    if let Some(Intersection2D::Point(_, t1, t2)) =
        get_intersection_xy(&a_pos_2, &a_vel_2, &b_pos_2, &b_vel_2)
    {
        if t1.is_non_negative() && t2.is_non_negative() {
            return Some((t1, t2));
        }
    }

    None
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

#[derive(Debug, Clone, PartialEq)]
struct Vec3F64 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3F64 {
    fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    fn times(&self, t: f64) -> Vec3F64 {
        Vec3F64 {
            x: self.x * t,
            y: self.y * t,
            z: self.z * t,
        }
    }

    fn dot(&self, v: &Vec3F64) -> f64 {
        self.x * v.x + self.y * v.y + self.z * v.z
    }
}

impl std::ops::Add for Vec3F64 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl std::ops::Sub for Vec3F64 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl std::ops::SubAssign for Vec3F64 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

#[derive(Debug, Clone)]
struct ObjectF64 {
    position: Vec3F64,
    velocity: Vec3F64,
}

impl ObjectF64 {
    fn position_at(&self, t: f64) -> Vec3F64 {
        self.position.clone() + self.velocity.times(t)
    }

    fn as_rat(&self) -> Object {
        Object {
            position: Vec3::new(
                (self.position.x.round()) as i128,
                (self.position.y.round()) as i128,
                (self.position.z.round()) as i128,
            ),
            velocity: Vec3::new(
                (self.position.x.round()) as i128,
                (self.position.y.round()) as i128,
                (self.position.z.round()) as i128,
            ),
        }
    }
}

#[derive(Debug, Clone)]
struct Weights {
    weights: [f64; 6],
    gradient: [f64; 6],
}

impl Weights {
    fn adjust(&mut self, position_step: f64, time_step: f64, objects_f64: &[ObjectF64]) -> f64 {
        let error = self.calculate_gradients(position_step, time_step, objects_f64);
        self.move_weights(position_step, time_step);
        error
    }

    fn move_weights(&mut self, position_step: f64, time_step: f64) {
        for i in 0..self.weights.len() {
            let step = if i > 2 { time_step } else { position_step };
            self.weights[i] = self.weights[i] - self.gradient[i] * step;
        }
    }

    fn calculate_gradients(
        &mut self,
        position_step: f64,
        time_step: f64,
        objects_f64: &[ObjectF64],
    ) -> f64 {
        let error = self.get_error(&objects_f64);
        for i in 0..self.weights.len() {
            let step = if i > 2 { time_step } else { position_step };
            let old = self.weights[i];
            self.weights[i] = old + step;
            let new_err = self.get_error(&objects_f64);
            self.weights[i] = old;
            let gradient = (new_err - error) / step;
            if gradient < -1e4 {
                self.gradient[i] = -1.;
                continue;
            } else if gradient > 1e4 {
                self.gradient[i] = 1.;
                continue;
            } else {
                self.gradient[i] = gradient;
            }
        }
        error
    }
    fn get_error(&self, objects_f64: &[ObjectF64]) -> f64 {
        let position = Vec3F64::new(self.weights[0], self.weights[1], self.weights[2]);
        let velocity = Vec3F64::new(self.weights[3], self.weights[4], self.weights[5]);
        let rock = ObjectF64 { position, velocity };
        let mut err = 0f64;
        for obj in objects_f64 {
            let min_distance_to_obj_trajectory = get_min_distance_to_trajectory(obj, &rock);
            err = err + min_distance_to_obj_trajectory;
        }
        err
    }
}

fn get_min_distance_to_trajectory(obj: &ObjectF64, rock: &ObjectF64) -> f64 {
    // We are using geometric formula to find the shortest distance between two straight lines
    // Lines are set by p0 and v0 and p1 and v1
    // https://en.wikipedia.org/wiki/Skew_lines#Distance

    let p = obj.position.clone();
    let u = obj.velocity.clone();
    let r = rock.position.clone();
    let v = rock.velocity.clone();

    let dot = u.dot(&v).abs();

    let pr = r - p;

    let d = pr.x * (u.y * v.z - u.z * v.y) - pr.y * (u.x * v.z - u.z * v.x)
        + pr.z * (u.x * v.y - u.y * v.x);

    let d = d.abs();

    return d / dot;
}

fn get_throw_position_sum(objects: &[Object]) -> i64 {
    let mut rand_generator = rand::thread_rng();
    let objects_f64 = objects
        .iter()
        .map(|x| ObjectF64 {
            position: x.position.as_f64(),
            velocity: x.velocity.as_f64(),
        })
        .collect_vec();

    let mut w = Weights {
        weights: [
            132777695020144.16,
            -84061531420895.06,
            76547399128688.84,
            0.19388713982967876,
            0.552358292295493,
            0.2794689019401747,
        ],
        gradient: [0f64; 6],
    };

    let mut it = 0;
    let mut min_error = 143812995896221140.;
    let mut last_error = f64::MAX;
    let mut pos_step = 1e3;
    loop {
        it += 1;
        let error = w.calculate_gradients(pos_step, 0.0001, &objects_f64);

        w.move_weights(pos_step, 0.0001);

        if it % 100000 == 0 {
            println!(
                "Error: {error} / {:.2}%,\t{:?}",
                (error - min_error) / error * 100.,
                &w
            );
        }
        if error < min_error {
            min_error = error;
        }
        if error > last_error {
            pos_step /= 2.;
        } else {
            pos_step *= 1.1;
        }

        if error < 0.1e-4 {
            break;
        }
    }

    w.weights
        .into_iter()
        .take(3)
        .map(|x| x.round() as i64)
        .sum()
}

pub fn solve_part_2(file_content: &str) -> i64 {
    let objects = parse(file_content);

    get_throw_position_sum(&objects)
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
    // #[case(EXAMPLE, 47)]
    #[case(ACTUAL, 0)]
    fn test_part2(#[case] input: &str, #[case] expected: i64) {
        let res = solve_part_2(input);
        assert_eq!(res, expected);
    }
}
