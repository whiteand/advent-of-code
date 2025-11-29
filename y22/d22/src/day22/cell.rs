use std::fmt::Write;

use advent_utils::grid::ToAsciiChar;

#[derive(Copy, Clone)]
pub enum Cell {
    Free,
    Wall,
}
impl ToAsciiChar for Cell {
    fn to_ascii_char(&self) -> u8 {
        match self {
            Cell::Free => b'.',
            Cell::Wall => b'#',
        }
    }
}
impl std::fmt::Debug for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_ascii_char() as char)
    }
}
