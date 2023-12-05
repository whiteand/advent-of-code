#[derive(Clone, Copy)]
pub enum Direction {
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn is_down(&self) -> bool {
        matches!(self, Direction::Down)
    }
}

impl std::fmt::Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => write!(f, "<"),
            Self::Right => write!(f, ">"),
            Self::Down => write!(f, "v"),
        }
    }
}

impl TryFrom<char> for Direction {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '<' => Ok(Direction::Left),
            '>' => Ok(Direction::Right),
            'v' => Ok(Direction::Down),
            _ => Err("invalid direction char".to_owned()),
        }
    }
}
