use std::fmt::Write;

use advent_utils::{declare_field, glam::IVec2, nom::AsBytes};
use itertools::Itertools;
use tracing::info;

#[tracing::instrument(ret)]
pub fn part1(passcode: &str) -> String {
    let map = Map {
        passcode: passcode.trim(),
    };
    for x in
        pathfinding::prelude::dijkstra_reach(&(IVec2::new(0, 0), Vec::new()), |(pos, path), _| {
            let pos = *pos;
            let doors = map.get_doors(path.iter().copied());
            let prev_path = path.clone();
            info!(?pos, ?path, %doors);
            doors
                .filter_map(move |t| {
                    let next_pos = pos + t.to_vec();

                    (((0..=3i32).contains(&next_pos.y)) && (0..=3i32).contains(&next_pos.x))
                        .then_some((next_pos, t))
                })
                .map(move |(next_pos, t)| {
                    let mut new_path = prev_path.clone();
                    new_path.push(t);
                    ((next_pos, new_path), 1)
                })
        })
    {
        if x.node.0 == IVec2::new(3, 3) {
            return x.node.1.into_iter().map(|x| x.to_letter() as char).join("");
        }
    }

    "".to_owned()
}
#[tracing::instrument(ret)]
pub fn part2(passcode: &str) -> usize {
    let map = Map {
        passcode: passcode.trim(),
    };
    let mut max_path_len = 0;
    for x in
        pathfinding::prelude::dijkstra_reach(&(IVec2::new(0, 0), Vec::new()), |(pos, path), _| {
            let pos = *pos;
            let doors = map.get_doors(path.iter().copied());
            let prev_path = path.clone();
            info!(?pos, ?path, %doors);
            doors
                .filter(move |_| pos != IVec2::new(3, 3))
                .filter_map(move |t| {
                    let next_pos = pos + t.to_vec();

                    (((0..=3i32).contains(&next_pos.y)) && (0..=3i32).contains(&next_pos.x))
                        .then_some((next_pos, t))
                })
                .map(move |(next_pos, t)| {
                    let mut new_path = prev_path.clone();
                    new_path.push(t);
                    ((next_pos, new_path), 1)
                })
        })
    {
        if x.node.0 == IVec2::new(3, 3) {
            max_path_len = x.total_cost;
        }
    }

    max_path_len
}

struct Map<'t> {
    passcode: &'t str,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct Doors(u8);

impl std::fmt::Display for Doors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in (*self).map(|x| x.to_letter() as char) {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Iterator for Doors {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        if self.open_down() {
            *self = self.set_down(false);
            return Some(Turn::Down);
        }
        if self.open_right() {
            *self = self.set_right(false);
            return Some(Turn::Right);
        }
        if self.open_up() {
            *self = self.set_up(false);
            return Some(Turn::Up);
        }
        if self.open_left() {
            *self = self.set_left(false);
            return Some(Turn::Left);
        }
        None
    }
}

impl Doors {
    declare_field!(u8, bool, open_up, set_up, 0);
    declare_field!(u8, bool, open_down, set_down, 1);
    declare_field!(u8, bool, open_left, set_left, 2);
    declare_field!(u8, bool, open_right, set_right, 3);
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u8)]
enum Turn {
    Up = 0b00,
    Down = 0b01,
    Left = 0b10,
    Right = 0b11,
}

impl std::fmt::Debug for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_letter() as char)
    }
}
impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_letter() as char)
    }
}

impl Turn {
    fn to_vec(&self) -> IVec2 {
        match self {
            Turn::Up => IVec2::NEG_Y,
            Turn::Down => IVec2::Y,
            Turn::Left => IVec2::NEG_X,
            Turn::Right => IVec2::X,
        }
    }
    fn to_letter(&self) -> u8 {
        match self {
            Turn::Up => b'U',
            Turn::Down => b'D',
            Turn::Left => b'L',
            Turn::Right => b'R',
        }
    }
}

impl Map<'_> {
    fn get_doors(&self, it: impl Iterator<Item = Turn>) -> Doors {
        let it_len = it.try_len().map(|x| x).unwrap_or_default();
        let mut payload = Vec::with_capacity(it_len + self.passcode.len());
        payload.extend_from_slice(self.passcode.as_bytes());
        for x in it {
            payload.push(x.to_letter());
        }
        let hash = md5::compute(payload.as_slice());
        let (up, down, left, right) = hash
            .as_bytes()
            .iter()
            .flat_map(|x| [x >> 4, x & 0b1111])
            .take(4)
            .map(|x| (11..16).contains(&x))
            .collect_tuple()
            .unwrap();

        Doors(0)
            .set_up(up)
            .set_down(down)
            .set_left(left)
            .set_right(right)
    }
}

#[cfg(test)]
mod tests {
    use super::part1;
    use super::{Doors, Map, Turn};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case("hijkl", Vec::new().into_iter(), Doors(0b0111))]
    #[case("hijkl", [Turn::Down].into_iter(), Doors(0b1101))]
    fn test_opened(
        #[case] passcode: &str,
        #[case] path: impl Iterator<Item = Turn>,
        #[case] expected: Doors,
    ) {
        let map = Map { passcode };
        let doors = map.get_doors(path);
        assert_eq!(doors, expected);
    }
    #[rstest]
    #[case::ihgpwlah("ihgpwlah", "DDRRRD")]
    #[case::example("kglvqrro", "DDUDRLRRUDRD")]
    #[case::example("ulqzkmiv", "DRURDRUDDLLDLUURRDULRLDUUDDDRR")]
    #[case::actual(ACTUAL, "RDURRDDLRD")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example("ihgpwlah", "370")]
    #[case::actual(ACTUAL, "526")]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        use crate::part2;
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
