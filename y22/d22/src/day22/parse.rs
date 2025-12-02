use advent_utils::{
    glam::IVec2,
    grid::Grid,
    nom::{self, Parser},
};

use super::cell::Cell;
use super::command::Command;

pub fn parse_input(input: &str) -> (Grid<Option<Cell>>, Vec<Command>) {
    let mut byte_grid = Grid::new(IVec2::new(0, 0), 0u8);
    let mut lines = input.lines();
    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }
        let new_size = (byte_grid.size() + IVec2::Y).max(IVec2::new(line.len() as i32, 0));

        byte_grid.resize(new_size, b' ');
        let last_row = &mut byte_grid.row_mut(byte_grid.rows_len() - 1).unwrap()[..line.len()];
        last_row.copy_from_slice(line.as_bytes());
    }

    let commands_line = lines
        .find(|line| !line.is_empty())
        .expect("There should be a non-empty line with commands");

    let grid = byte_grid.map(|b, _| match b {
        b' ' => None,
        b'.' => Some(Cell::Free),
        b'#' => Some(Cell::Wall),
        x => unreachable!("Unexpected cell: {x}"),
    });

    (
        grid,
        nom::multi::many1(Command::parse)
            .parse(commands_line)
            .map(|(_, v)| v)
            .unwrap(),
    )
}
