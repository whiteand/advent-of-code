use std::{
    num::TryFromIntError,
    ops::{AddAssign, DivAssign, RangeInclusive},
};

use crate::math::{get_gcd, get_gcd_i, get_lcm};

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

macro_rules! happy_path_mul {
    ($a:expr, $b:expr, $d:expr) => {
        if $a.bottom == 1 {
            $d.checked_set($a.top * $b.top, $b.bottom);
            return;
        }
    };
}
impl std::ops::MulAssign for Rat {
    fn mul_assign(&mut self, rhs: Self) {
        happy_path_mul!(self, rhs, self);
        happy_path_mul!(rhs, self, self);

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
