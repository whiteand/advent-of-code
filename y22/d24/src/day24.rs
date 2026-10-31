use advent_utils::{
    glam::IVec2,
    grid::{Grid, NonDiagonal},
};
use itertools::Itertools;

struct Map {
    positions: Vec<IVec2>,
    temp_positions: std::cell::RefCell<Vec<IVec2>>,
    directions: Vec<IVec2>,
    size: IVec2,
}

impl Map {
    fn get_wind_positions(&self, t: usize, dst: &mut [IVec2]) {
        for (i, dir) in self.directions.iter().copied().enumerate() {
            let pos = self.positions.get(i).unwrap();

            dst[i].x =
                ((pos.x + (t as i32 % (self.size.x - 2)) * dir.x) - 1) % (self.size.x - 2) + 1;
            dst[i].y =
                ((pos.y + (t as i32 % (self.size.y - 2)) * dir.y) - 1) % (self.size.y - 2) + 1;
        }
    }

    fn get(&self, pos: IVec2, t: usize) -> u8 {
        match pos.to_array() {
            [1, 0] => b'.',
            [_, 0] | [0, _] => b'#',
            [x, _] if x == self.size.x - 1 => b'#',
            [x, y] if x == self.size.x - 2 && y == self.size.y - 1 => b'.',
            [_, y] if y == self.size.y - 1 => b'#',
            [x, y] => {
                let mut temp_pos = self.temp_positions.borrow_mut();

                self.get_wind_positions(t, temp_pos.as_mut_slice());

                let mut found_dir = IVec2::ZERO;
                let mut cnt = 0;
                for (i, p) in temp_pos.iter().copied().enumerate() {
                    if p == IVec2::new(x, y) {
                        cnt += 1;
                        found_dir = self.directions[i];
                    }
                }
                drop(temp_pos);
                if cnt == 0 {
                    return b'.';
                }
                if cnt == 1 {
                    return NonDiagonal::to_ascii_char(found_dir);
                }

                return cnt + b'0';
            }
        }
    }
}

impl std::fmt::Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = Grid::new(self.size, b'.');
        for (p, v) in grid.entries_mut() {
            *v = self.get(p, 2);
        }

        write!(f, "{}", grid.render_ascii())
    }
}

impl Map {
    fn new(size: IVec2) -> Self {
        Self {
            positions: Vec::new(),
            temp_positions: Vec::new().into(),
            directions: Vec::new(),
            size: size,
        }
    }

    fn spawn_wind(&mut self, pos: IVec2, dir: IVec2) {
        self.positions.push(pos);
        self.temp_positions.borrow_mut().push(pos);
        self.directions.push(dir);
    }
}

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> usize {
    let grid = advent_utils::parse::ascii_grid(file_content);
    let mut map = Map::new(grid.size());

    for (p, v) in grid.entries_copy() {
        match v {
            b'#' | b'.' => continue,
            x => {
                let Some(direction) = NonDiagonal::try_from_ascii_char(x) else {
                    unreachable!("{}", v as char);
                };
                map.spawn_wind(p, direction);
            }
        }
    }

    println!("{map:?}");

    0
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
    #[case::example(EXAMPLE, "0")]
    // #[case::actual(ACTUAL, "0")]
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
