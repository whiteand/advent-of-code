use super::direction::Direction;

pub fn parse(file: &str) -> impl Iterator<Item = Direction> + '_ {
    file.chars().map(|ch| ch.try_into()).map(Result::unwrap)
}
