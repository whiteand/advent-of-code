mod try2;

use std::{collections::HashMap, fmt::Write};

use advent_utils::glam::IVec2;
use itertools::Itertools;
use tracing::info;

// actions: &[KeypadAction],
// positions: [IVec2; ROBOTS],
// robot_index: usize,

#[derive(Clone)]
struct CachePerRobot<const ROBOTS: usize> {
    cache_per_positions:
        HashMap<[IVec2; ROBOTS], [Option<(usize, [IVec2; ROBOTS])>; KeypadAction::VALUES_LEN]>,
}

struct PerformActionCache<const ROBOTS: usize> {
    cache_per_robot: Vec<CachePerRobot<ROBOTS>>,
}

impl<const ROBOTS: usize> PerformActionCache<ROBOTS> {
    fn new() -> Self {
        Self {
            cache_per_robot: vec![
                CachePerRobot {
                    cache_per_positions: HashMap::new(),
                };
                ROBOTS
            ],
        }
    }
    fn append(
        &mut self,
        robot_index: usize,
        action: KeypadAction,
        positions: [IVec2; ROBOTS],
        result: (usize, [IVec2; ROBOTS]),
    ) {
        let x = self.cache_per_robot[robot_index]
            .cache_per_positions
            .entry(positions)
            .or_default();

        x[action.to_usize()] = Some(result);
    }

    fn get(
        &mut self,
        robot_index: usize,
        actions: KeypadAction,
        positions: [IVec2; ROBOTS],
    ) -> Option<(usize, [IVec2; ROBOTS])> {
        let x = self.cache_per_robot[robot_index]
            .cache_per_positions
            .get(&positions)?;
        x[actions.to_usize()]
    }
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve::<3>(file_content)
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve::<26>(file_content)
}

#[tracing::instrument(skip(file_content))]
pub fn solve<const ROBOTS: usize>(file_content: &str) -> usize {
    let mut cache = PerformActionCache::<ROBOTS>::new();
    file_content
        .lines()
        .map(|line| {
            let num_part = advent_utils::parse::nums::<usize>(line).next().unwrap();

            min_steps_to_enter_code::<ROBOTS>(line, &mut cache) * num_part
        })
        .sum()
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum KeypadAction {
    Press,
    Move(DirectionButton),
}

impl std::fmt::Display for KeypadAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_u8() as char)
    }
}

impl KeypadAction {
    const VALUES_LEN: usize = 6;
    fn to_u8(&self) -> u8 {
        match self {
            Self::Press => b'A',
            Self::Move(arg0) => arg0.to_u8(),
        }
    }
    fn to_usize(&self) -> usize {
        match self {
            Self::Press => 0,
            Self::Move(DirectionButton::A) => 1,
            Self::Move(DirectionButton::Up) => 2,
            Self::Move(DirectionButton::Down) => 3,
            Self::Move(DirectionButton::Left) => 4,
            Self::Move(DirectionButton::Right) => 5,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum DirectionButton {
    Up,
    A,
    Left,
    Down,
    Right,
}

impl DirectionButton {
    fn to_u8(&self) -> u8 {
        match self {
            Self::Up => b'^',
            Self::A => b'A',
            Self::Left => b'<',
            Self::Down => b'v',
            Self::Right => b'>',
        }
    }
}

impl DirectionButton {
    fn to_vec(&self) -> IVec2 {
        match self {
            DirectionButton::Up => IVec2::new(0, -1),
            DirectionButton::A => IVec2::new(0, 0),
            DirectionButton::Left => IVec2::new(-1, 0),
            DirectionButton::Down => IVec2::new(0, 1),
            DirectionButton::Right => IVec2::new(1, 0),
        }
    }
}

impl std::fmt::Display for DirectionButton {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.to_u8() as char)
    }
}

#[derive(Debug, Copy, Clone)]
struct CodeTask<'t, const ROBOTS: usize> {
    code: &'t [u8],
    positions: [IVec2; ROBOTS],
}

fn min_steps_to_enter_code<const ROBOTS: usize>(
    code: &str,
    cache: &mut PerformActionCache<ROBOTS>,
) -> usize {
    let mut positions = [get_directional_keypad_position(DirectionButton::A); ROBOTS];
    positions[0] = get_position(b'A');

    min_steps_to_solve_code_task(
        CodeTask {
            code: code.as_bytes(),
            positions,
        },
        cache,
    )
}

fn min_steps_to_solve_code_task<'t, const ROBOTS: usize>(
    code_task: CodeTask<'t, ROBOTS>,
    cache: &mut PerformActionCache<ROBOTS>,
) -> usize {
    let Some((&first, rest)) = code_task.code.split_first() else {
        return 0;
    };
    let next_a = get_position(first);
    let mut paths = actions_to_move(code_task.positions[0], next_a, get_value);
    for p in paths.iter_mut() {
        p.push(KeypadAction::Press);
    }
    assert!(!paths.is_empty());
    let mut min_steps = usize::MAX;
    for path in paths {
        info!(?next_a, f = ?(first as char), ?rest, ?path);
        let (steps, mut new_positions) = perform_actions(&path, code_task.positions, 0, cache);
        if steps >= min_steps {
            continue;
        }
        new_positions[0] = next_a;

        let rest_steps = min_steps_to_solve_code_task(
            CodeTask {
                code: rest,
                positions: new_positions,
            },
            cache,
        );
        let total_steps = steps + rest_steps;
        if min_steps > total_steps {
            min_steps = total_steps;
        }
    }
    min_steps
}

fn perform_actions<const ROBOTS: usize>(
    actions: &[KeypadAction],
    positions: [IVec2; ROBOTS],
    robot_index: usize,
    cache: &mut PerformActionCache<ROBOTS>,
) -> (usize, [IVec2; ROBOTS]) {
    let Some((action, rest_actions)) = actions.split_first() else {
        return (0, positions);
    };

    if robot_index == positions.len() - 1 {
        let mut new_positions = positions;
        for action in actions {
            if let KeypadAction::Move(d) = action {
                new_positions[robot_index] += d.to_vec();
            }
        }
        return (actions.len(), new_positions);
    }

    let (after_action_steps, after_action_positions) =
        match cache.get(robot_index, *action, positions) {
            Some(res) => res,
            None => {
                let parent = robot_index + 1;
                let parent_pos = positions[parent];
                let next_parent_value = match action {
                    KeypadAction::Press => DirectionButton::A,
                    KeypadAction::Move(direction_button) => *direction_button,
                };
                let next_parent_pos = get_directional_keypad_position(next_parent_value);
                let mut parent_actions_to_pos =
                    actions_to_move(parent_pos, next_parent_pos, get_directional_keypad_value);

                for x in &mut parent_actions_to_pos {
                    x.push(KeypadAction::Press);
                }

                let mut min_steps = usize::MAX;
                let mut min_new_positions = positions;

                for actions in parent_actions_to_pos {
                    let (steps, mut new_positions) =
                        perform_actions(&actions, positions, parent, cache);
                    if steps > min_steps {
                        continue;
                    }
                    new_positions[parent] = next_parent_pos;
                    min_steps = steps;
                    min_new_positions = new_positions
                }

                cache.append(
                    robot_index,
                    *action,
                    positions,
                    (min_steps, min_new_positions),
                );
                (min_steps, min_new_positions)
            }
        };

    let (rest_steps, rest_positions) =
        perform_actions(rest_actions, after_action_positions, robot_index, cache);

    (rest_steps + after_action_steps, rest_positions)
}

// transitions current position to the target_position
#[tracing::instrument(skip(get_v), ret)]
fn actions_to_move<X>(
    current: IVec2,
    target_p: IVec2,
    get_v: impl Fn(IVec2) -> Option<X>,
) -> Vec<Vec<KeypadAction>> {
    if current == target_p {
        return vec![vec![]];
    }
    let angle1 = IVec2::new(target_p.x, current.y);
    let angle2 = IVec2::new(current.x, target_p.y);
    let mut angles = Vec::with_capacity(2);
    if get_v(angle1).is_some() {
        angles.push(angle1);
    }
    if get_v(angle2).is_some() {
        angles.push(angle2);
    }
    let mut res = Vec::with_capacity(angles.len());
    for angle in angles {
        let mut path = vec![];
        let first_dir = (angle - current).signum();
        let second_dir = (target_p - angle).signum();
        let first_steps = (angle - current).abs().dot(IVec2::splat(1)) as usize;
        let second_steps = (target_p - angle).abs().dot(IVec2::splat(1)) as usize;

        if let Some(c) = dir_to_command(first_dir) {
            path.extend(std::iter::repeat_n(KeypadAction::Move(c), first_steps));
        }
        if let Some(c) = dir_to_command(second_dir) {
            path.extend(std::iter::repeat_n(KeypadAction::Move(c), second_steps));
        }
        res.push(path)
    }
    res
}

fn dir_to_command(dir: IVec2) -> Option<DirectionButton> {
    if dir.x == 0 && dir.y == 0 {
        return None;
    }
    if dir.x == 1 {
        return Some(DirectionButton::Right);
    }
    if dir.x == -1 {
        return Some(DirectionButton::Left);
    }
    if dir.y == -1 {
        return Some(DirectionButton::Up);
    }
    if dir.y == 1 {
        return Some(DirectionButton::Down);
    }
    unreachable!("{dir}")
}

fn get_position(value: u8) -> IVec2 {
    match value {
        b'0' => IVec2::new(1, 3),
        b'A' => IVec2::new(2, 3),
        b'1' => IVec2::new(0, 2),
        b'2' => IVec2::new(1, 2),
        b'3' => IVec2::new(2, 2),
        b'4' => IVec2::new(0, 1),
        b'5' => IVec2::new(1, 1),
        b'6' => IVec2::new(2, 1),
        b'7' => IVec2::new(0, 0),
        b'8' => IVec2::new(1, 0),
        b'9' => IVec2::new(2, 0),
        _ => unreachable!(),
    }
}
fn get_value(pos: IVec2) -> Option<u8> {
    if pos == IVec2::new(1, 3) {
        return Some(b'0');
    }
    if pos == IVec2::new(2, 3) {
        return Some(b'A');
    }
    if pos == IVec2::new(0, 2) {
        return Some(b'1');
    }
    if pos == IVec2::new(1, 2) {
        return Some(b'2');
    }
    if pos == IVec2::new(2, 2) {
        return Some(b'3');
    }
    if pos == IVec2::new(0, 1) {
        return Some(b'4');
    }
    if pos == IVec2::new(1, 1) {
        return Some(b'5');
    }
    if pos == IVec2::new(2, 1) {
        return Some(b'6');
    }
    if pos == IVec2::new(0, 0) {
        return Some(b'7');
    }
    if pos == IVec2::new(1, 0) {
        return Some(b'8');
    }
    if pos == IVec2::new(2, 0) {
        return Some(b'9');
    }
    None
}
fn get_directional_keypad_position(value: DirectionButton) -> IVec2 {
    match value {
        DirectionButton::Up => IVec2::new(1, 0),
        DirectionButton::A => IVec2::new(2, 0),
        DirectionButton::Left => IVec2::new(0, 1),
        DirectionButton::Down => IVec2::new(1, 1),
        DirectionButton::Right => IVec2::new(2, 1),
    }
}
fn get_directional_keypad_value(pos: IVec2) -> Option<DirectionButton> {
    if pos == IVec2::new(2, 0) {
        return Some(DirectionButton::A);
    }
    if pos == IVec2::new(1, 0) {
        return Some(DirectionButton::Up);
    }
    if pos == IVec2::new(0, 1) {
        return Some(DirectionButton::Left);
    }
    if pos == IVec2::new(1, 1) {
        return Some(DirectionButton::Down);
    }
    if pos == IVec2::new(2, 1) {
        return Some(DirectionButton::Right);
    }
    None
}

#[cfg(test)]
mod tests {

    use crate::solve;

    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[test]
    fn test_part1_example() {
        assert_eq!(solve::<3>(EXAMPLE), 126384);
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(solve::<1>(ACTUAL), 35296);
        assert_eq!(solve::<2>(ACTUAL), 74370);
        assert_eq!(solve::<3>(ACTUAL), 176650);
        assert_eq!(solve::<4>(ACTUAL), 432650);
        assert_eq!(solve::<5>(ACTUAL), 1070284);
        assert_eq!(solve::<6>(ACTUAL), 2650152);
        assert_eq!(solve::<7>(ACTUAL), 6582206);
        assert_eq!(solve::<8>(ACTUAL), 16366742);
        // assert_eq!(solve_part_2(ACTUAL) > 251800402);
    }
}
