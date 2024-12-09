use std::ops::RangeInclusive;

pub fn get_gcd(mut a: u128, mut b: u128) -> u128 {
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

pub fn get_lcm(a: u128, b: u128) -> u128 {
    a * (b / get_gcd(a, b))
}

#[derive(Copy, Clone)]
pub struct Rat {
    pub top: i128,
    pub bottom: u128,
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

pub struct Equations {
    pub lefts: Vec<Vec<Rat>>,
    pub rights: Vec<Rat>,
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

    pub fn solve(&mut self) -> Option<Vec<Rat>> {
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
        self.rights[var_index] = self.rights[non_zero_row];
        self.rights[non_zero_row] = tmp;
    }
}
