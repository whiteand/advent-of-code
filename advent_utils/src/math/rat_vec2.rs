use crate::math::Rat;

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
