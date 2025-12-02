use advent_utils::parse;
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let initials = parse::nums::<u32>(file_content).map(Random).collect_vec();

    initials
        .into_iter()
        .map(|x| x.into_iter().nth(2000).map(|x| x.0).unwrap() as usize)
        .sum::<usize>()
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> u16 {
    get_max_benefit(parse::nums::<u32>(file_content).map(Random))
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Random(u32);
impl Random {
    fn next(&self) -> Random {
        let secret = self.0;
        let a = ((secret << 6) ^ secret) & 0b111111111111111111111111;
        let b = ((a >> 5) ^ a) & 0b111111111111111111111111;
        let c = ((b << 11) ^ b) & 0b111111111111111111111111;
        Self(c)
    }

    fn prices(self) -> impl Iterator<Item = u32> {
        self.into_iter().map(|x| x.0 % 10)
    }
    fn prices_with_offsets(self) -> impl Iterator<Item = (u32, i32)> {
        self.prices()
            .tuple_windows()
            .map(|(b, a)| (a, (a as i32 - b as i32)))
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
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let mut r = self.0;
        for _ in 0..n {
            r = r.next();
        }
        self.0 = r;
        self.next()
    }
}

fn get_max_benefit(randoms: impl Iterator<Item = Random>) -> u16 {
    let mut total_by_key = vec![0u16; 19 * 19 * 19 * 19];
    let mut max_visited_by_key = [None; 19 * 19 * 19 * 19];
    let mut max_total = 0;
    randoms.into_iter().enumerate().for_each(|(i, r)| {
        for (seq, n) in r
            .prices_with_offsets()
            .tuple_windows()
            .take(1997)
            .map(|(a, b, c, d)| ([a, b, c, d].map(|x| x.1), d.0))
        {
            let key = get_seq_key(seq);
            if max_visited_by_key[key].is_none_or(|x| x < i) {
                max_visited_by_key[key] = Some(i);

                let total = total_by_key[key] + (n as u16);
                total_by_key[key] = total;
                if total > max_total {
                    max_total = total;
                }
            }
        }
    });

    max_total
}

fn get_seq_key(seq: [i32; 4]) -> usize {
    let x = seq.map(|x| x + 9);
    (((x[0] * 19 + x[1]) * 19 + x[2]) * 19 + x[3]) as usize
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use crate::{get_max_benefit, Random};

    use super::{part1, part2};
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
        assert_eq!(format!("{}", part1(EXAMPLE)), "37327623");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(ACTUAL)), "19458130434");
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
    /// runs 2.53s
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        let actual = part2(ACTUAL);
        assert_eq!(actual, 2130);
    }
}
