use advent_utils::{glam::IVec2, grid::Grid};

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
        let mut res = 0;
        let mut it = FULL_NEIGHBOURS.iter().map(|d| colors.get(p + *d));
        for _ in 0..FULL_NEIGHBOURS.len() {
            res <<= 1;
            res |= if it.next().flatten().map(|v| *v == color).unwrap_or_default() {
                1
            } else {
                0
            };
        }
        res
    };
    const MINUSES: [usize; 256] = [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 0, 4, 2, 0,
        4, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 2, 0, 0, 2,
        0, 0, 2, 2, 0, 0, 0, 2, 0, 0, 2, 0, 0, 0, 0, 4, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 4, 2, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 2, 0, 4, 4, 2, 0, 2, 2, 0, 0, 0, 0, 0, 0, 2, 0,
        4, 2, 0, 0, 2, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 2, 0, 0, 2, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 2, 2,
        4, 2, 0, 0, 4, 2, 4, 2, 0, 0, 2, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0,
        0, 0, 2, 0, 0, 0, 2, 0, 4, 2, 0, 4, 2, 0, 4, 4,
    ];
    const PLUSES: [usize; 256] = [
        4, 0, 0, 2, 0, 0, 2, 4, 0, 2, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 2, 0, 0, 0, 4, 2, 4, 0, 0, 0, 4, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 2, 4, 4,
        0, 2, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 4, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0,
        4, 0, 0, 2, 0, 2, 0, 0, 2, 4, 0, 0, 2, 0, 0, 0, 2, 0, 0, 2, 0, 2, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 4, 0, 2, 4, 0, 2, 4, 0, 2, 0, 0, 0, 2, 0, 0,
        0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ];

    *sides = *sides + PLUSES[neighbours] - MINUSES[neighbours];

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
