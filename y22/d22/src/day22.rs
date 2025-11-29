use advent_utils::{
    glam::IVec2,
    grid::{Grid, NonDiagonal, N8},
    math::PerNonDiagonalDirection,
};
use itertools::Itertools;

use crate::day22::{
    cell::Cell,
    command::Command,
    paths::{find_2d_wrap, precalculate_neighbours},
};

mod cell;
mod command;
mod parse;
mod paths;

type Shape = Grid<Option<Cell>>;

fn find_initial_position(grid: &Shape) -> IVec2 {
    grid.entries()
        .find_map(|(p, c)| matches!(c, Some(Cell::Free)).then_some(p))
        .expect("should be present")
}

fn solve<'t>(
    grid: &'t Shape,
    commands: Vec<Command>,
    find_wrap: impl Fn(&'t Shape, IVec2, IVec2) -> Option<IVec2>,
) -> i32 {
    tracing::info!("Grid:\n```\n{}\n```", grid.render_ascii());
    let initial_position = find_initial_position(&grid);

    let mut dir = IVec2::X;
    let mut pos = initial_position;
    let neighbours = precalculate_neighbours(&grid, find_wrap);

    for c in commands {
        match c {
            Command::Move(s) => {
                for _ in 0..s {
                    let neighbours = neighbours
                        .get(pos)
                        .copied()
                        .unwrap_or_else(|| PerNonDiagonalDirection::splat(&pos));
                    let next_pos = unsafe { *neighbours.unsafe_get(dir) };
                    pos = next_pos
                }
            }
            Command::TurnRight => {
                dir = dir.rotate(IVec2::Y);
            }
            Command::TurnLeft => {
                dir = dir.rotate(IVec2::NEG_Y);
            }
        }
    }

    let final_password = (pos.y + 1) * 1000
        + (pos.x + 1) * 4
        + match NonDiagonal::to_ascii_char(dir) {
            b'>' => 0,
            b'v' => 1,
            b'<' => 2,
            b'^' => 3,
            _ => unreachable!(),
        };

    final_password
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> i32 {
    let (grid, commands) = parse::parse_input(file_content);
    solve(&grid, commands, find_2d_wrap)
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> i32 {
    let (grid, commands) = parse::parse_input(file_content);
    let corners = parse_corners(&grid);
    println!("{:?}", &corners);
    let sides = corners
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(prev, next)| (prev - next).abs().dot(IVec2::splat(1)))
        .collect_vec();
    println!("Sides: {:?}", sides);

    // TODO: Write this function
    let find_3d_wrap = |_, start, _| Some(start);
    solve(&grid, commands, find_3d_wrap)
}

fn parse_corners(grid: &Shape) -> Vec<IVec2> {
    let top_left_corner = find_initial_position(&grid);
    let mut direction = IVec2::X;
    let mut res = Vec::new();

    let mut visited = grid.map(|_, _| false);

    let mut pos = top_left_corner;
    loop {
        if visited.get(pos).copied().unwrap_or_default() {
            break;
        }
        tracing::info!(?pos, "visited");
        visited.set(pos, true);
        res.push(pos);
        let positions = N8::directions()
            .map(|d| d.rotate(direction))
            .map(|d| pos + d);
        let [_, left_pos, _, _, _, right_pos, _, _] = positions;
        let [left_back, left, _, forward, _, right, _, _] = positions.map(|p| {
            if grid.get(p).copied().flatten().is_some() {
                1u8
            } else {
                0u8
            }
        });
        let should_go_forward = forward != 0 && left == 0;
        if should_go_forward {
            pos = pos + direction;
            res.pop();
            continue;
        }
        let should_go_right = forward == 0 && right != 0 && left == 0;
        if should_go_right {
            direction = right_pos - pos;
            pos = right_pos;
            continue;
        }
        let should_go_left = left != 0 && forward != 0 && left_back == 0;
        if should_go_left {
            direction = left_pos - pos;
            pos = left_pos;
            continue;
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case::example(EXAMPLE, "6032")]
    #[case::actual(ACTUAL, "196134")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    // #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
