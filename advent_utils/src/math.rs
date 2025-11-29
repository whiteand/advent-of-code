mod per_non_diagonal;
mod rat;
mod rat_vec2;
mod rat_vec3;
mod systems;

use glam::IVec2;
pub use per_non_diagonal::PerNonDiagonalDirection;
pub use rat::Rat;
pub use rat_vec2::Vec2;
pub use rat_vec3::Vec3;
pub use systems::{solve_system, Equations, SolveError};

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

pub trait LinearProgressionElement: Copy {
    fn linear_progression_add(self, other: Self) -> Self;
    fn linear_progression_mul(self, times: usize) -> Self;
}

macro_rules! impl_linear_progression_for_nums {
    ($($t:ty),*) => {
        $(impl LinearProgressionElement for $t {
            fn linear_progression_add(self, other: Self) -> Self {
                self + other
            }
            fn linear_progression_mul(self, other: usize) -> Self {
                if Self::MAX as usize > other {
                    panic!("Too large multiplier");
                }
                self * (other as $t)
            }
        })*
    };
}

impl_linear_progression_for_nums! {u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize}

impl LinearProgressionElement for IVec2 {
    fn linear_progression_add(self, other: Self) -> Self {
        self + other
    }

    fn linear_progression_mul(self, times: usize) -> Self {
        self.wrapping_mul(IVec2::splat(times as i32))
    }
}

#[derive(Clone)]
pub struct LinearProgression<T> {
    pub current: T,
    pub remaining: usize,
    pub inc: T,
}

impl<T: LinearProgressionElement> LinearProgression<T> {
    #[inline]
    fn get(&self, index: usize) -> T {
        self.current
            .linear_progression_add(self.inc.linear_progression_mul(index))
    }
}

impl<T> LinearProgression<T> {
    pub fn new(init: T, inc: T, remaining: usize) -> Self
    where
        T: Clone,
    {
        Self {
            current: init.clone(),
            inc,
            remaining,
        }
    }
    pub fn new_unbound(init: T, inc: T) -> Self
    where
        T: Clone,
    {
        Self {
            current: init.clone(),
            inc,
            remaining: usize::MAX,
        }
    }
}

impl<T: LinearProgressionElement> Iterator for LinearProgression<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining <= 0 {
            None
        } else {
            let value = self.current;
            self.current = self.current.linear_progression_add(self.inc);
            self.remaining -= 1;
            Some(value)
        }
    }

    fn nth(&mut self, index: usize) -> Option<Self::Item> {
        if index >= self.remaining {
            self.remaining = 0;
            None
        } else {
            let value = self.get(index);
            self.current = value.linear_progression_add(self.inc);
            self.remaining -= index + 1;
            Some(value)
        }
    }

    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.nth(self.remaining - 1)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl<T: LinearProgressionElement> ExactSizeIterator for LinearProgression<T> {
    fn len(&self) -> usize {
        self.remaining
    }
}

#[cfg(test)]
mod tests {
    use crate::math::LinearProgression;

    #[test]
    fn test_lin_progression_next() {
        let mut a = LinearProgression::new(0, 2, 10);
        assert_eq!(a.next(), Some(0));
        assert_eq!(a.next(), Some(2));
        assert_eq!(a.next(), Some(4));
        assert_eq!(a.next(), Some(6));
        assert_eq!(a.next(), Some(8));
        assert_eq!(a.next(), Some(10));
        assert_eq!(a.next(), Some(12));
        assert_eq!(a.next(), Some(14));
        assert_eq!(a.next(), Some(16));
        assert_eq!(a.next(), Some(18));
        assert_eq!(a.next(), None);
    }
    #[test]
    fn test_lin_progression_nth() {
        let mut a = LinearProgression::new(0, 2, 10);
        assert_eq!(a.nth(3), Some(6));
        assert_eq!(a.nth(5), Some(18));
        assert_eq!(a.next(), None);
    }
}
