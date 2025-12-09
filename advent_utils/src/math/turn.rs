use std::cmp::Ordering;

use derive_more::derive::IsVariant;
use glam::I64Vec2;

#[derive(Debug, Copy, Clone, PartialEq, Eq, IsVariant)]
pub enum TurnOrientation {
    RightIsFromOxToOy,
    RightIsFromOyToOx,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, IsVariant)]
pub enum Turn {
    Right,
    Left,
    None,
}

impl Turn {
    pub fn opposite(self) -> Self {
        match self {
            Turn::Right => Turn::Left,
            Turn::Left => Turn::Right,
            Turn::None => Turn::None,
        }
    }
}

pub fn get_turn(
    orientation: TurnOrientation,
    prev_direction: I64Vec2,
    new_direction: I64Vec2,
) -> Turn {
    match (
        (prev_direction.x * new_direction.y - prev_direction.y * new_direction.x).cmp(&0),
        orientation,
    ) {
        (Ordering::Equal, _) => Turn::None,
        (Ordering::Greater, TurnOrientation::RightIsFromOxToOy) => Turn::Right,
        (Ordering::Less, TurnOrientation::RightIsFromOxToOy) => Turn::Left,
        (Ordering::Greater, TurnOrientation::RightIsFromOyToOx) => Turn::Left,
        (Ordering::Less, TurnOrientation::RightIsFromOyToOx) => Turn::Right,
    }
}

#[cfg(test)]
mod tests {
    use glam::I64Vec2;

    use super::{get_turn, Turn, TurnOrientation};

    #[test]
    fn test_get_turn() {
        assert_eq!(
            get_turn(TurnOrientation::RightIsFromOxToOy, I64Vec2::X, I64Vec2::X,),
            Turn::None
        );
        assert_eq!(
            get_turn(TurnOrientation::RightIsFromOxToOy, I64Vec2::X, I64Vec2::Y),
            Turn::Right
        );
        assert_eq!(
            get_turn(TurnOrientation::RightIsFromOxToOy, I64Vec2::Y, I64Vec2::X),
            Turn::Left
        );
        assert_eq!(
            get_turn(TurnOrientation::RightIsFromOyToOx, I64Vec2::X, I64Vec2::Y),
            Turn::Left
        );
        assert_eq!(
            get_turn(TurnOrientation::RightIsFromOyToOx, I64Vec2::Y, I64Vec2::X),
            Turn::Right
        );
    }
}
