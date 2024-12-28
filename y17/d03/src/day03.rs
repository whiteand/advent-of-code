use advent_utils::{glam::IVec2, grid::N8};
use fxhash::FxHashMap;
use tracing::info;

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let n = file_content.trim().parse::<usize>().unwrap();
    positions().nth(n - 1).unwrap().abs().dot(IVec2::splat(1)) as usize
}

pub fn positions() -> impl Iterator<Item = IVec2> {
    let dirs = [IVec2::X, IVec2::NEG_Y, IVec2::NEG_X, IVec2::Y];
    let sizes = (1..).flat_map(|x| std::iter::repeat_n(x, 2));

    dirs.into_iter()
        .cycle()
        .zip(sizes)
        .flat_map(|(d, n)| std::iter::repeat_n(d, n))
        .scan(IVec2::ZERO, |p, d| {
            let res = *p;
            *p += d;
            Some(res)
        })
}

#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> usize {
    let n = file_content.trim().parse::<usize>().unwrap();
    let mut map = FxHashMap::default();
    map.insert(IVec2::ZERO, 1);
    for p in positions().skip(1) {
        let value = N8
            .into_iter()
            .map(|x| x + p)
            .flat_map(|p| map.get(&p).copied())
            .sum();
        if value > n {
            return value;
        }
        map.insert(p, value);
    }
    0
}

#[cfg(test)]
mod tests {
    use super::{part1, part2};
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example("23", "2")]
    #[case::example("1024", "31")]
    #[case::actual(ACTUAL, "552")]
    fn test_part1(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input)), expected);
    }
    #[rstest]
    #[case::actual(ACTUAL, "330785")]
    fn test_part2(#[case] input: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input)), expected);
    }
}
