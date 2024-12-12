use advent_utils::{glam::IVec2, grid::Grid};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());
    let mut colors = grid.map(|_, _| 0);
    let mut color = 0;
    let mut total = 0;
    for p in grid.coords() {
        let value = grid.get(p).copied().unwrap();
        if colors.get(p).copied().unwrap_or_default() == 0 {
            color += 1;
            let mut area = 0;
            let mut perimeter = 0;
            calculate_area_and_perimeter(
                &grid,
                &mut colors,
                p,
                value,
                color,
                &mut area,
                &mut perimeter,
            );
            total += area * perimeter;
        }
    }
    total
}

const DIRS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];
fn calculate_area_and_sides(
    grid: &Grid<u8>,
    colors: &mut Grid<usize>,
    p: IVec2,
    region_value: u8,
    color: usize,
    area: &mut usize,
    sides: &mut usize,
) {
    if colors.get(p).copied().unwrap() == color {
        return;
    }
    *area += 1;
    colors.set(p, color);
    let neighbours = {
        let mut res = [false; 8];
        let mut it = FULL_NEIGHBOURS.iter().map(|d| colors.get(p + *d));
        for x in &mut res {
            *x = it.next().flatten().map(|v| *v == color).unwrap_or_default();
        }
        res
    };

    match &neighbours {
        [false, false, false, false, false, false, false, false] => {
            *sides += 4;
        }
        [false, false, false, true, false, false, false, false]
        | [false, true, false, false, false, false, false, false]
        | [false, false, false, false, true, false, false, false]
        | [false, false, false, false, false, false, true, false] => {
            // ...
            // #?.
            // ...
            *sides += 0;
        }
        [true, true, false, false, false, false, false, false]
        | [false, false, true, false, true, false, false, false]
        | [false, false, false, false, false, false, true, true]
        | [false, false, false, true, false, true, false, false]
        | [true, false, false, true, false, false, false, false]
        | [false, false, false, false, false, true, true, false]
        | [false, false, false, false, true, false, false, true]
        | [false, true, true, false, false, false, false, false] => {
            // ##.
            // .?.
            // ...
            *sides += 2;
        }
        [false, false, true, false, true, false, true, true]
        | [false, false, false, true, false, true, true, true]
        | [true, true, false, true, false, true, false, false]
        | [true, true, true, false, true, false, false, false]
        | [false, false, false, false, true, true, true, true]
        | [false, true, true, false, true, false, false, true]
        | [true, true, true, true, false, false, false, false]
        | [true, false, false, true, false, true, true, false] => {
            // ..#
            // .?#
            // .##
            *sides += 0;
        }
        [false, true, true, false, true, false, true, true]
        | [false, false, false, true, true, true, true, true]
        | [true, true, false, true, false, true, true, false]
        | [true, true, true, true, true, false, false, false] => {
            // .##
            // .?#
            // .##
            *sides -= 4;
        }
        [false, false, true, false, true, false, false, true]
        | [false, false, false, false, false, true, true, true]
        | [true, false, false, true, false, true, false, false]
        | [true, true, true, false, false, false, false, false] => {
            // ..#
            // .?#
            // ..#
            *sides += 4;
        }
        [true, false, true, false, true, false, false, true]
        | [false, false, true, false, false, true, true, true]
        | [true, false, false, true, false, true, false, true]
        | [true, true, true, false, false, true, false, false]
        | [true, false, false, false, false, true, true, true]
        | [false, false, true, false, true, true, false, true]
        | [true, true, true, false, false, false, false, true]
        | [true, false, true, true, false, true, false, false] => {
            // #.#
            // .?#
            // ..#
            *sides += 4;
        }
        [true, true, true, true, true, false, true, true]
        | [false, true, true, true, true, true, true, true]
        | [true, true, false, true, true, true, true, true]
        | [true, true, true, true, true, true, true, false] => {
            // ###
            // #?#
            // .##
            *sides -= 4;
        }
        [false, true, true, false, true, false, false, false]
        | [false, false, false, false, true, false, true, true]
        | [false, false, false, true, false, true, true, false]
        | [true, true, false, true, false, false, false, false] => {
            // .##
            // .?#
            // ...
            *sides -= 2;
        }
        [true, true, true, true, true, false, false, true]
        | [false, true, true, false, true, true, true, true]
        | [true, false, false, true, true, true, true, true]
        | [true, true, true, true, false, true, true, false]
        | [true, true, false, true, false, true, true, true]
        | [false, false, true, true, true, true, true, true]
        | [true, true, true, false, true, false, true, true]
        | [true, true, true, true, true, true, false, false] => {
            // ###
            // #?#
            // ..#
            *sides -= 2;
        }
        [true, true, true, false, true, false, false, true]
        | [false, false, true, false, true, true, true, true]
        | [true, false, false, true, false, true, true, true]
        | [true, true, true, true, false, true, false, false] => {
            // ###
            // .?#
            // ..#
            *sides += 2;
        }
        [true, false, false, false, false, true, true, false]
        | [true, false, true, true, false, false, false, false]
        | [false, true, true, false, false, false, false, true]
        | [false, false, false, false, true, true, false, true]
        | [true, false, true, false, true, false, false, false]
        | [true, true, false, false, false, true, false, false]
        | [false, false, false, true, false, true, false, true]
        | [false, false, true, false, false, false, true, true] => {
            // #..
            // .?.
            // ##.
            *sides += 2;
        }
        [false, false, true, false, false, false, true, false]
        | [false, false, false, true, false, false, false, true]
        | [false, true, false, false, false, true, false, false]
        | [true, false, false, false, true, false, false, false]
        | [false, false, false, false, true, true, false, false]
        | [false, true, false, false, false, false, false, true]
        | [false, false, true, true, false, false, false, false]
        | [true, false, false, false, false, false, true, false] => {
            // ..#
            // .?.
            // .#.
            *sides += 0;
        }
        [false, false, false, false, true, true, true, false]
        | [false, false, false, true, false, false, true, true]
        | [false, false, true, false, true, false, true, false]
        | [false, true, false, false, true, false, false, true]
        | [false, true, false, true, false, true, false, false]
        | [false, true, true, true, false, false, false, false]
        | [true, false, false, true, false, false, true, false]
        | [true, true, false, false, true, false, false, false] => {
            // ..#
            // .?#
            // .#.
            *sides += 0;
        }
        [true, false, true, false, true, true, true, true]
        | [true, false, true, true, false, true, true, true]
        | [true, true, true, false, true, true, false, true]
        | [true, true, true, true, false, true, false, true] => {
            // ###
            // .?#
            // #.#
            *sides += 2;
        }
        [false, false, true, true, false, true, true, true]
        | [false, true, true, false, true, true, false, true]
        | [true, false, false, false, true, true, true, true]
        | [true, false, true, false, true, false, true, true]
        | [true, false, true, true, false, true, true, false]
        | [true, true, false, true, false, true, false, true]
        | [true, true, true, false, true, true, false, false]
        | [true, true, true, true, false, false, false, true] => {
            // #.#
            // .?#
            // .##
            *sides += 0;
        }
        [false, true, true, true, false, true, true, true]
        | [false, true, true, true, true, true, false, true]
        | [true, false, true, true, true, false, true, true]
        | [true, false, true, true, true, true, true, false]
        | [true, true, false, false, true, true, true, true]
        | [true, true, false, true, true, true, false, true]
        | [true, true, true, false, true, true, true, false]
        | [true, true, true, true, false, false, true, true] => {
            // #.#
            // #?#
            // .##
            *sides += 0;
        }
        [true, true, true, true, true, true, true, true] => {
            // ###
            // #?#
            // ###
            *sides -= 4;
        }
        [true, false, true, true, true, true, true, true]
        | [true, true, true, false, true, true, true, true]
        | [true, true, true, true, false, true, true, true]
        | [true, true, true, true, true, true, false, true] => {
            // ###
            // #?#
            // #.#
            *sides += 0;
        }
        [false, false, false, true, true, false, true, true]
        | [false, false, false, true, true, true, true, false]
        | [false, true, false, false, true, false, true, true]
        | [false, true, false, true, false, true, true, false]
        | [false, true, true, false, true, false, true, false]
        | [false, true, true, true, true, false, false, false]
        | [true, true, false, true, false, false, true, false]
        | [true, true, false, true, true, false, false, false] => {
            // .##
            // .?#
            // .#.
            *sides -= 4;
        }
        [false, false, true, true, true, true, false, true]
        | [false, true, true, false, false, true, true, true]
        | [true, false, false, true, true, true, false, true]
        | [true, false, true, true, true, false, false, true]
        | [true, false, true, true, true, true, false, false]
        | [true, true, false, false, false, true, true, true]
        | [true, true, true, false, false, false, true, true]
        | [true, true, true, false, false, true, true, false] => {
            // #.#
            // #?#
            // #..
            *sides += 2;
        }
        [false, false, true, true, false, true, true, false]
        | [false, true, true, false, true, true, false, false]
        | [true, false, false, false, true, false, true, true]
        | [true, true, false, true, false, false, false, true] => {
            // #..
            // .?#
            // .##
            *sides -= 2;
        }
        [false, false, false, true, true, true, false, true]
        | [false, true, true, false, false, false, true, true]
        | [true, false, true, true, true, false, false, false]
        | [true, true, false, false, false, true, true, false] => {
            // #.#
            // #?#
            // ...
            *sides += 0;
        }
        [true, false, true, true, true, true, false, true]
        | [true, true, true, false, false, true, true, true] => {
            // #.#
            // #?#
            // #.#
            *sides += 4;
        }
        [false, false, false, true, true, false, false, true]
        | [false, false, false, true, true, true, false, false]
        | [false, false, true, true, true, false, false, false]
        | [false, true, false, false, false, false, true, true]
        | [false, true, false, false, false, true, true, false]
        | [false, true, true, false, false, false, true, false]
        | [true, false, false, true, true, false, false, false]
        | [true, true, false, false, false, false, true, false] => {
            // .##
            // .?.
            // .#.
            *sides -= 2;
        }
        [false, false, true, true, true, true, true, false]
        | [false, true, true, false, true, true, true, false]
        | [false, true, true, true, false, true, true, false]
        | [false, true, true, true, true, true, false, false]
        | [true, false, false, true, true, false, true, true]
        | [true, true, false, false, true, false, true, true]
        | [true, true, false, true, false, false, true, true]
        | [true, true, false, true, true, false, false, true] => {
            // ##.
            // #?#
            // ..#
            *sides -= 2;
        }
        [false, false, true, false, true, true, true, false]
        | [false, true, true, true, false, true, false, false]
        | [true, false, false, true, false, false, true, true]
        | [true, true, false, false, true, false, false, true] => {
            // ##.
            // .?#
            // ..#
            *sides += 2;
        }
        [false, false, true, true, true, false, true, true]
        | [false, true, false, false, true, true, true, true]
        | [false, true, false, true, false, true, true, true]
        | [false, true, true, true, true, false, false, true]
        | [true, false, false, true, true, true, true, false]
        | [true, true, false, true, true, true, false, false]
        | [true, true, true, false, true, false, true, false]
        | [true, true, true, true, false, false, true, false] => {
            // .##
            // #?#
            // ..#
            *sides -= 2;
        }
        [false, false, true, true, true, false, false, true]
        | [false, true, false, false, false, true, true, true]
        | [true, false, false, true, true, true, false, false]
        | [true, true, true, false, false, false, true, false] => {
            // ###
            // .?.
            // .#.
            *sides += 0;
        }
        [false, false, true, true, true, true, false, false]
        | [false, true, true, false, false, true, true, false]
        | [true, false, false, true, true, false, false, true]
        | [true, true, false, false, false, false, true, true] => {
            // ##.
            // .?.
            // .##
            *sides += 0;
        }
        c => {
            let mut it = c.iter().copied().map(|x| if x { '#' } else { '.' });
            macro_rules! rot90 {
                ($v:expr) => {
                    [$v[5], $v[3], $v[0], $v[6], $v[1], $v[7], $v[4], $v[2]]
                };
            }
            // 012
            // 3 4
            // 567
            // 035
            // 1 6
            // 247
            macro_rules! flip {
                ($v:expr) => {
                    [$v[0], $v[3], $v[5], $v[1], $v[6], $v[2], $v[4], $v[7]]
                };
            }
            let rot90 = rot90!(c);
            let rot180 = rot90!(rot90);
            let rot270 = rot90!(rot180);
            let flip_rot90 = flip!(rot90);
            let flip_rot180 = flip!(rot180);
            let flip_rot270 = flip!(rot270);
            let f = flip!(c);
            let combos = [
                *c,
                rot90,
                rot180,
                rot270,
                f,
                flip_rot90,
                flip_rot180,
                flip_rot270,
            ]
            .into_iter()
            .unique()
            .sorted()
            .map(|x| format!("{x:?}"))
            .join(" | ");
            panic!(
                "{combos} => {{\n  // {}{}{}\n  // {}?{}\n  // {}{}{}\n  *sides += 0;\n}}",
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            );
        }
    }

    for p in grid
        .neighbours(p, DIRS)
        .filter(|(_, v)| **v == region_value)
        .map(|(p, _)| p)
    {
        calculate_area_and_sides(grid, colors, p, region_value, color, area, sides);
    }
}
const FULL_NEIGHBOURS: [IVec2; 8] = [
    IVec2::splat(-1),
    IVec2::NEG_Y,
    IVec2::new(1, -1),
    IVec2::NEG_X,
    IVec2::X,
    IVec2::new(-1, 1),
    IVec2::Y,
    IVec2::splat(1),
];
fn calculate_area_and_perimeter(
    grid: &Grid<u8>,
    colors: &mut Grid<usize>,
    p: IVec2,
    region_value: u8,
    color: usize,
    area: &mut usize,
    perimeter: &mut usize,
) {
    if colors.get(p).copied().unwrap() == color {
        return;
    }
    *area += 1;
    colors.set(p, color);
    *perimeter = *perimeter + 4
        - colors
            .neighbours(p, DIRS)
            .filter(|(_, v)| **v == color)
            .count()
            * 2;

    for p in grid
        .neighbours(p, DIRS)
        .filter(|(_, v)| **v == region_value)
        .map(|(p, _)| p)
    {
        calculate_area_and_perimeter(grid, colors, p, region_value, color, area, perimeter);
    }
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());
    let mut colors = grid.map(|_, _| 0);
    let mut color = 0;
    let mut total = 0;
    for p in grid.coords() {
        let value = grid.get(p).copied().unwrap();
        if colors.get(p).copied().unwrap_or_default() == 0 {
            color += 1;
            let mut area = 0;
            let mut sides = 0;
            calculate_area_and_sides(&grid, &mut colors, p, value, color, &mut area, &mut sides);
            total += area * sides;
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "1930");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1522850");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "1206");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "953738");
    }
}
