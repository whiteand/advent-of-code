use std::cmp::Ordering;

use advent_utils::{binary_search, glam::IVec2, grid::Grid, parse};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1<const F: usize>(file_content: &str) -> usize {
    let (ptrs, target) = parse_pointers(file_content);
    let mut corruptions = build_grid(target);
    for x in ptrs.into_iter().take(F) {
        corruptions.set(x, true);
    }
    find_path(&corruptions, target).unwrap()
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> String {
    let (ptrs, target) = parse_pointers(file_content);
    let mut corruptions = build_grid(target);
    let mut prev_number = 0;
    let res = binary_search(1, ptrs.len(), |mid| {
        if mid > prev_number {
            for x in &ptrs[prev_number..mid] {
                corruptions.set(*x, true);
            }
        } else {
            for x in &ptrs[mid..prev_number] {
                corruptions.set(*x, false);
            }
        }
        prev_number = mid;
        find_path(&corruptions, target).map_or(Ordering::Greater, |_| Ordering::Less)
    })
    .expect_err("there will not be 'equal' value");

    format!("{},{}", ptrs[res - 1].x, ptrs[res - 1].y)
}

#[inline(always)]
#[tracing::instrument(skip(corruptions, target))]
pub fn find_path(corruptions: &Grid<bool>, target: IVec2) -> Option<usize> {
    pathfinding::prelude::dijkstra(
        &IVec2::new(0, 0),
        |p| {
            [
                *p + IVec2::NEG_X,
                *p + IVec2::X,
                *p + IVec2::NEG_Y,
                *p + IVec2::Y,
            ]
            .into_iter()
            .filter(|p| p.x >= 0 && p.y >= 0 && p.x <= target.x && p.y <= target.y)
            .filter(|x| !corruptions.get(*x).copied().unwrap_or_default())
            .map(|x| (x, 1i32))
        },
        |p| *p == target,
    )
    .map(|x| x.1 as usize)
}

fn parse_pointers(file_content: &str) -> (Vec<IVec2>, IVec2) {
    let positions = parse::nums::<i32>(file_content)
        .chunks(2)
        .into_iter()
        .map(|x| x.collect_tuple().unwrap())
        .map(|(a, b)| IVec2::new(a, b))
        .collect_vec();
    let mut target = IVec2::new(0, 0);
    for p in &positions {
        target = target.max(*p);
    }

    (positions, target)
}

fn build_grid(target: IVec2) -> Grid<bool> {
    let corruptions = Grid::from_iter((0..=target.y).map(|_| (0..=target.x).map(|_| false)));
    corruptions
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
        assert_eq!(solve_part_1::<12>(EXAMPLE), 22);
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1::<1024>(ACTUAL)), "296");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "6,1");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "28,44");
    }
}
