use itertools::Either;
use pathfinding::num_traits::CheckedSub;

fn get_cell(favorite_num: usize, x: usize, y: usize) -> Cell {
    if (x * x + 3 * x + 2 * x * y + y + y * y + favorite_num).count_ones() % 2 == 0 {
        Cell::Open
    } else {
        Cell::Wall
    }
}

enum Cell {
    Wall,
    Open,
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let favorite = file_content.trim().parse::<usize>().unwrap();
    pathfinding::prelude::dijkstra(
        &(1, 1),
        |(x, y)| {
            let x = *x;
            let y = *y;
            [
                Some((x + 1, y)),
                x.checked_sub(&1).map(|x| (x, y)),
                Some((x, y + 1)),
                y.checked_sub(&1).map(|y| (x, y)),
            ]
            .into_iter()
            .flatten()
            .filter(|(x, y)| matches!(get_cell(favorite, *x, *y), Cell::Open))
            .map(|p| (p, 1usize))
        },
        |(x, y)| *x == 31 && *y == 39,
    )
    .map(|(_, cost)| cost)
    .unwrap_or(usize::MAX)
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let favorite = file_content.trim().parse::<usize>().unwrap();
    let x = pathfinding::prelude::dijkstra_reach(&(1, 1), |(x, y), c| {
        if c >= 50 {
            return Either::Left(std::iter::empty());
        }
        let x = *x;
        let y = *y;
        Either::Right(
            [
                Some((x + 1, y)),
                x.checked_sub(&1).map(|x| (x, y)),
                Some((x, y + 1)),
                y.checked_sub(&1).map(|y| (x, y)),
            ]
            .into_iter()
            .flatten()
            .filter(|(x, y)| matches!(get_cell(favorite, *x, *y), Cell::Open))
            .map(|p| (p, 1usize)),
        )
    });

    x.count()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "92");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "124");
    }
}
