use advent_utils::{
    glam::IVec2,
    grid::Grid,
    math::{LinearProgression, PerNonDiagonalDirection},
};

use super::cell::Cell;

pub fn precalculate_neighbours<'t>(
    grid: &'t Grid<Option<Cell>>,
    find_wrap: impl Fn(&'t Grid<Option<Cell>>, IVec2, IVec2) -> Option<IVec2>,
) -> Grid<PerNonDiagonalDirection<IVec2>> {
    grid.map(|cell, pos| match cell {
        Some(Cell::Free) => {
            PerNonDiagonalDirection::from_fn(|direction| match grid.get(pos + direction) {
                Some(Some(Cell::Wall)) => pos,
                Some(Some(Cell::Free)) => pos + direction,
                Some(None) | None => find_wrap(grid, pos, direction).unwrap_or(pos),
            })
        }
        _ => PerNonDiagonalDirection::splat(&pos),
    })
}

pub fn find_2d_wrap(grid: &Grid<Option<Cell>>, pos: IVec2, direction: IVec2) -> Option<IVec2> {
    let opposite_pos = match direction.to_array() {
        [0, 1] => IVec2::new(pos.x, 0),
        [0, -1] => IVec2::new(pos.x, (grid.rows_len() - 1) as i32),
        [1, 0] => IVec2::new(0, pos.y),
        [-1, 0] => IVec2::new(grid.cols(pos.y as usize) as i32, pos.y),
        _ => return None,
    };
    let filled_pos = LinearProgression::new_unbound(opposite_pos, direction)
        .find(|p| grid.get(*p).copied().flatten().is_some())?;

    let filled_cell = grid.get(filled_pos).unwrap().unwrap();

    match filled_cell {
        Cell::Wall => Some(pos),
        Cell::Free => Some(filled_pos),
    }
}
