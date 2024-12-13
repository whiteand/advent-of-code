use std::{
    num::TryFromIntError,
    ops::{AddAssign, DivAssign, RangeInclusive},
};

use thiserror::Error;

pub fn get_gcd(mut a: u128, mut b: u128) -> u128 {
    if b == 1 {
        return 1;
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
pub fn get_gcd_i(mut a: i128, mut b: i128) -> u128 {
    if a == 0 {
        return b.unsigned_abs();
    }
    if b == 0 {
        return a.unsigned_abs();
    }
    if a < 0 {
        a = -a;
    }
    if b < 0 {
        b = -b;
    }

    get_gcd(a.try_into().unwrap(), b.try_into().unwrap())
}

pub fn get_lcm(a: u128, b: u128) -> u128 {
    a * (b / get_gcd(a, b))
}

#[derive(Copy, Clone)]
pub struct Rat {
    pub top: i128,
    pub bottom: u128,
}

impl Rat {
    #[inline(always)]
    fn checked_set(&mut self, top: i128, bottom: u128) {
        let gcd = get_gcd(top.unsigned_abs(), bottom);
        if gcd == 1 {
            self.top = top;
            self.bottom = bottom;
        } else {
            self.top = top / (gcd as i128);
            self.bottom = bottom / gcd;
        }
    }
}

impl std::iter::Sum for Rat {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut initial = Self::ZERO;
        for x in iter {
            initial += x;
        }
        initial
    }
}

impl Rat {
    pub const ZERO: Self = Self { top: 0, bottom: 1 };
    pub const ONE: Self = Self { top: 1, bottom: 1 };
    pub const MINUS_ONE: Self = Self { top: -1, bottom: 1 };
    pub fn new(top: i128, bottom: u128) -> Self {
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
    pub fn reverse(&self) -> Rat {
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
    pub fn is_non_negative(&self) -> bool {
        self.top >= 0
    }
    pub fn is_zero(&self) -> bool {
        self.top == 0
    }

    pub fn in_range(&self, range: &RangeInclusive<i128>) -> bool {
        let min = *range.start();
        let max = *range.end();
        let bottom = self.bottom as i128;
        min * bottom <= self.top && self.top <= max * bottom
    }

    pub fn signum(&self) -> i128 {
        match self.top {
            0 => 0,
            1.. => 1,
            _ => -1,
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

    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}
impl std::ops::Mul for Rat {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

impl std::ops::AddAssign for Rat {
    fn add_assign(&mut self, rhs: Self) {
        if self.bottom == rhs.bottom {
            self.checked_set(self.top + rhs.top, self.bottom);
            return;
        }
        let bottom = get_lcm(self.bottom, rhs.bottom);
        let top =
            self.top * ((bottom / self.bottom) as i128) + rhs.top * ((bottom / rhs.bottom) as i128);
        self.checked_set(top, bottom);
    }
}

impl std::ops::MulAssign for Rat {
    fn mul_assign(&mut self, rhs: Self) {
        if self.bottom == 1 {
            self.checked_set(self.top * rhs.top, rhs.bottom);
            return;
        }
        let left_g = get_gcd_i(self.top, rhs.bottom as i128) as i128;
        let right_g = get_gcd_i(self.bottom as i128, rhs.top);
        let top = self.top / left_g * (rhs.top / (right_g as i128));
        let bottom = self.bottom / right_g * (rhs.bottom / left_g as u128);
        self.checked_set(top, bottom);
    }
}

impl std::ops::Div for Rat {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self.div_assign(rhs);
        self
    }
}
impl std::ops::DivAssign for Rat {
    fn div_assign(&mut self, rhs: Self) {
        if rhs.top == 0 && self.top == 0 {
            self.top = 1;
            self.bottom = 1;
        } else {
            let new_sign = self.signum() * rhs.signum();
            let top = self.top.abs() * (rhs.bottom as i128) * new_sign;
            let bottom = self.bottom * (rhs.top.unsigned_abs());
            self.checked_set(top, bottom);
        }
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
        if self.bottom == rhs.bottom {
            self.checked_set(self.top - rhs.top, self.bottom);
            return;
        }
        let bottom = get_lcm(self.bottom, rhs.bottom);
        let top =
            self.top * ((bottom / self.bottom) as i128) - rhs.top * ((bottom / rhs.bottom) as i128);
        self.checked_set(top, bottom);
    }
}

impl From<i128> for Rat {
    fn from(i: i128) -> Self {
        Self { top: i, bottom: 1 }
    }
}

macro_rules! impl_from_num {
    ($($typ:ty),+) => {
        $(
            impl From<$typ> for Rat {
                fn from(i: $typ) -> Self {
                    Self {
                        top: i as i128,
                        bottom: 1,
                    }
                }
            }
        )+
    };
}
impl_from_num!(u64, i64, u32, i32, u16, i16, u8, i8, usize, isize);
impl TryFrom<u128> for Rat {
    type Error = TryFromIntError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        i128::try_from(value).map(|top| Rat { top, bottom: 1 })
    }
}

macro_rules! try_from_rat_to_num {
    ($($typ:ty),+) => {
        $(
            impl TryFrom<Rat> for $typ {
                type Error = ();
                fn try_from(r: Rat) -> Result<Self, Self::Error> {
                    if r.bottom != 1 {
                        return Err(());
                    }
                    r.top.try_into().map_err(|_| ())
                }
            }
        )+
    };
}

try_from_rat_to_num!(i64, i32, i16, i8, isize, u128, u64, u32, u16, u8, usize);

impl TryFrom<Rat> for i128 {
    type Error = ();
    fn try_from(r: Rat) -> Result<Self, Self::Error> {
        if r.bottom != 1 {
            return Err(());
        }
        Ok(r.top)
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
pub struct Vec2 {
    pub x: Rat,
    pub y: Rat,
}

impl Vec2 {
    pub fn new<X: Into<Rat>, Y: Into<Rat>>(x: X, y: Y) -> Self {
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
pub struct Vec3 {
    pub x: Rat,
    pub y: Rat,
    pub z: Rat,
}

impl Vec3 {
    pub fn new<X: Into<Rat>, Y: Into<Rat>, Z: Into<Rat>>(x: X, y: Y, z: Z) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
            z: z.into(),
        }
    }
    pub fn xy(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.y,
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

#[derive(Debug, Error)]
pub enum SolveError {
    #[error("lefts should have the same len as rights")]
    IncompatibleLeftAndRight,
    #[error("failed to determine the variable at index {0}")]
    CannotSolveFor(usize),
}

pub struct Equations<'t> {
    pub lefts: &'t mut [&'t mut [Rat]],
    pub rights: &'t mut [Rat],
}

impl Equations<'_> {
    fn multiply_row(&mut self, row_index: usize, k: &Rat) {
        let vars = self.lefts[0].len();
        for j in 0..vars {
            self.lefts[row_index][j] *= *k;
        }
        self.rights[row_index] *= *k;
    }

    fn sub_row(&mut self, row_index: usize, other_row_index: usize) {
        let vars = self.lefts[0].len();
        for j in 0..vars {
            let new_value = self.lefts[row_index][j] - self.lefts[other_row_index][j];
            self.lefts[row_index][j] = new_value;
        }

        self.rights[row_index] -= self.rights[other_row_index];
    }

    pub fn solve(mut self) -> Result<(), SolveError> {
        let vars = self.lefts[0].len();
        let eqs = self.rights.len();

        if vars > eqs {
            return Err(SolveError::IncompatibleLeftAndRight);
        }
        for var_index in 0..vars {
            let Some(non_zero_row) = self
                .lefts
                .iter()
                .enumerate()
                .skip(var_index)
                .find(|(_, row)| !row[var_index].is_zero())
                .map(|(i, _)| i)
            else {
                return Err(SolveError::CannotSolveFor(var_index));
            };
            self.multiply_row(non_zero_row, &self.lefts[non_zero_row][var_index].reverse());
            self.swap_rows(var_index, non_zero_row);
            for eq_index in (var_index + 1)..eqs {
                let own_coef = self.lefts[eq_index][var_index];
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
                let own_coef = self.lefts[eq][var_index];
                if own_coef.is_zero() {
                    continue;
                }
                let k = own_coef.reverse();
                self.multiply_row(var_index, &own_coef);
                self.sub_row(eq, var_index);
                self.multiply_row(var_index, &k);
            }
        }
        Ok(())
    }

    fn swap_rows(&mut self, var_index: usize, non_zero_row: usize) {
        if var_index == non_zero_row {
            return;
        }
        let vars = self.lefts[0].len();
        for j in 0..vars {
            let tmp = self.lefts[var_index][j];
            self.lefts[var_index][j] = self.lefts[non_zero_row][j];
            self.lefts[non_zero_row][j] = tmp;
        }
        self.rights.swap(var_index, non_zero_row);
    }
}

pub fn solve_system<const VARS: usize, const EQS: usize>(
    mut lefts: [[Rat; VARS]; EQS],
    mut rights: [Rat; EQS],
) -> Option<[Rat; EQS]> {
    let mut lefts: [&mut [Rat]; EQS] =
        array_init::from_iter(lefts.iter_mut().map(|x| x.as_mut())).unwrap();
    let rights_mut = rights.as_mut();
    Equations {
        lefts: &mut lefts,
        rights: rights_mut,
    }
    .solve()
    .ok()?;

    Some(rights)
}
