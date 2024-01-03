use rand::prelude::*;

use core::panic;
use std::{iter::Sum, ops::RangeInclusive};

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

    fn abs(&self) -> Self {
        Self {
            top: self.top.abs(),
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
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
    fn square(&self) -> Rat {
        self.x * self.x + self.y * self.y + self.z * self.z
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

    fn dot(&self, u: &Vec3) -> Rat {
        self.x * u.x + self.y * u.y + self.z * u.z
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

#[derive(Debug, Clone)]
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

#[derive(Clone, Copy, PartialEq)]
struct Vec3F64 {
    x: f64,
    y: f64,
    z: f64,
}

impl std::fmt::Debug for Vec3F64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(x: {:.2}, y: {:.2}, z: {:.2})", self.x, self.y, self.z)
    }
}

impl Sum for Vec3F64 {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Vec3F64::new(0., 0., 0.), |a, b| a + b)
    }
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

    fn is_zero(&self) -> bool {
        self.x == 0. && self.y == 0. && self.z == 0.
    }

    fn cos(&self, other: &Vec3F64) -> f64 {
        self.dot(other) / (self.len() * other.len())
    }

    fn unit(&self) -> Vec3F64 {
        if self.is_zero() {
            println!("Warning: unit of zero vector");
            return Vec3F64::new(1., 0., 0.);
        }
        let d = self.len();
        Vec3F64 {
            x: self.x / d,
            y: self.y / d,
            z: self.z / d,
        }
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

fn get_closest_distance(obj: &Object, rock: &Object) -> (Vec3, Rat, Rat) {
    let p = obj.position.clone();
    let u = obj.velocity.clone();
    let r = rock.position.clone();
    let v = rock.velocity.clone();

    if v.is_zero() {
        let t = (r.dot(&u) - p.dot(&u)) / u.dot(&u);
        let x = p + u.times(t);
        return (x - r, t, t);
    }

    // p1 = p + u * t1
    // p2 = r + v * t2
    // dpv + uv * t1 - vv * t2  = 0
    // dpu + uu * t1 - vu * t2 = 0
    // t1 = (vu * t2 - dpu) / uu
    // dpv + uv * (vu * t2 - dpu) / uu - vv * t2  = 0
    // dpv uu + uv * vu * t2 - dpu * uv - vv * uu * t2 = 0
    // t2(uv * vu  - vv * uu)  = dpu * uv - dpv uu
    // t2 = (dpu * uv - dpv uu) / (uv * vu  - vv * uu)

    let vv = v.dot(&v);
    let vu = v.dot(&u);
    let uu = u.dot(&u);
    let uv = u.dot(&v);
    let dp = p.clone() - r.clone();
    let dpv = dp.dot(&v);
    let dpu = dp.dot(&u);

    let t2 = (dpu * uv - dpv * uu) / (uv * vu - vv * uu);
    let t1 = (vu * t2 - dpu) / uu;

    let p1 = p + u.times(t1);
    let p2 = r + v.times(t2);

    return (p2 - p1, t1, t2);
}
fn get_closest_distance_f64(obj: &ObjectF64, rock: &ObjectF64) -> (Vec3F64, f64, f64) {
    let p = obj.position.clone();
    let u = obj.velocity.clone();
    let r = rock.position.clone();
    let v = rock.velocity.clone();

    if u.is_zero() && v.is_zero() {
        let dp = p - r;
        return (dp, 0., 0.);
    }

    if v.is_zero() {
        let t = (r.dot(&u) - p.dot(&u)) / u.dot(&u);
        let x = p + u.times(t);
        return ((x - r), t, t);
    }

    if u.is_zero() {
        let (dv, t2, t1) = get_closest_distance_f64(rock, obj);
        return (dv.times(-1.), t1, t2);
    }

    let vv = v.dot(&v);
    let uu = u.dot(&u);
    let uv = u.dot(&v);

    if (uv * uv - vv * uu).abs() == 0. {
        let t = (p - r).dot(&v) / v.len();

        if t < 0. {
            let m = p + u.times(-t);
            let d = r - m;
            return (d, -t, 0.);
        }

        let m = r + v.times(t);
        let d = m - p;
        return (d, 0., t);
    }

    let dp = p.clone() - r.clone();
    let dpv = dp.dot(&v);
    let dpu = dp.dot(&u);

    let t2 = (dpu * uv - dpv * uu) / (uv * uv - vv * uu);
    let t1 = (uv * t2 - dpu) / uu;

    let p1 = p + u.times(t1);
    let p2 = r + v.times(t2);

    return (p2 - p1, t1, t2);
}

fn get_error(objects: &[ObjectF64], times: &[f64; 2], line: &mut ObjectF64) -> f64 {
    line.position = objects[0].position_at(times[0]);
    line.velocity = (objects[1].position_at(times[1]) - line.position).unit();

    let mut negative_cost = 0.;

    if times[0] < 0. {
        negative_cost += times[0].abs();
    }

    if times[1] < 0. {
        negative_cost += times[1].abs();
    }

    objects
        .iter()
        .map(|o| {
            let (d, _, _) = get_closest_distance_f64(o, line);
            d.len()
        })
        .sum::<f64>()
        + negative_cost
}

fn minimize(mut left: f64, mut right: f64, mut f: impl FnMut(f64) -> f64) -> f64 {
    let mut right_v = f(right);
    let mut mid = (left + right) / 2.;
    let mut mid_v = f(mid);

    while mid_v > right_v {
        right *= 1.618;
        right_v = f(right);
        mid = (left + right) / 2.;
        mid_v = f(mid);
    }

    while right > left + 1e-9 {
        let t1 = left + (right - left) * (1. - 0.618);
        let t2 = left + (right - left) * 0.618;
        let t1_v = f(t1);
        let t2_v = f(t2);

        if t1_v > t2_v {
            left = t1;
        } else {
            right = t2;
        }
    }

    right
}

pub fn solve_part_2(file_content: &str) -> i64 {
    let mut r = rand::thread_rng();
    let objects = parse(file_content)
        .into_iter()
        .map(|o| ObjectF64 {
            position: o.position.as_f64(),
            velocity: o.velocity.as_f64(),
        })
        .collect_vec();

    // let (_, t1, t2) = get_closest_distance_f64(&objects[0], &objects[1]);

    let mut times = [5., 3.];
    let mut line = ObjectF64 {
        position: Vec3F64::new(0., 0., 0.),
        velocity: Vec3F64::new(0., 0., 0.),
    };

    loop {
        let t0 = times[0];
        let new_t1 = minimize(0., 1., |t| get_error(&objects[2..], &[t0, t], &mut line));
        times[1] = new_t1;

        let t1 = times[1];
        let new_t0 = minimize(0., 1., |t| get_error(&objects[2..], &[t, t1], &mut line));
        times[0] = new_t0;

        println!("t0: {:.2}, t1: {:.2}", times[0], times[1]);
        let error = get_error(&objects[2..], &times, &mut line);
        if error < 1e-9 {
            break;
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::{parse, solve_part_2, solve_xy, ObjectF64, Vec3F64};

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
    #[case(
        Vec3F64::new(0., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(0., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        (Vec3F64::new(0., 0., 0.), 0., 0.)
    )]
    #[case(
        Vec3F64::new(0., 0., 1.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(0., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        (Vec3F64::new(0., 0., -1.), 0., 0.)
    )]
    #[case(
        Vec3F64::new(0., 0., 2.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        (Vec3F64::new(0., 0., -2.), 1., 0.)
    )]
    #[case(
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(0., 0., 1.),
        Vec3F64::new(1., 0., 0.),
        (Vec3F64::new(0., 0., 1.), 0., 1.)
    )]
    #[case(
        Vec3F64::new(0., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        (Vec3F64::new(0., 0., 0.), 1., 0.)
    )]
    #[case(
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(0., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        (Vec3F64::new(0., 0., 0.), 0., 1.)
    )]
    #[case(
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(1., 0., 0.),
        Vec3F64::new(0., 1., 1.),
        Vec3F64::new(0., 1., 0.),
        (Vec3F64::new(0., 0., 1.), -1., -1.)
    )]
    #[case(
        Vec3F64::new(0., 0., 0.),
        Vec3F64::new(3., 4., 0.),
        Vec3F64::new(0., 0., 1.),
        Vec3F64::new(1., 1., 0.),
        (Vec3F64::new(0., 0., 1.), 0., 0.)
    )]
    fn test_distance(
        #[case] a_pos: Vec3F64,
        #[case] a_vel: Vec3F64,
        #[case] b_pos: Vec3F64,
        #[case] b_vel: Vec3F64,
        #[case] expected: (Vec3F64, f64, f64),
    ) {
        let res = super::get_closest_distance_f64(
            &ObjectF64 {
                position: a_pos,
                velocity: a_vel,
            },
            &ObjectF64 {
                position: b_pos,
                velocity: b_vel,
            },
        );
        assert_eq!(res, expected);
    }

    #[rstest]
    #[case(EXAMPLE, 47)]
    // #[case(ACTUAL, 0)]
    fn test_part2(#[case] input: &str, #[case] expected: i64) {
        let res = solve_part_2(input);
        assert_eq!(res, expected);
    }
}
