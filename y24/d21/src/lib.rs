use std::collections::HashMap;

use advent_utils::glam::IVec2;
use itertools::{Either, Itertools};
use tracing::info;

#[tracing::instrument(skip(file_content))]
pub fn solve<const ROBOTS: usize>(file_content: &str) -> usize {
    let mut cache = HashMap::new();
    let res = file_content
        .lines()
        .map(|line| {
            let num_part = advent_utils::parse::nums::<usize>(line).next().unwrap();

            min_steps_for_code::<ROBOTS>(line, &mut cache) * num_part
        })
        .sum();
    for ((tasks, controllers), v) in cache.into_iter().sorted() {
        info!(?tasks, ?controllers, ?v);
    }
    res
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DirectionButton {
    Up,
    A,
    Left,
    Down,
    Right,
}

const POSITIONS_BY_VALUE: [IVec2; 18] = [
    IVec2::new(1, 3),
    IVec2::new(0, 2),
    IVec2::new(1, 2),
    IVec2::splat(2),
    IVec2::new(0, 1),
    IVec2::splat(1),
    IVec2::new(2, 1),
    IVec2::ZERO,
    IVec2::new(1, 0),
    IVec2::new(2, 0),
    IVec2::ZERO,
    IVec2::ZERO,
    IVec2::ZERO,
    IVec2::ZERO,
    IVec2::ZERO,
    IVec2::ZERO,
    IVec2::ZERO,
    IVec2::new(2, 3),
];

fn get_numeric_position(value: u8) -> IVec2 {
    POSITIONS_BY_VALUE[(value - b'0') as usize]
}

const NUMERIC_BY_POS_IND: [Option<u8>; 12] = [
    Some(b'7'),
    Some(b'8'),
    Some(b'9'),
    Some(b'4'),
    Some(b'5'),
    Some(b'1'),
    Some(b'2'),
    Some(b'3'),
    Some(b'6'),
    None,
    Some(b'0'),
    Some(b'A'),
];
fn get_numeric_value(pos: IVec2) -> Option<u8> {
    let ind = (pos.y * 3 + pos.x) as usize;
    NUMERIC_BY_POS_IND[ind]
}

const POSITION_PER_DIRECTIONAL_BUTTON: [IVec2; 5] = [
    IVec2::new(1, 0),
    IVec2::new(2, 0),
    IVec2::new(0, 1),
    IVec2::new(1, 1),
    IVec2::new(2, 1),
];
fn get_directional_keypad_position(value: DirectionButton) -> IVec2 {
    POSITION_PER_DIRECTIONAL_BUTTON[value as usize]
}
const DIRECTION_KEYPAD_VALUE: [Option<DirectionButton>; 6] = [
    None,
    Some(DirectionButton::Up),
    Some(DirectionButton::A),
    Some(DirectionButton::Left),
    Some(DirectionButton::Down),
    Some(DirectionButton::Right),
];

fn get_directional_keypad_value(pos: IVec2) -> Option<DirectionButton> {
    DIRECTION_KEYPAD_VALUE[(pos.y * 3 + pos.x) as usize]
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl From<Direction> for DirectionButton {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => DirectionButton::Up,
            Direction::Down => DirectionButton::Down,
            Direction::Left => DirectionButton::Left,
            Direction::Right => DirectionButton::Right,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Copy, Hash, Debug)]
enum RobotTask {
    Move { direction: Direction, steps: usize },
    Press(usize),
}

impl PartialOrd for RobotTask {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for RobotTask {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let self_steps = match self {
            RobotTask::Move { steps, .. } => steps,
            RobotTask::Press(steps) => steps,
        };
        let other_steps = match other {
            RobotTask::Move { steps, .. } => steps,
            RobotTask::Press(steps) => steps,
        };

        self_steps.cmp(other_steps)
    }
}

fn get_possible_trajectories(code: &str) -> impl Iterator<Item = Vec<RobotTask>> + '_ {
    std::iter::once(b'A')
        .chain(code.as_bytes().iter().copied())
        .tuple_windows()
        .map(|(a, b)| {
            if a == b {
                return vec![vec![RobotTask::Press(1)]];
            }
            let ap = get_numeric_position(a);
            let bp = get_numeric_position(b);
            match get_paths(ap, bp, get_numeric_value) {
                Either::Left(x) => vec![vec![x, RobotTask::Press(1)]],
                Either::Right(Either::Left([x, y])) => vec![vec![x, y, RobotTask::Press(1)]],
                Either::Right(Either::Right(([a, b], [c, d]))) => vec![
                    vec![a, b, RobotTask::Press(1)],
                    vec![c, d, RobotTask::Press(1)],
                ],
            }
        })
        .multi_cartesian_product()
        .map(|x| x.into_iter().flatten().collect_vec())
}

fn get_paths<X>(
    ap: IVec2,
    bp: IVec2,
    get_v: impl Fn(IVec2) -> Option<X>,
) -> Either<RobotTask, Either<[RobotTask; 2], ([RobotTask; 2], [RobotTask; 2])>> {
    if ap.x == bp.x {
        if ap.y > bp.y {
            return Either::Left(RobotTask::Move {
                direction: Direction::Up,
                steps: (ap.y - bp.y) as usize,
            });
        } else {
            return Either::Left(RobotTask::Move {
                direction: Direction::Down,
                steps: (bp.y - ap.y) as usize,
            });
        }
    }
    if ap.y == bp.y {
        if ap.x > bp.x {
            return Either::Left(RobotTask::Move {
                direction: Direction::Left,
                steps: (ap.x - bp.x) as usize,
            });
        } else {
            return Either::Left(RobotTask::Move {
                direction: Direction::Right,
                steps: (bp.x - ap.x) as usize,
            });
        }
    }
    let angle1 = IVec2::new(ap.x, bp.y);
    let angle2 = IVec2::new(bp.x, ap.y);
    match (get_v(angle1), get_v(angle2)) {
        (None, None) => unreachable!(),
        (None, Some(_)) => {
            let path = [move_from_to(ap, angle2), move_from_to(angle2, bp)];
            Either::Right(Either::Left(path))
        }
        (Some(_), None) => Either::Right(Either::Left([
            move_from_to(ap, angle1),
            move_from_to(angle1, bp),
        ])),
        (Some(_), Some(_)) => Either::Right(Either::Right((
            [move_from_to(ap, angle1), move_from_to(angle1, bp)],
            [move_from_to(ap, angle2), move_from_to(angle2, bp)],
        ))),
    }
}

fn move_from_to(a: IVec2, b: IVec2) -> RobotTask {
    let delta = b - a;
    let dir = delta.signum();
    let steps = ((delta.x + delta.y) / (dir.x + dir.y)) as usize;
    match (dir.x, dir.y) {
        (-1, 0) => RobotTask::Move {
            direction: Direction::Left,
            steps,
        },
        (1, 0) => RobotTask::Move {
            direction: Direction::Right,
            steps,
        },
        (0, -1) => RobotTask::Move {
            direction: Direction::Up,
            steps,
        },
        (0, 1) => RobotTask::Move {
            direction: Direction::Down,
            steps,
        },
        x => unreachable!("{x:?}"),
    }
}

type Cache = HashMap<(usize, usize), usize>;

fn min_steps_for_code<const ROBOTS: usize>(code: &str, cache: &mut Cache) -> usize {
    get_possible_trajectories(code)
        .map(move |controls| min_steps_to_execute_controls(&controls, ROBOTS, cache))
        .min()
        .unwrap_or(usize::MAX)
}

fn total_steps(controls: &[RobotTask]) -> usize {
    controls.iter().copied().fold(0, |a, x| match x {
        RobotTask::Move { steps, .. } => steps + a,
        RobotTask::Press(steps) => steps + a,
    })
}

fn controls_key(controls: &[RobotTask]) -> usize {
    controls.iter().fold(0, |res, c| match c {
        RobotTask::Move { direction, steps } => {
            assert!(*steps <= 3, "{} > 3", *steps);
            (((res << 3)
                | match direction {
                    Direction::Up => 0b000,
                    Direction::Down => 0b010,
                    Direction::Left => 0b011,
                    Direction::Right => 0b100,
                })
                << 2)
                | *steps
        }
        RobotTask::Press(steps) => {
            assert!(*steps <= 3, "{} > 3", *steps);
            (((res << 3) | 0b101) << 2) | *steps
        }
    })
}
fn min_steps_to_execute_controls(
    controls: &[RobotTask],
    controllers: usize,
    cache: &mut Cache,
) -> usize {
    if controllers == 0 {
        return total_steps(controls);
    }
    let key = controls_key(controls);
    if let Some(x) = cache.get(&(key, controllers)) {
        return *x;
    }
    let mut min_steps = 0;

    let mut current_pos = get_directional_keypad_position(DirectionButton::A);
    for c in controls {
        let (target_pos, steps) = match c {
            RobotTask::Move { direction, steps } => {
                let target_button: DirectionButton = (*direction).into();
                (get_directional_keypad_position(target_button), steps)
            }
            RobotTask::Press(steps) => (get_directional_keypad_position(DirectionButton::A), steps),
        };
        if target_pos == current_pos {
            min_steps +=
                min_steps_to_execute_controls(&[RobotTask::Press(*steps)], controllers - 1, cache);
            continue;
        }

        match get_paths(current_pos, target_pos, get_directional_keypad_value) {
            Either::Left(p) => {
                min_steps += min_steps_to_execute_controls(
                    &[p, RobotTask::Press(*steps)],
                    controllers - 1,
                    cache,
                );
                current_pos = target_pos;
            }
            Either::Right(Either::Left([a, b])) => {
                min_steps += min_steps_to_execute_controls(
                    &[a, b, RobotTask::Press(*steps)],
                    controllers - 1,
                    cache,
                );
                current_pos = target_pos;
            }
            Either::Right(Either::Right((p1, p2))) => {
                let left_cost = min_steps_to_execute_controls(
                    &[p1[0], p1[1], RobotTask::Press(*steps)],
                    controllers - 1,
                    cache,
                );
                let right_cost = min_steps_to_execute_controls(
                    &[p2[0], p2[1], RobotTask::Press(*steps)],
                    controllers - 1,
                    cache,
                );

                min_steps += left_cost.min(right_cost);

                current_pos = target_pos;
            }
        }
    }

    cache.insert((key, controllers), min_steps);

    min_steps
}

#[cfg(test)]
mod tests {

    use crate::solve;

    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solve::<2>(EXAMPLE), 126384);
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(solve::<2>(ACTUAL), 176650);
    }
    #[test]
    fn test_part2_actual() {
        assert_eq!(solve::<25>(ACTUAL), 217698355426872);
    }
}
