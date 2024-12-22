use advent_utils::parse;
use itertools::Itertools;
use tracing::info;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Random(usize);
impl Random {
    fn next(&self) -> Random {
        let secret = self.0;

        let a = ((secret << 6) ^ secret) & 0b111111111111111111111111;
        let b = ((a >> 5) ^ a) & 0b111111111111111111111111;
        let c = ((b << 11) ^ b) & 0b111111111111111111111111;
        Self(c)
    }

    fn prices(self) -> impl Iterator<Item = usize> {
        self.into_iter().map(|x| x.0 % 10)
    }
    fn prices_with_offsets(self) -> impl Iterator<Item = (usize, i32)> {
        self.prices()
            .skip(1)
            .zip(self.prices())
            .map(|(a, b)| (a, (a as i32 - b as i32)))
    }
}

impl IntoIterator for Random {
    type Item = Self;

    type IntoIter = RandomIter;

    fn into_iter(self) -> Self::IntoIter {
        RandomIter(self)
    }
}

struct RandomIter(Random);

impl Iterator for RandomIter {
    type Item = Random;

    fn next(&mut self) -> Option<Self::Item> {
        let res = self.0;
        self.0 = self.0.next();
        Some(res)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (usize::MAX, None)
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let initials = parse::nums::<usize>(file_content).map(Random).collect_vec();

    initials
        .into_iter()
        .map(|x| x.into_iter().nth(2000).map(|x| x.0).unwrap())
        .sum::<usize>()
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    get_max_benefit(parse::nums::<usize>(file_content).map(Random))
}

fn get_max_benefit(randoms: impl Iterator<Item = Random>) -> usize {
    let results = randoms
        .into_iter()
        .map(|r| {
            let mut sequences = fxhash::FxHashMap::default();
            for (seq, n) in r
                .prices_with_offsets()
                .tuple_windows()
                .take(1997)
                .map(|(a, b, c, d)| ([a, b, c, d].map(|x| x.1), d.0))
            {
                let key = get_seq_key(seq);
                sequences.entry(key).or_insert_with(|| n);
            }

            sequences
        })
        .collect_vec();

    results
        .iter()
        .flat_map(|x| x.keys())
        .copied()
        .unique()
        .map(|x| {
            results
                .iter()
                .flat_map(|prices| prices.get(&x).copied())
                .sum()
        })
        .max()
        .unwrap_or(0)
}

fn get_seq_key(seq: [i32; 4]) -> usize {
    let x = seq.map(|x| x + 9);
    (x[0] * 19 * 19 * 19 + x[1] * 19 * 19 + x[2] * 19 + x[3]) as usize
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{get_max_benefit, Random};

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_random() {
        let random = super::Random(123);
        assert_eq!(
            random.into_iter().take(11).map(|x| x.0).collect_vec(),
            vec![
                123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
                7753432, 5908254
            ]
        )
    }
    #[test]
    fn test_prices() {
        let random = super::Random(123);
        assert_eq!(random.prices().take(4).collect_vec(), vec![3, 0, 6, 5])
    }
    #[test]
    fn test_prices_with_offsets() {
        let random = super::Random(123);
        assert_eq!(
            random.prices_with_offsets().take(9).collect_vec(),
            vec![
                (0, -3),
                (6, 6),
                (5, -1),
                (4, -1),
                (4, 0),
                (6, 2),
                (4, -2),
                (4, 0),
                (2, -2)
            ]
        );
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "37327623");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "19458130434");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(get_max_benefit([1, 2, 3, 2024].map(Random).into_iter()), 23);
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        let actual = solve_part_2(ACTUAL);
        assert_eq!(actual, 2130);
    }
}
