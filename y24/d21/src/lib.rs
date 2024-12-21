use std::fmt::Write;

use advent_utils::glam::IVec2;
use tracing::info;

#[derive(Debug, Copy, Clone)]
enum KeypadAction {
    Press,
    Move(DirectionButton),
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
pub fn solve_part_1(file_content: &str) -> String {
    get_directions_for(file_content.trim())
}
#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    file_content.len()
}

#[tracing::instrument(ret)]
fn get_directions_for(code: &str) -> String {
    let mut res = String::with_capacity(128);
    let mut current_target_input_position = get_position(b'A');
    let mut current_parent_position = get_directional_keypad_position(b'A');
    let mut current_parent2_position = get_directional_keypad_position(b'A');
    info!(
        ?current_target_input_position,
        ?current_parent_position,
        ?current_parent2_position
    );
    let mut actions = Vec::with_capacity(9);

    for target_code in code.as_bytes() {
        actions.clear();
        let target_p = get_position(*target_code);
        actions_to_move(
            current_target_input_position,
            target_p,
            &mut actions,
            get_value,
        );
        actions.push(KeypadAction::Press);

        for action in actions.iter().copied() {
            execute_keypad_action(
                action,
                &mut current_target_input_position,
                &mut current_parent_position,
                &mut current_parent2_position,
                &mut res,
            );
        }
    }

    res
}

// transitions current position to the target_position
fn actions_to_move(
    current: IVec2,
    target_p: IVec2,
    path: &mut Vec<KeypadAction>,
    get_v: impl Fn(IVec2) -> Option<u8>,
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

/// Outputs commands necessary to
fn execute_keypad_action(
    action: KeypadAction,
    _current_target_input_position: &mut IVec2,
    _current_parent_position: &mut IVec2,
    _current_parent2_position: &mut IVec2,
    res: &mut String,
) {
    match action {
        KeypadAction::Press => res.push('A'),
        KeypadAction::Move(direction_button) => {
            let dir = direction_button.to_vec();
            *_current_target_input_position += dir;
            res.push(direction_button.to_u8() as char);
        }
    }
    info!(?action);
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
fn get_directional_keypad_position(value: u8) -> IVec2 {
    match value {
        b'A' => IVec2::new(2, 0),
        b'^' => IVec2::new(1, 0),
        b'<' => IVec2::new(0, 1),
        b'v' => IVec2::new(1, 1),
        b'>' => IVec2::new(2, 1),
        _ => unreachable!(),
    }
}
fn get_directional_keypad_value(pos: IVec2) -> Option<u8> {
    if pos == IVec2::new(2, 0) {
        return Some(b'A');
    }
    if pos == IVec2::new(1, 0) {
        return Some(b'^');
    }
    if pos == IVec2::new(0, 1) {
        return Some(b'<');
    }
    if pos == IVec2::new(1, 1) {
        return Some(b'v');
    }
    if pos == IVec2::new(2, 1) {
        return Some(b'>');
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::get_directions_for;

    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    use rstest::rstest;

    #[rstest]
    #[case(
        "029A",
        "<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A"
    )]
    // #[case("980A", "<v<A>>^AAAvA^A<vA<AA>>^AvAA<^A>A<v<A>A>^AAAvA<^A>A<vA>^A<A>A")]
    // #[case(
    //     "179A",
    //     "<v<A>>^A<vA<A>>^AAvAA<^A>A<v<A>>^AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    // )]
    // #[case(
    //     "456A",
    //     "<v<A>>^AA<vA<A>>^AAvAA<^A>A<vA>^A<A>A<vA>^A<A>A<v<A>A>^AAvA<^A>A"
    // )]
    // #[case(
    //     "379A",
    //     "<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A"
    // )]
    fn test_part1(#[case] target_code: &str, #[case] expected_directions: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(get_directions_for(target_code), expected_directions);
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
