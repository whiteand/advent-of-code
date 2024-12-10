pub(crate) enum Direction {
    Up,
    Right,
    Down,
    Left,
}

pub struct Move {
    pub(crate) direction: Direction,
    pub(crate) distance: u32,
}

impl std::fmt::Debug for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        match self.direction {
            Up => write!(f, "U {}", self.distance),
            Right => write!(f, "R {}", self.distance),
            Down => write!(f, "D {}", self.distance),
            Left => write!(f, "L {}", self.distance),
        }
    }
}

impl Move {
    #[inline]
    pub fn apply(&mut self, x: &mut i32, y: &mut i32) {
        match self.direction {
            Direction::Up => {
                *y -= 1;
            }
            Direction::Right => {
                *x += 1;
            }
            Direction::Down => {
                *y += 1;
            }
            Direction::Left => {
                *x -= 1;
            }
        }
        self.distance -= 1;
    }
}
