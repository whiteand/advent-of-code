/// N - how much larger the space should become.
/// if N = 1, then space is not expanded
pub fn solve<const N: usize>(file_content: &str) -> usize {
    let mut stars = file_content
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .flat_map(move |(col, c)| (c == '#').then_some((row, col)))
        })
        .collect::<Vec<_>>();
    let rows = stars.iter().map(|(r, _)| r).max().unwrap() + 1;
    let cols = stars.iter().map(|(_, c)| c).max().unwrap() + 1;

    // Expanding

    let without_stars_cols = (0..cols)
        .filter(|col| !stars.iter().any(|(_, c)| c == col))
        .collect::<Vec<_>>();
    let without_stars_rows = (0..rows)
        .filter(|row| !stars.iter().any(|(r, _)| r == row))
        .collect::<Vec<_>>();

    stars = stars
        .into_iter()
        .map(|(row, col)| {
            let row = without_stars_rows.iter().filter(|r| **r < row).count() * (N - 1) + row;
            let col = without_stars_cols.iter().filter(|c| **c < col).count() * (N - 1) + col;
            (row, col)
        })
        .collect();

    // Finding sum of distances

    stars
        .iter()
        .enumerate()
        .flat_map(|(ind, s)| {
            stars.iter().skip(ind).map(move |s2| {
                let dr = if s2.0 > s.0 { s2.0 - s.0 } else { s.0 - s2.0 };
                let dc = if s2.1 > s.1 { s2.1 - s.1 } else { s.1 - s2.1 };
                dr + dc
            })
        })
        .filter(|d| *d != 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d11/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d11.txt");

    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve::<2>(INPUT)), "374");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve::<2>(ACTUAL)), "10033566");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve::<10>(INPUT)), "1030");
    }
    #[test]
    fn test_task2_2() {
        assert_eq!(format!("{}", solve::<100>(INPUT)), "8410");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve::<1000000>(ACTUAL)), "560822911938");
    }
}
