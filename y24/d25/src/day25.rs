use advent_utils::{grid::Grid, parse};
use itertools::{iproduct, Itertools};

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grids = file_content
        .split("\n\n")
        .map(parse::ascii_grid)
        .collect_vec();
    let (locks, keys): (Vec<_>, Vec<_>) = grids.into_iter().partition(is_lock);
    iproduct!(locks.iter(), keys.iter())
        .filter(|(lock, key)| {
            lock.entries()
                .filter(|(_, v)| **v == b'#')
                .all(|(pos, _)| key.get(pos).filter(|x| **x == b'#').is_none())
        })
        .count()
}

fn is_lock(s: &Grid<u8>) -> bool {
    s.row(0)
        .unwrap()
        .iter()
        .copied()
        .filter(|x| *x == b'#')
        .count()
        >= 5
}

#[cfg(test)]
mod tests {
    use super::part1;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(EXAMPLE)), "3");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(ACTUAL)), "3136");
    }
}
