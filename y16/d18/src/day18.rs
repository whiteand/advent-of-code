use itertools::Itertools;

#[derive(Clone, Copy)]
enum Tile {
    Safe,
    Trap,
}

impl std::fmt::Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Safe => write!(f, "."),
            Self::Trap => write!(f, "^"),
        }
    }
}

#[tracing::instrument]
pub fn part1(file_content: &str) -> usize {
    solve(file_content, 40)
}
#[tracing::instrument]
pub fn part2(file_content: &str) -> usize {
    solve(file_content, 400000)
}

#[tracing::instrument]
pub fn solve(file_content: &str, number: usize) -> usize {
    let tiles = file_content
        .trim()
        .as_bytes()
        .iter()
        .copied()
        .map(|x| match x {
            b'.' => Tile::Safe,
            b'^' => Tile::Trap,
            x => unreachable!("{x}"),
        })
        .collect_vec();

    let mut rows = Rows::new(tiles);

    (0..number)
        .map(|_| {
            rows.next()
                .iter()
                .copied()
                .filter(|x| matches!(x, Tile::Safe))
                .count()
        })
        .sum()
}

#[derive(Debug, Clone)]
struct Rows {
    current_row: Vec<Tile>,
    next_row: Vec<Tile>,
}

impl Rows {
    fn new(tiles: Vec<Tile>) -> Self {
        Self {
            next_row: tiles.clone(),
            current_row: tiles,
        }
    }
    fn next(&mut self) -> &[Tile] {
        self.next_row.clear();
        self.next_row.extend(
            std::iter::once(Tile::Safe)
                .chain(self.current_row.iter().copied())
                .chain(std::iter::once(Tile::Safe))
                .tuple_windows()
                .map(|(left, center, right)| match (left, center, right) {
                    (Tile::Safe, _, Tile::Trap) => Tile::Trap,
                    (Tile::Trap, _, Tile::Safe) => Tile::Trap,
                    _ => Tile::Safe,
                }),
        );

        std::mem::swap(&mut self.current_row, &mut self.next_row);

        self.next_row.as_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::solve;
    use rstest::rstest;
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(".^^.^.^^^^", 10, 38)]
    #[case::actual(ACTUAL, 40, 2005)]
    // #[case::actual(ACTUAL, 400000, 20008491)]
    fn test_part1(#[case] input: &str, #[case] rows: usize, #[case] expected: usize) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve(input, rows), expected);
    }
}
