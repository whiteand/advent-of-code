use std::fmt::Write;

use advent_utils::glam::IVec2;
use itertools::Itertools;
use tracing::info;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    file_content.lines().map(|line| complexity_of(line)).sum()
}

#[derive(Debug, Copy, Clone)]
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
    fn to_u8(&self) -> u8 {
        match self {
            Self::Press => b'A',
            Self::Move(arg0) => arg0.to_u8(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
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

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    file_content.len()
}

#[derive(Debug)]
struct State {
    target: IVec2,
    parents: Vec<IVec2>,
    parents_planned: Vec<Vec<KeypadAction>>,
    keypad_planned_actions: Vec<KeypadAction>,
}

impl State {
    fn enter_code(&mut self, code: &str) {
        for target_code in code.as_bytes() {
            self.plan_entering_code_button(*target_code)
        }

        macro_rules! plan_parent {
            ($child_plan:expr, $parent_ind:expr) => {{
                let child_actions = std::mem::take(&mut $child_plan);
                for child_action in child_actions.iter().copied() {
                    self.plan_parent_to_control_action(child_action, $parent_ind);
                }
                let _ = std::mem::replace(&mut $child_plan, child_actions);
            }};
        }

        plan_parent!(self.keypad_planned_actions, 0);
        plan_parent!(self.parents_planned[0], 1);

        // self.print()
    }

    // fn print(&self) {
    //     let target_plan = self.keypad_planned_actions.iter().join("");
    //     let parent_plan = self.parents_planned[0].iter().join("");
    //     let parent2_plan = self.parents_planned[1].iter().join("");
    //     let my_plan = self.parents_planned[2].iter().join("");
    //     info!(?target_plan, ?parent_plan, ?parent2_plan, ?my_plan)
    // }

    fn plan_parent_to_control_action(&mut self, action: KeypadAction, parent_ind: usize) {
        let next_button = match action {
            KeypadAction::Press => DirectionButton::A,
            KeypadAction::Move(dir) => dir,
        };
        let next_target = get_directional_keypad_position(next_button);
        actions_to_move(
            self.parents[parent_ind],
            next_target,
            &mut self.parents_planned[parent_ind],
            get_directional_keypad_value,
        );
        self.parents[parent_ind] = next_target;
        self.parents_planned[parent_ind].push(KeypadAction::Press);
    }
    fn plan_entering_code_button(&mut self, target_code: u8) {
        let next_target = get_position(target_code);
        actions_to_move(
            self.target,
            next_target,
            &mut self.keypad_planned_actions,
            get_value,
        );
        self.keypad_planned_actions.push(KeypadAction::Press);
        self.target = next_target;
    }
}

#[tracing::instrument(ret)]
fn complexity_of(code: &str) -> usize {
    let mut state = State {
        target: get_position(b'A'),
        parents: vec![
            get_directional_keypad_position(DirectionButton::A),
            get_directional_keypad_position(DirectionButton::A),
        ],
        parents_planned: vec![Vec::with_capacity(54), Vec::with_capacity(324)],
        keypad_planned_actions: Vec::with_capacity(9),
    };

    state.enter_code(code);

    let me_actions_len = state.parents_planned.last().unwrap().len();
    let num_part = advent_utils::parse::nums::<usize>(code).next().unwrap();

    num_part * me_actions_len
}

// transitions current position to the target_position
fn actions_to_move<X>(
    current: IVec2,
    target_p: IVec2,
    path: &mut Vec<KeypadAction>,
    get_v: impl Fn(IVec2) -> Option<X>,
) {
    if current == target_p {
        return;
    }
    let angle1 = IVec2::new(target_p.x, current.y);
    let angle2 = IVec2::new(current.x, target_p.y);
    let angle = match get_v(angle1) {
        Some(_) => angle1,
        None => angle2,
    };
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
    if pos == IVec2::new(1, 3) {
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
    if pos == IVec2::new(0, 1) {
        return Some(b'7');
    }
    if pos == IVec2::new(1, 1) {
        return Some(b'8');
    }
    if pos == IVec2::new(2, 1) {
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
    use crate::complexity_of;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    use rstest::rstest;

    #[rstest]
    #[case("029A", 68 * 29)]
    #[case("980A", 60 * 980)]
    #[case(
        "179A", 68 * 179
    )]
    #[case(
        "456A", 64 * 456
    )]
    #[case(
        "379A", 64 * 379
    )]
    fn test_complexity(#[case] target_code: &str, #[case] expected_result: usize) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(complexity_of(target_code), expected_result);
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve_part_1(EXAMPLE), 126384);
        assert!(solve_part_1(ACTUAL) > 175970);
        assert_eq!(solve_part_1(ACTUAL), 0);
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
