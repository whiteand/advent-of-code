use advent_utils::{
    glam::I64Vec2,
    math::{Turn, get_turn},
    parse,
};
use itertools::Itertools;

#[tracing::instrument(skip(input))]
pub fn part1(input: &str) -> usize {
    parse_points(input)
        .tuple_combinations()
        .map(|(a, b)| rect_area(a, b))
        .max()
        .unwrap_or_default()
}
pub fn parse_points(input: &str) -> impl Iterator<Item = I64Vec2> + Clone {
    input.lines().filter_map(|line| {
        parse::nums::<i64>(line)
            .collect_tuple()
            .map(|(a, b)| I64Vec2::new(a, b))
    })
}
pub fn rect_area(a: I64Vec2, b: I64Vec2) -> usize {
    let w = a.x.abs_diff(b.x) as usize + 1;
    let h = a.y.abs_diff(b.y) as usize + 1;
    w * h
}

#[tracing::instrument(skip(input))]
pub fn part2(input: &str) -> usize {
    let vertices = parse_points(input).collect::<Vec<_>>();

    let (xs, ys, rects) = split(vertices.iter().copied());

    vertices
        .iter()
        .copied()
        .tuple_combinations()
        .filter(|(corner1, corner2)| {
            cut_rect_with(&xs, &ys, corner1.min(*corner2), corner1.max(*corner2))
                .all(|r| contains_fully(&rects, &r))
        })
        .map(|(a, b)| {
            let w = a.x.abs_diff(b.x) as usize + 1;
            let h = a.y.abs_diff(b.y) as usize + 1;
            w * h
        })
        .max()
        .unwrap_or_default()
}

fn contains_fully(parent: &[(I64Vec2, I64Vec2)], child: &(I64Vec2, I64Vec2)) -> bool {
    let min_index_of_parent_with_sufficiently_large_max_y =
        parent.partition_point(|p| p.1.y <= child.0.y);
    if min_index_of_parent_with_sufficiently_large_max_y >= parent.len() {
        return false;
    }
    let parent = &parent[min_index_of_parent_with_sufficiently_large_max_y..];

    parent.iter().any(|parent| {
        parent.0.x <= child.0.x
            && parent.0.y <= child.0.y
            && parent.1.x >= child.1.x
            && parent.1.y >= child.1.y
    })
}

fn cut_rect_with(
    xs: &[i64],
    ys: &[i64],
    a: I64Vec2,
    c: I64Vec2,
) -> impl Iterator<Item = (I64Vec2, I64Vec2)> {
    let ax = a.x;
    let ay = a.y;
    let c = c + I64Vec2::splat(1);
    let cx = c.x;
    let cy = c.y;
    std::iter::once(i64::MIN)
        .chain(ys.iter().copied())
        .chain(std::iter::once(i64::MAX))
        .tuple_windows()
        .skip_while(move |(_, y_max)| {
            // tracing::info!(?y_max, ?ay);
            *y_max < ay
        })
        .take_while(move |(y_min, _)| {
            // tracing::info!(?y_min, ?ay);
            *y_min < cy
        })
        .map(move |(y_min, y_max)| (y_min.max(ay), y_max.min(cy)))
        .filter(|(a, b)| a != b)
        .flat_map(move |(y_min, y_max)| {
            std::iter::once(i64::MIN)
                .chain(xs.iter().copied())
                .chain(std::iter::once(i64::MAX))
                .tuple_windows()
                .skip_while(move |(_, x_max)| *x_max < ax)
                .take_while(move |(x_min, _)| *x_min < cx)
                .map(move |(x_min, x_max)| (x_min.max(ax), x_max.min(cx)))
                .filter(|(a, b)| a != b)
                .map(move |(x_min, x_max)| (I64Vec2::new(x_min, y_min), I64Vec2::new(x_max, y_max)))
        })
}

fn get_nearest_next_left_corner(next_cell: I64Vec2, current: I64Vec2) -> I64Vec2 {
    let dir = (next_cell - current).signum();
    let dx = [I64Vec2::Y, I64Vec2::NEG_X].contains(&dir) as i64;
    let dy = [I64Vec2::NEG_Y, I64Vec2::NEG_X].contains(&dir) as i64;

    next_cell + I64Vec2::new(dx, dy)
}

fn get_outside_contour_of_cell_path(cells: &[I64Vec2]) -> Vec<I64Vec2> {
    let mut current = cells[0];
    let mut direction = (cells[1] - cells[0]).signum();
    let mut outside_contour = vec![cells[0]];

    // Invariant my back left point is added
    for next_cell in cells.iter().cycle().skip(1).take(cells.len()).copied() {
        if next_cell == current {
            continue;
        }
        let new_dir = (next_cell - current).signum();
        match get_turn(
            advent_utils::math::TurnOrientation::RightIsFromOxToOy,
            direction,
            new_dir,
        ) {
            Turn::None => {
                outside_contour.push(get_nearest_next_left_corner(next_cell, current));
            }
            Turn::Right => {
                outside_contour.push(get_nearest_next_left_corner(current + direction, current));
                outside_contour.push(get_nearest_next_left_corner(next_cell, current));
            }
            Turn::Left => {
                outside_contour.push(get_nearest_next_left_corner(next_cell, current));
            }
        }
        direction = new_dir;
        current = next_cell;
    }
    outside_contour.push(cells[0]);

    simplify_path(&mut outside_contour);

    outside_contour
}

fn simplify_path(path: &mut Vec<I64Vec2>) {
    if path.len() < 3 {
        return;
    }

    let mut src = 1;
    let mut dst = 1;

    while src < path.len() {
        let a = path[dst - 1];
        let b = path[src];
        if let Some(c) = path.get(src + 1)
            && (b == a || (b - a).signum() == (c - b).signum())
        {
            src += 1;
            continue;
        }
        path[dst] = path[src];
        src += 1;
        dst += 1;
    }
    path.truncate(dst);
}

fn split_into_rects(path: &[I64Vec2]) -> (Vec<i64>, Vec<i64>, Vec<(I64Vec2, I64Vec2)>) {
    let xs = path
        .iter()
        .map(|p| p.x)
        .sorted_unstable()
        .dedup()
        .collect_vec();

    let ys = path
        .iter()
        .map(|p| p.y)
        .sorted_unstable()
        .dedup()
        .collect_vec();

    if xs.len() <= 1 || ys.len() <= 1 {
        return (xs, ys, Vec::new());
    }

    let mut rects: Vec<(I64Vec2, I64Vec2)> = Vec::new();

    let rect_rows = ys.len() - 1;

    let (cell_min_y, cell_max_y) = path.iter().map(|c| c.y).minmax().into_option().unwrap();

    for row in 0..rect_rows {
        let y_min = ys[row];
        let y_max = ys[row + 1];
        if y_max < cell_min_y {
            continue;
        }
        if y_min > cell_max_y {
            break;
        }
        let y = y_max + y_min;

        let mut is_inside = false;

        for (i, &intersecting_x) in xs.iter().enumerate().take(xs.len() - 1) {
            if !path
                .iter()
                .circular_tuple_windows()
                .filter(|(a, b)| {
                    if a.x != intersecting_x || b.x != intersecting_x {
                        return false;
                    }
                    let yr = (a.y.min(b.y) * 2)..=(a.y.max(b.y) * 2);
                    yr.contains(&y)
                })
                .count()
                .is_multiple_of(2)
            {
                is_inside = !is_inside;
            }

            if !is_inside {
                continue;
            }

            let x_min = xs[i];
            let x_max = xs[i + 1];
            let a = I64Vec2::new(x_min, y_min);
            let c = I64Vec2::new(x_max, y_max);
            // Merging rect with previous one
            if let Some((a0, c0)) = rects.last().copied()
                && a.y == a0.y
                && c.y == c0.y
                && a.x == c0.x
            {
                rects.pop();
                rects.push((a0, c));
            } else {
                rects.push((a, c));
            }
        }
    }

    (xs, ys, rects)
}

fn split(cells: impl Iterator<Item = I64Vec2>) -> (Vec<i64>, Vec<i64>, Vec<(I64Vec2, I64Vec2)>) {
    let cells = cells.collect_vec();
    let outside_path = get_outside_contour_of_cell_path(&cells);
    let (xs, ys, rects) = split_into_rects(&outside_path);

    (xs, ys, rects)
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "50")]
    #[case::actual(ACTUAL, "4754955192")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "24")]
    // #[case::actual(ACTUAL, "1568849600")] // 22s
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
