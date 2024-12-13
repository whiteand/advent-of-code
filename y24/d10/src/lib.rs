use advent_utils::{glam::IVec2, grid::Grid, parse};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content);
    let mut visited = grid.map(|_, _| false);

    grid.coords()
        .filter(|pos| grid.get(*pos).copied() == Some(b'0'))
        .map(|pos| {
            visited.fill(false);
            load_finish_positions(&grid, pos, &mut visited);
            visited.iter().filter(|x| **x).count()
        })
        .sum()
}

fn load_finish_positions(grid: &Grid<u8>, pos: IVec2, visited: &mut Grid<bool>) {
    let Some(value) = grid.get(pos).copied() else {
        return;
    };
    if value == b'9' {
        visited.set(pos, true);
        return;
    }
    for (p, _) in grid
        .neighbours(pos, [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X])
        .filter(|(_, v)| **v == value + 1)
    {
        load_finish_positions(grid, p, visited);
    }
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content);
    let mut dp = grid.map(|x, _| (*x == b'9').then_some(1));
    grid.coords()
        .filter(|pos| grid.get(*pos).copied() == Some(b'0'))
        .map(|pos| get_rating(&grid, pos, &mut dp))
        .sum()
}
fn get_rating(grid: &Grid<u8>, pos: IVec2, dp: &mut Grid<Option<usize>>) -> usize {
    if let Some(Some(x)) = dp.get(pos) {
        return *x;
    }
    let Some(value) = grid.get(pos).copied() else {
        return 0;
    };

    let res = grid
        .neighbours(pos, [IVec2::NEG_Y, IVec2::Y, IVec2::X, IVec2::NEG_X])
        .filter(|(_, v)| **v == value + 1)
        .map(|(p, v)| {
            if *v == b'9' {
                1
            } else {
                get_rating(grid, p, dp)
            }
        })
        .sum();

    dp.set(pos, Some(res));
    res
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "36");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "776");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "81");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "1657");
    }
}
