use advent_utils::parse;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let discs = parse_discs(file_content);
    solve(&discs)
}
fn solve(discs: &[Disc]) -> usize {
    let mut positions = discs.iter().map(|x| x.positions()).collect_vec();
    let mut current_pos = positions
        .iter_mut()
        .map(|x| x.next().unwrap())
        .collect_vec();
    for t in 0.. {
        if current_pos
            .iter()
            .copied()
            .enumerate()
            .all(|(i, p)| p == (discs[i].positions * 2 - i) % discs[i].positions)
        {
            return t - 1;
        }
        for (dst, src) in current_pos.iter_mut().zip(positions.iter_mut()) {
            *dst = src.next().unwrap();
        }
    }
    usize::MAX
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    let mut discs = parse_discs(file_content);
    discs.push(Disc {
        positions: 11,
        start_pos: 0,
    });

    solve(&discs)
}

#[derive(Debug)]
struct Disc {
    start_pos: usize,
    positions: usize,
}

impl Disc {
    fn positions(&self) -> impl Iterator<Item = usize> {
        (0..self.positions).cycle().skip(self.start_pos)
    }
}

fn parse_discs(file_content: &str) -> Vec<Disc> {
    let mut nums = parse::nums::<usize>(file_content);
    let mut res = Vec::with_capacity(6);
    loop {
        let Some(_) = nums.next() else {
            break;
        };
        let positions = nums.next().unwrap();
        let start_position = nums.nth(1).unwrap();
        res.push(Disc {
            start_pos: start_position,
            positions,
        })
    }
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "5");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "376777");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "85");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "3903937");
    }
}
