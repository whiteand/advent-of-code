use crate::math::{Rat, Vec2};

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
