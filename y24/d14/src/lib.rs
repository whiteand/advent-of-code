use advent_utils::{glam::IVec2, parse};
use itertools::Itertools;
use quadtree_rs::{area::AreaBuilder, point::Point, Quadtree};

#[derive(Debug, Clone)]
struct Robot {
    p: IVec2,
    v: IVec2,
}

impl Robot {
    fn step_many(&mut self, steps: usize, screen_size: IVec2) {
        self.p.x = (self.p.x + self.v.x * steps as i32).rem_euclid(screen_size.x);
        self.p.y = (self.p.y + self.v.y * steps as i32).rem_euclid(screen_size.y);
    }

    fn is_in(&self, top_left: IVec2, bottom_right: IVec2) -> bool {
        (top_left.x..bottom_right.x).contains(&self.p.x)
            && (top_left.y..bottom_right.y).contains(&self.p.y)
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve_part_1_with_size(file_content, IVec2::new(101, 103))
}

fn solve_part_1_with_size(file_content: &str, size: IVec2) -> usize {
    let mut robots = parse_robots(file_content);

    for robot in robots.iter_mut() {
        robot.step_many(100, size);
    }

    let quadrant_size = IVec2::new(size.x / 2, size.y / 2);

    let (mut a, mut b, mut c, mut d) = (0, 0, 0, 0);
    for robot in robots.iter() {
        if robot.is_in(IVec2::new(0, 0), quadrant_size) {
            a += 1;
        }
        if robot.is_in(
            IVec2::new(quadrant_size.x + 1, 0),
            IVec2::new(size.x, quadrant_size.y),
        ) {
            b += 1;
        }
        if robot.is_in(
            IVec2::new(0, quadrant_size.y + 1),
            IVec2::new(quadrant_size.x, size.y),
        ) {
            c += 1;
        }
        if robot.is_in(
            IVec2::new(quadrant_size.x + 1, quadrant_size.y + 1),
            IVec2::new(size.x, size.y),
        ) {
            d += 1;
        }
    }

    a * b * c * d
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
            .enumerate()
            .filter(|(i, r)| {
                robots
                    .iter()
                    .skip(i + 1)
                    .any(|r2| (r.p - r2.p).abs().dot(IVec2::splat(1)) <= 1)
            })
            .count();
        if m >= robots.len() / 2 {
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
            let c = map.get(&IVec2::new(j, i)).is_some();
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
    parse::nums::<i32>(file_content.trim())
        .chunks(4)
        .into_iter()
        .map(|x| x.collect_tuple().unwrap())
        .map(|(a, b, c, d)| Robot {
            p: IVec2::new(a, b),
            v: IVec2::new(c, d),
        })
        .collect_vec()
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
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL, false)), "7753");
    }
}
