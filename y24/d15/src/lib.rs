use advent_utils::{glam::IVec2, grid::Grid, parse};
use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    let (first, second) = file_content.split_once("\n\n").unwrap();
    let mut grid = parse::ascii_grid(first);
    tracing::info!("\n{}", grid.render_ascii());
    let robot_pos = grid
        .coords()
        .find(|x| grid.get(*x).copied() == Some(b'@'))
        .unwrap();

    tracing::info!(?robot_pos);
    let moves = second
        .as_bytes()
        .iter()
        .filter_map(|x| match x {
            // >>vv<v>>v<
            b'<' => Some(IVec2::NEG_X),
            b'v' => Some(IVec2::Y),
            b'>' => Some(IVec2::X),
            b'^' => Some(IVec2::NEG_Y),
            _ => None,
            //
        })
        .collect_vec();
    grid.set(robot_pos, b'.').unwrap();

    play(&mut grid, robot_pos, &moves);

    grid.coords()
        .filter(|x| grid.get(*x).copied() == Some(b'O'))
        .map(|p| (100 * p.y + p.x) as usize)
        .sum::<usize>()
}

fn print_grid(grid: &mut Grid<u8>, player: IVec2, m: IVec2) {
    let prev = grid.set(player, b'@').unwrap();
    tracing::info!("\n{}\nm={m}", grid.render_ascii());
    grid.set(player, prev);
}
fn play(grid: &mut Grid<u8>, mut player: IVec2, moves: &[IVec2]) -> IVec2 {
    for m in moves {
        print_grid(grid, player, *m);

        let m = *m;
        if grid.get(player + m).copied().unwrap() == b'.' {
            player += m;
            continue;
        }

        let boxes = (1i32..)
            .take_while(|i| {
                let p = player + m * *i;
                grid.get(p).copied().map_or(false, |x| x == b'O')
            })
            .count();
        if boxes == 0 {
            continue;
        }
        if grid
            .get(player + m * (1 + boxes as i32))
            .copied()
            .unwrap_or_default()
            == b'#'
        {
            continue;
        }
        grid.set(player + m * (1 + boxes as i32), b'O');
        grid.set(player + m, b'.');
        player += m;
    }
    player
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    0
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
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "0");
    }

    #[test]
    #[ignore]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "0");
    }
}
