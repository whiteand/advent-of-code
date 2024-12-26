use core::hash;
use std::fmt::Write;

use advent_utils::{declare_array, declare_field, glam::IVec2, nom::AsBytes};
use itertools::Itertools;
use tracing::{info, span, Level};

#[tracing::instrument(ret)]
pub fn part1(passcode: &str) -> String {
    let map = Map {
        passcode: passcode.trim(),
    };
    for x in pathfinding::prelude::dijkstra_reach(
        &(IVec2::new(0, 0), Path::empty()),
        |(pos, path), _| {
            let pos = *pos;
            let path = *path;
            let doors = map.get_doors(path);
            info!(?pos, %path, %doors);
            doors
                .filter(move |_| path.len() < Path::MAX_LEN)
                .filter_map(move |t| {
                    let _span = span!(Level::INFO, "suc", p = %path, ?pos, %t).entered();
                    let next_pos = pos + t.to_vec();
                    info!(?next_pos);
                    if !(0..=3i32).contains(&next_pos.x) {
                        return None;
                    }
                    if !(0..=3i32).contains(&next_pos.y) {
                        return None;
                    }
                    let next_path = path.push_back(t);
                    info!(%next_path);
                    // tracing::info!(?next_pos, t = ?(t.to_letter() as char), new_path = ?next_path.clone().map(|x| x.to_letter() as char).join(""));
                    Some(((next_pos, next_path), 1))
                })
        },
    ) {
        if x.node.0 == IVec2::new(3, 3) {
            return x.node.1.map(|x| x.to_letter() as char).join("");
        }
    }

    "".to_owned()
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
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

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Path(u128);

impl std::fmt::Display for Path {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in (*self).map(|x| x.to_letter() as char) {
            f.write_char(c)?;
        }
        Ok(())
    }
}

impl Path {
    const MAX_LEN: usize = 127;
    const ITEMS_OFFSET: usize = 7;
    const ITEM_SIZE: usize = 2;
    fn empty() -> Self {
        Self(0)
    }
    declare_field!(u128, usize, len, set_len, 0, 0b1111111);
    declare_array!(
        u128,
        u8,
        get_turn,
        set_turn,
        Self::ITEMS_OFFSET,
        Self::ITEM_SIZE,
        0b11
    );

    fn split_first(&mut self) -> Option<(Turn, Self)> {
        let prev_len = self.len();
        if prev_len > 0 {
            let value = self.get_turn(0);
            let turn = Turn::from_bitmask(value);
            let mut rest = self.0 >> Self::ITEMS_OFFSET + Self::ITEM_SIZE;
            rest <<= Self::ITEMS_OFFSET;
            rest |= (prev_len - 1) as u128;

            Some((turn, Self(rest)))
        } else {
            None
        }
    }
    fn push_back(&self, turn: Turn) -> Self {
        let prev_len = self.len();
        assert!(prev_len < Self::MAX_LEN);
        self.set_turn(prev_len, turn as u8).set_len(prev_len + 1)
    }
}

impl Iterator for Path {
    type Item = Turn;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some((first, rest)) = self.split_first() {
            *self = rest;
            Some(first)
        } else {
            None
        }
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
#[repr(u8)]
enum Turn {
    Up = 0b00,
    Down = 0b01,
    Left = 0b10,
    Right = 0b11,
}

impl std::fmt::Display for Turn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_letter() as char)
    }
}

impl Turn {
    fn from_bitmask(bits: u8) -> Turn {
        match bits {
            0b00 => Turn::Up,
            0b01 => Turn::Down,
            0b10 => Turn::Left,
            0b11 => Turn::Right,
            x => unreachable!("unknown turn {x}"),
        }
    }
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
    fn get_doors(&self, path: Path) -> Doors {
        let mut payload = Vec::with_capacity(path.len() + self.passcode.len());
        payload.extend_from_slice(self.passcode.as_bytes());
        let it = path;
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
    use super::{part1, part2};
    use super::{Doors, Map, Path, Turn};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case("hijkl", Path::empty(), Doors(0b0111))]
    #[case("hijkl", Path::empty().push_back(Turn::Down), Doors(0b1101))]
    fn test_opened(#[case] passcode: &str, #[case] path: Path, #[case] expected: Doors) {
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
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    // #[rstest]
    // #[case::actual(ACTUAL, "0")]
    // fn test_part2(#[case] input: &str, #[case] expected: &str) {
    //     let _guard = tracing::subscriber::set_default(
    //         tracing_subscriber::FmtSubscriber::builder()
    //             .without_time()
    //             .finish(),
    //     );
    //     assert_eq!(format!("{}", part2(input)), expected);
    // }
}
