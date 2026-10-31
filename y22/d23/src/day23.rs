use std::collections::{HashMap, HashSet};

use advent_utils::{glam::IVec2, grid::N8};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(file_content);
    println!("{}", grid.render_ascii());
    let mut elves_positions = grid
        .entries_copy()
        .filter(|(_, v)| *v == b'#')
        .map(|(p, _)| p)
        .collect::<HashSet<_>>();

    print_positions(&elves_positions);

    for i in 0..10 {
        make_iteration(&mut elves_positions, i);
        println!("After {} iteration: ", i + 1);
        print_positions(&elves_positions);
    }

    get_part1_score(&elves_positions)
}

fn make_iteration(elves_positions: &mut HashSet<IVec2>, iteration: usize) {
    let half_round_actions = elves_positions
        .iter()
        .copied()
        .map(|p| {
            let neighbours = N8::neighbours_of(p).map(|x| elves_positions.contains(&x));

            // If no other Elves are in one of those eight positions,
            // the Elf does not do anything during this round.
            if !neighbours.contains(&true) {
                return None;
            }
            let [nw, n, ne, e, se, s, sw, w] = neighbours;

            let dirs = [
                ([n, ne, nw], IVec2::NEG_Y),
                ([s, se, sw], IVec2::Y),
                ([w, nw, sw], IVec2::NEG_X),
                ([e, ne, se], IVec2::X),
            ];

            for (has, dir) in dirs
                .iter()
                .cycle()
                .copied()
                .skip(iteration % dirs.len())
                .take(dirs.len())
            {
                if !has.contains(&true) {
                    println!("{} proposes {} (to move into {})", p, dir, p + dir);
                    return Some((p, dir));
                }
            }

            None
        })
        .filter_map(|x| x)
        .collect_vec();

    let mut movements = HashMap::new();
    let mut forbidden_positions = HashSet::new();

    for (prev, dir) in half_round_actions {
        let new_pos = prev + dir;
        if movements.contains_key(&new_pos) || forbidden_positions.contains(&new_pos) {
            movements.remove(&new_pos);
            forbidden_positions.insert(new_pos);
        } else {
            movements.insert(new_pos, prev);
        }
    }

    for (new_p, prev_p) in movements {
        elves_positions.remove(&prev_p);
        elves_positions.insert(new_p);
    }
}

fn get_bounding_box(positions: &HashSet<IVec2>) -> [IVec2; 2] {
    let first = positions.iter().next().copied().unwrap();
    let mut left_top = first;
    let mut bottom_right = first;
    for p in positions.iter().copied() {
        left_top = left_top.min(p);
        bottom_right = bottom_right.max(p);
    }

    [left_top, bottom_right]
}

fn get_part1_score(positions: &HashSet<IVec2>) -> usize {
    let [left_top, bottom_right] = get_bounding_box(positions);
    let total_area = ((bottom_right - left_top) + IVec2::new(1, 1)).element_product() as usize;
    total_area - positions.len()
}

fn print_positions(positions: &HashSet<IVec2>) {
    let [left_top, bottom_right] = get_bounding_box(&positions);
    print!("  ");
    for x in left_top.x..=bottom_right.x {
        print!("{}", x);
    }
    println!();
    for y in left_top.y..=bottom_right.y {
        print!("{y} ");
        for x in left_top.x..=bottom_right.x {
            if positions.contains(&IVec2 { x, y }) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    file_content.len()
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "110")]
    #[case::actual(ACTUAL, "0")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
    #[ignore]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
