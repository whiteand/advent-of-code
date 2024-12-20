use advent_utils::{
    glam::IVec2,
    grid::{Grid, NonDiagonal},
    parse,
};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve::<Perimeter>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve::<Sides>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve<V: GroupVisitor>(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content);
    let mut groups = grid.map(|_, _| 0);
    let mut group_id = 0;
    let mut total = 0;
    let mut positions = Vec::with_capacity(grid.rows_len() * grid.cols(0));

    for p in grid.coords() {
        let value = grid.get(p).copied().unwrap();
        if groups.get(p).copied().unwrap_or_default() == 0 {
            group_id += 1;
            let mut area = 0;
            let mut res2 = 0;

            positions.push(p);
            while let Some(p) = positions.pop() {
                if groups.get(p).copied().unwrap() == group_id {
                    continue;
                }
                groups.set(p, group_id);

                area += 1;
                V::visit(&groups, p, group_id, &mut res2);

                for p in grid
                    .neighbours(p, NonDiagonal)
                    .filter(|(_, v)| **v == value)
                    .map(|(p, _)| p)
                {
                    positions.push(p);
                }
            }

            total += area * res2;
        }
    }
    total
}

trait GroupVisitor {
    fn visit(colors: &Grid<usize>, p: IVec2, color: usize, res2: &mut usize);
}

struct Perimeter;

impl GroupVisitor for Perimeter {
    fn visit(colors: &Grid<usize>, p: IVec2, color: usize, perimeter: &mut usize) {
        *perimeter = *perimeter + 4
            - colors
                .neighbours(p, NonDiagonal)
                .filter(|(_, v)| **v == color)
                .count()
                * 2;
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
struct Sides;

impl GroupVisitor for Sides {
    fn visit(groups: &Grid<usize>, p: IVec2, group_id: usize, sides: &mut usize) {
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
