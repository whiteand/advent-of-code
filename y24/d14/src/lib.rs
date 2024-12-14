use advent_utils::{glam::IVec2, parse};
use itertools::Itertools;

#[derive(Debug, Clone)]
struct Robot {
    p: IVec2,
    v: IVec2,
}

impl Robot {
    fn step_many(&mut self, steps: usize, screen_size: IVec2) {
        self.p = (self.p + self.v * (steps as i32)).rem_euclid(screen_size)
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve_part_1_with_size(file_content, IVec2::new(101, 103))
}

#[inline(always)]
fn solve_part_1_with_size(file_content: &str, size: IVec2) -> usize {
    let mut robots = parse_robots(file_content);

    for robot in robots.iter_mut() {
        robot.step_many(100, size);
    }

    let cx = size.x / 2;
    let cy = size.y / 2;

    let mut cnt = [0, 0, 0, 0];
    for robot in robots.iter() {
        if robot.p.x == cx || robot.p.y == cy {
            continue;
        }
        let r = (robot.p.y / (cy + 1)) as usize;
        let c = (robot.p.x / (cx + 1)) as usize;
        let i = (r << 1) | c;
        cnt[i] += 1;
    }

    cnt.into_iter().product()
}

const TREE_TOP_LEFT: IVec2 = IVec2::new(42, 42);
const TREE_BOTTOM_RIGHT: IVec2 = IVec2::new(73, 75);

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str, print: bool) -> usize {
    let mut robots = parse_robots(file_content);
    let size = IVec2::new(101, 103);

    let mut i = 0;
    loop {
        let m = robots
            .iter()
            .tuple_combinations()
            .filter(|(r, r2)| r.p.x.abs_diff(r2.p.x) + r.p.y.abs_diff(r2.p.y) <= 1)
            .count();
        if m >= robots.len() {
            if print {
                print_robots(robots.iter(), TREE_TOP_LEFT, TREE_BOTTOM_RIGHT);
            }
            break i;
        }

        for robot in robots.iter_mut() {
            robot.step_many(1, size);
        }

        i += 1
    }
}

fn print_robots<'t>(robots: impl Iterator<Item = &'t Robot>, from: IVec2, to: IVec2) {
    let map = robots.into_group_map_by(|x| x.p);
    for i in from.y..to.y {
        for j in from.x..to.x {
            let c = map.contains_key(&IVec2::new(j, i));
            if c {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn parse_robots(file_content: &str) -> Vec<Robot> {
    let mut robots = Vec::with_capacity(file_content.trim().lines().count());
    let mut it = parse::nums::<i32>(file_content.trim());
    loop {
        let Some(px) = it.next() else { break robots };
        let py = it.next().unwrap();
        let vx = it.next().unwrap();
        let vy = it.next().unwrap();
        robots.push(Robot {
            p: IVec2::new(px, py),
            v: IVec2::new(vx, vy),
        })
    }
}

#[cfg(test)]
mod tests {
    use advent_utils::glam::IVec2;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_step() {
        let mut robot = super::Robot {
            p: IVec2::new(2, 4),
            v: IVec2::new(2, -3),
        };
        let size = IVec2::new(11, 7);
        robot.step_many(1, size);
        assert_eq!(robot.p, IVec2::new(4, 1));
        robot.step_many(1, size);
        assert_eq!(robot.p, IVec2::new(6, 5));
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(
            format!(
                "{}",
                super::solve_part_1_with_size(EXAMPLE, IVec2::new(11, 7))
            ),
            "12"
        );
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "226548000");
    }

    #[test]
    #[ignore] // runs 25s
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL, false)), "7753");
    }
}
