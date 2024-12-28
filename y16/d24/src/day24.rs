use advent_utils::{glam::IVec2, grid::NonDiagonal, parse};
use fxhash::FxHashMap;
use tracing::info;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct State {
    pos: IVec2,
    visited: u16,
}

impl State {
    fn new(zero_pos: IVec2) -> Self {
        Self {
            pos: zero_pos,
            visited: 0b1,
        }
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content.trim());
    let mut digits_coords = FxHashMap::default();
    let mut zero_pos = IVec2::ZERO;
    for (p, d) in grid.entries() {
        let digit = match *d {
            b'0'..=b'9' => *d - b'0',
            _ => continue,
        };
        if digit == 0 {
            zero_pos = p;
        }
        digits_coords.insert(p, digit);
    }

    let total_visited = digits_coords
        .values()
        .copied()
        .fold(0, |acc, b| acc | (1 << (b as usize)));

    let res = pathfinding::prelude::dijkstra(
        &State::new(zero_pos),
        |s| {
            let s = *s;
            grid.neighbours(s.pos, NonDiagonal)
                .filter(|(_, n)| **n != b'#')
                .map(move |(p, n)| {
                    let n = *n;
                    if n == b'.' {
                        return (
                            State {
                                pos: p,
                                visited: s.visited,
                            },
                            1,
                        );
                    }
                    let digit = n - b'0';
                    let new_visited = s.visited | (1 << digit);

                    (
                        State {
                            pos: p,
                            visited: new_visited,
                        },
                        1,
                    )
                })
        },
        |x| x.visited == total_visited,
    )
    .unwrap();

    res.1
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let grid = parse::ascii_grid(file_content.trim());
    let mut digits_coords = FxHashMap::default();
    let mut zero_pos = IVec2::ZERO;
    for (p, d) in grid.entries() {
        let digit = match *d {
            b'0'..=b'9' => *d - b'0',
            _ => continue,
        };
        if digit == 0 {
            zero_pos = p;
        }
        digits_coords.insert(p, digit);
    }

    info!("grid:\n{}", grid.render_ascii());
    info!(?digits_coords, ?zero_pos);

    let total_visited = digits_coords
        .values()
        .copied()
        .fold(0, |acc, b| acc | (1 << (b as usize)));

    let res = pathfinding::prelude::dijkstra(
        &State::new(zero_pos),
        |s| {
            let s = *s;
            grid.neighbours(s.pos, NonDiagonal)
                .filter(|(_, n)| **n != b'#')
                .map(move |(p, n)| {
                    let n = *n;
                    if n == b'.' {
                        return (
                            State {
                                pos: p,
                                visited: s.visited,
                            },
                            1,
                        );
                    }
                    let digit = n - b'0';
                    let new_visited = s.visited | (1 << digit);

                    (
                        State {
                            pos: p,
                            visited: new_visited,
                        },
                        1,
                    )
                })
        },
        |x| x.visited == total_visited && x.pos == zero_pos,
    )
    .unwrap();

    res.1
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "14")]
    #[case::actual(ACTUAL, "430")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "20")]
    #[case::actual(ACTUAL, "700")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
