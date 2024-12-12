use advent_utils::{glam::IVec2, grid::Grid};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve::<AreaAndPerimeter>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve::<AreaAndSides>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve<V: GroupVisitor>(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content.trim());
    let mut groups = grid.map(|_, _| 0);
    let mut group_id = 0;
    let mut total = 0;
    for p in grid.coords() {
        let value = grid.get(p).copied().unwrap();
        if groups.get(p).copied().unwrap_or_default() == 0 {
            group_id += 1;
            let mut res1 = 0;
            let mut res2 = 0;

            V::visit(&grid, &mut groups, p, value, group_id, &mut res1, &mut res2);

            total += res1 * res2;
        }
    }
    total
}

trait GroupVisitor {
    fn visit(
        grid: &Grid<u8>,
        colors: &mut Grid<usize>,
        p: IVec2,
        region_value: u8,
        color: usize,
        res1: &mut usize,
        res2: &mut usize,
    );
}

const DIRS: [IVec2; 4] = [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X];

struct AreaAndPerimeter;

impl GroupVisitor for AreaAndPerimeter {
    fn visit(
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
            AreaAndPerimeter::visit(grid, colors, p, region_value, color, area, perimeter);
        }
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

const SIDES: [isize; 256] = [
    4, 4, 0, 2, 4, 4, 2, 4, 0, 2, -2, -2, 0, 2, 0, 0, 0, 0, -2, 0, 2, 2, -2, 0, -4, -2, -4, -4, -2,
    0, -4, -4, 4, 4, 0, 2, 4, 4, 2, 4, 2, 4, 0, 0, 2, 4, 2, 2, 0, 0, -2, 0, 2, 2, -2, 0, -2, 0, -2,
    -2, 0, 2, -2, -2, 0, 0, -4, -2, 0, 0, -2, 0, -2, 0, -4, -4, -2, 0, -2, -2, -2, -2, -4, -2, 0,
    0, -4, -2, -4, -2, -4, -4, -2, 0, -4, -4, 2, 2, -2, 0, 2, 2, 0, 2, -2, 0, -4, -4, -2, 0, -2,
    -2, 0, 0, -2, 0, 2, 2, -2, 0, -4, -2, -4, -4, -2, 0, -4, -4, 4, 4, 0, 2, 4, 4, 2, 4, 0, 2, -2,
    -2, 0, 2, 0, 0, 2, 2, 0, 2, 4, 4, 0, 2, -2, 0, -2, -2, 0, 2, -2, -2, 4, 4, 0, 2, 4, 4, 2, 4, 2,
    4, 0, 0, 2, 4, 2, 2, 2, 2, 0, 2, 4, 4, 0, 2, 0, 2, 0, 0, 2, 4, 0, 0, 2, 2, -2, 0, 2, 2, 0, 2,
    0, 2, -2, -2, 0, 2, 0, 0, -2, -2, -4, -2, 0, 0, -4, -2, -4, -2, -4, -4, -2, 0, -4, -4, 4, 4, 0,
    2, 4, 4, 2, 4, 0, 2, -2, -2, 0, 2, 0, 0, 0, 0, -2, 0, 2, 2, -2, 0, -4, -2, -4, -4, -2, 0, -4,
    -4,
];
struct AreaAndSides;

impl GroupVisitor for AreaAndSides {
    fn visit(
        grid: &Grid<u8>,
        groups: &mut Grid<usize>,
        p: IVec2,
        region_value: u8,
        group_id: usize,
        area: &mut usize,
        sides: &mut usize,
    ) {
        if groups.get(p).copied().unwrap() == group_id {
            return;
        }
        *area += 1;
        groups.set(p, group_id);

        let neighbours_bitmask = FULL_NEIGHBOURS
            .iter()
            .map(|d| {
                groups
                    .get(p + *d)
                    .and_then(|x| (*x == group_id).then_some(1))
                    .unwrap_or_default()
            })
            .fold(0u8, |x, b| (x << 1) | b);

        *sides = ((*sides) as isize + SIDES[neighbours_bitmask as usize]) as usize;

        for p in grid
            .neighbours(p, DIRS)
            .filter(|(_, v)| **v == region_value)
            .map(|(p, _)| p)
        {
            AreaAndSides::visit(grid, groups, p, region_value, group_id, area, sides);
        }
    }
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
