use advent_utils::{glam::IVec2, grid::Grid};

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content);
    grid.coords()
        .filter(|pos| grid.get(*pos).copied() == Some(b'0'))
        .map(|pos| {
            let mut visited = grid.map(|_, _, _| false);
            load_finish_positions(&grid, pos, &mut visited);
            visited.into_iter().filter(|x| *x).count()
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
    let top = pos + IVec2::NEG_Y;
    if grid.get(top).copied() == Some(value + 1) {
        load_finish_positions(grid, top, visited);
    }
    let bottom = pos + IVec2::Y;
    if grid.get(bottom).copied() == Some(value + 1) {
        load_finish_positions(grid, bottom, visited);
    }
    let left = pos + IVec2::NEG_X;
    if grid.get(left).copied() == Some(value + 1) {
        load_finish_positions(grid, left, visited);
    }
    let right = pos + IVec2::X;
    if grid.get(right).copied() == Some(value + 1) {
        load_finish_positions(grid, right, visited);
    }
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let grid = Grid::from_ascii_grid(file_content);
    let mut dp = grid.map(|_, _, _| None);
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
    if value == b'9' {
        dp.set(pos, Some(1));
        return 1;
    }
    let mut res = 0;
    let top = pos + IVec2::NEG_Y;
    if grid.get(top).copied() == Some(value + 1) {
        res += get_rating(grid, top, dp);
    }
    let bottom = pos + IVec2::Y;
    if grid.get(bottom).copied() == Some(value + 1) {
        res += get_rating(grid, bottom, dp);
    }
    let left = pos + IVec2::NEG_X;
    if grid.get(left).copied() == Some(value + 1) {
        res += get_rating(grid, left, dp);
    }
    let right = pos + IVec2::X;
    if grid.get(right).copied() == Some(value + 1) {
        res += get_rating(grid, right, dp);
    }
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
