use advent_utils::{declare_array, declare_field, parse};
use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
struct State(u128);

impl State {
    declare_array!(u128, u8, bank, set_bank, 8, 7, 0b0111_1111);
    declare_field!(u128, usize, len, set_len, 0, 0b1111_1111);
    fn max_bank_position(&self) -> (usize, u8) {
        (0..self.len()).fold((0, self.bank(0)), |(prev_i, prev_max), i| {
            let bank = self.bank(i);
            if prev_max >= bank {
                (prev_i, prev_max)
            } else {
                (i, bank)
            }
        })
    }
    fn redistribute(&self) -> Self {
        let mut res;
        let (max_i, max_value) = self.max_bank_position();
        res = self.set_bank(max_i, 0);
        let ptr = (max_i + 1) % self.len();
        for i in 0..(max_value as usize) {
            let p = (ptr + i) % self.len();
            let prev = res.bank(p);
            res = res.set_bank(p, prev + 1);
        }
        res
    }
    fn redistributions(&self) -> impl Iterator<Item = Self> {
        std::iter::successors(Some(*self), |x| Some(x.redistribute()))
    }
}

impl std::fmt::Debug for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.len() {
            let bank = self.bank(i);
            if i > 0 {
                write!(f, "\t{bank}")?
            } else {
                write!(f, "{bank}")?
            }
        }
        Ok(())
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let banks = parse::nums::<u8>(file_content).collect_vec();
    let n = banks.len();
    let state = banks
        .into_iter()
        .enumerate()
        .fold(State(0).set_len(n), |s, (i, x)| s.set_bank(i, x));

    let mut prevs = FxHashSet::default();

    for (i, x) in state.redistributions().enumerate() {
        if !prevs.insert(x) {
            return i;
        }
    }
    0
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let banks = parse::nums::<u8>(file_content).collect_vec();
    let n = banks.len();
    let state = banks
        .into_iter()
        .enumerate()
        .fold(State(0).set_len(n), |s, (i, x)| s.set_bank(i, x));

    let mut first_occurence: FxHashMap<State, usize> = FxHashMap::default();

    for (i, x) in state.redistributions().enumerate() {
        match first_occurence.entry(x) {
            std::collections::hash_map::Entry::Occupied(e) => {
                let prev = *e.get();
                return i - prev;
            }
            std::collections::hash_map::Entry::Vacant(e) => {
                e.insert(i);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "5")]
    #[case::actual(ACTUAL, "11137")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(EXAMPLE, "4")]
    #[case::actual(ACTUAL, "1037")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
