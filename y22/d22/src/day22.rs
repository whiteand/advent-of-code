use advent_utils::{
    glam::IVec2,
    grid::{Grid, N8, NonDiagonal},
    math::{LinearProgression, get_gcd},
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
    find_wrap: impl FnMut(&'t Shape, IVec2, IVec2) -> Option<(IVec2, IVec2)>,
) -> i32 {
    // tracing::info!("Grid:\n```\n{}\n```", grid.render_ascii());
    let initial_position = find_initial_position(grid);

    let mut dir = IVec2::X;
    let mut pos = initial_position;
    let neighbours = precalculate_neighbours(grid, find_wrap);

    for c in commands {
        match c {
            Command::Move(s) => {
                for _ in 0..s {
                    let neighbours = neighbours.get(pos).copied().unwrap();
                    let (next_pos, next_direction) = unsafe { *neighbours.unsafe_get(dir) };
                    pos = next_pos;
                    dir = next_direction;
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

    (pos.y + 1) * 1000
        + (pos.x + 1) * 4
        + match NonDiagonal::to_ascii_char(dir) {
            b'>' => 0,
            b'v' => 1,
            b'<' => 2,
            b'^' => 3,
            _ => unreachable!(),
        }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> i32 {
    let (grid, commands) = parse::parse_input(file_content);
    solve(&grid, commands, find_2d_wrap)
}

// Have not solved it in general sense. Since I failed to fastly recognize the exact connections
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> i32 {
    let (grid, commands) = parse::parse_input(file_content);
    let corners = parse_corners(&grid);
    let sides = get_sides(&grid, &corners);
    let cube_size = sides
        .iter()
        .fold(sides[0], |a, b| get_gcd(a as u128, *b as u128) as usize);

    let mut combined = grid.map(|x, _| if x.is_some() { b'.' } else { b' ' });

    let rules = get_edge_connections(cube_size as i32);

    let mut i = 0;

    for (in_dir, in_pos, out_dir, out_pos) in rules.iter().map(|(a, b, c, d)| (*a, b, *c, d)) {
        for (f, t) in in_pos.iter().copied().zip(out_pos.iter().copied()) {
            let letter = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes()[i % 52];
            i += 1;
            combined.set(f, letter);
            combined.set(f - in_dir, NonDiagonal::to_ascii_char(in_dir));
            combined.set(t, letter);
            combined.set(t + out_dir, NonDiagonal::to_ascii_char(out_dir));
        }
    }

    println!("Connections:\n```\n{}\n```\n", combined.render_ascii());

    // TODO: Write this function
    let res = solve(&grid, commands, |grid, start, d| {
        for (in_dir, in_pos, out_dir, out_pos) in rules.iter().map(|(a, b, c, d)| (*a, b, *c, d)) {
            if d == in_dir
                && let Some(i) = in_pos.iter().position(|p| *p == start)
            {
                let res = out_pos[i];
                if grid
                    .get(res)
                    .map(|x| matches!(x, Some(Cell::Wall)))
                    .unwrap_or_default()
                {
                    return None;
                }
                return Some((res, out_dir));
            }
            if d == IVec2::new(-out_dir.x, -out_dir.y)
                && let Some(i) = out_pos.iter().position(|p| *p == start)
            {
                let res = in_pos[i];
                if grid
                    .get(res)
                    .map(|x| matches!(x, Some(Cell::Wall)))
                    .unwrap_or_default()
                {
                    return None;
                }
                return Some((res, IVec2::new(-in_dir.x, -in_dir.y)));
            }
        }

        unreachable!("start={start}, d={d}")
    });
    println!("{}", combined.render_ascii());
    res
}

fn get_edge_connections(s: i32) -> Vec<(IVec2, Vec<IVec2>, IVec2, Vec<IVec2>)> {
    let g = |x, y, d| LinearProgression::new(IVec2::new(x, y), d, 50).collect_vec();
    vec![
        (
            IVec2::NEG_Y,
            g(s, 0, IVec2::X),
            IVec2::X,
            g(0, 3 * s, IVec2::Y),
        ),
        (
            IVec2::NEG_X,
            g(s, 0, IVec2::Y),
            IVec2::X,
            g(0, 3 * s - 1, IVec2::NEG_Y),
        ),
        (
            IVec2::NEG_Y,
            g(2 * s, 0, IVec2::X),
            IVec2::NEG_Y,
            g(0, 4 * s - 1, IVec2::X),
        ),
        (
            IVec2::X,
            g(3 * s - 1, 0, IVec2::Y),
            IVec2::NEG_X,
            g(2 * s - 1, 3 * s - 1, IVec2::NEG_Y),
        ),
        (
            IVec2::Y,
            g(2 * s, s - 1, IVec2::X),
            IVec2::NEG_X,
            g(2 * s - 1, s, IVec2::Y),
        ),
        (
            IVec2::NEG_X,
            g(s, s, IVec2::Y),
            IVec2::Y,
            g(0, 2 * s, IVec2::X),
        ),
        (
            IVec2::Y,
            g(s, 3 * s - 1, IVec2::X),
            IVec2::NEG_X,
            g(s - 1, 3 * s, IVec2::Y),
        ),
    ]
}

fn get_sides(grid: &Shape, corners: &[IVec2]) -> Vec<usize> {
    corners
        .iter()
        .copied()
        .circular_tuple_windows()
        .map(|(p, n)| {
            ((p - n).abs().dot(IVec2::splat(1)) + 1
                - ([p, n]
                    .into_iter()
                    .filter(|p| is_inner_angle(grid, *p))
                    .count() as i32)) as usize
        })
        .collect_vec()
}

fn is_inner_angle(grid: &Shape, p: IVec2) -> bool {
    NonDiagonal::directions()
        .map(|d| d + p)
        .into_iter()
        .all(|p| grid.get(p).copied().flatten().is_some())
}

fn parse_corners(grid: &Shape) -> Vec<IVec2> {
    let top_left_corner = find_initial_position(grid);
    let mut direction = IVec2::X;
    let mut res = Vec::new();

    let mut visited = grid.map(|_, _| false);

    let mut pos = top_left_corner;
    let mut has_intermediate_points = false;
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
            pos += direction;
            if has_intermediate_points {
                res.pop();
            } else {
                has_intermediate_points = true;
            }
            continue;
        }
        let should_go_right = forward == 0 && right != 0 && left == 0;
        if should_go_right {
            direction = right_pos - pos;
            pos = right_pos;
            has_intermediate_points = true;
            continue;
        }
        let should_go_left = left != 0 && forward != 0 && left_back == 0;
        if should_go_left {
            direction = left_pos - pos;
            pos = left_pos;
            has_intermediate_points = true;
            continue;
        }
        unreachable!("Here")
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
    // #[case::example(EXAMPLE, "0")] // have not solved in general sense
    #[case::actual(ACTUAL, "146011")]
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
