use advent_utils::nom::{
    self,
    branch::alt,
    bytes::complete::tag,
    character::complete::line_ending,
    combinator::{all_consuming, value},
    multi::separated_list1,
    parse_usize,
    sequence::{preceded, terminated},
    Parser,
};
use itertools::{Either, Itertools};
use tracing::{info, trace};

#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str, seq: &str) -> String {
    let (_, instructions) = parse_instructions(file_content).unwrap();
    scramble(&instructions, seq)
}
fn scramble(instructions: &[Instruction], seq: &str) -> String {
    let mut state = seq.as_bytes().to_vec();
    state.reserve(state.len() * 2);
    trace!(text = unsafe { String::from_utf8_unchecked(state.clone()) });
    for instruction in instructions.iter().cloned() {
        match instruction {
            Instruction::SwapPosition(a, b) => {
                state.swap(a, b);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
            Instruction::SwapLetter(a, b) => {
                for x in state.iter_mut() {
                    if *x == a {
                        *x = b;
                    } else if *x == b {
                        *x = a;
                    }
                }
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
            Instruction::ReversePositions(a, b) => {
                let (a, b) = (a.min(b), a.max(b));
                state[a..=b].reverse();
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
            Instruction::RotateLeft(offset) => {
                rotate_left(&mut state, offset);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
            Instruction::RotateRight(offset) => {
                rotate_right(&mut state, offset);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
            Instruction::MovePosition(a, b) => {
                let value = state[a];
                state.remove(a);
                state.insert(b, value);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
            Instruction::RotateBasedOnPositionOfLetter(letter) => {
                let ind = state.iter().position(|x| *x == letter).unwrap();
                let mut rights = ind + 1;
                if ind >= 4 {
                    rights += 1;
                }
                rotate_right(&mut state, rights);
                trace!(
                    ?instruction,
                    ?ind,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
            }
        }
    }
    trace!(text = unsafe { String::from_utf8_unchecked(state.clone()) });
    unsafe { String::from_utf8_unchecked(state) }
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str, seq: &str) -> String {
    let (_, instructions) = parse_instructions(file_content).unwrap();
    let mut state = seq.as_bytes().to_vec();
    state.reserve(state.len() * 2);
    let mut options = unscramble(state, &instructions);
    for x in options.iter() {
        info!(value = ?unsafe { String::from_utf8_unchecked(x.clone()) });
    }
    unsafe { String::from_utf8_unchecked(options.pop().unwrap()) }
}

fn unscramble(input: Vec<u8>, instructions: &[Instruction]) -> Vec<Vec<u8>> {
    trace!(text = unsafe { String::from_utf8_unchecked(input.clone()) });
    let mut reversed = vec![input];
    let mut buf = vec![];
    for instruction in instructions.iter().copied().rev() {
        buf.append(&mut reversed);
        reversed.extend(buf.drain(..).flat_map(|mut state| match instruction {
            Instruction::SwapPosition(a, b) => {
                state.swap(a, b);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
                Either::Left(std::iter::once(state))
            }
            Instruction::SwapLetter(a, b) => {
                for x in state.iter_mut() {
                    if *x == a {
                        *x = b;
                    } else if *x == b {
                        *x = a;
                    }
                }
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
                Either::Left(std::iter::once(state))
            }
            Instruction::ReversePositions(a, b) => {
                let (a, b) = (a.min(b), a.max(b));
                state[a..=b].reverse();
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
                Either::Left(std::iter::once(state))
            }
            Instruction::RotateLeft(offset) => {
                rotate_right(&mut state, offset);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
                Either::Left(std::iter::once(state))
            }
            Instruction::RotateRight(offset) => {
                rotate_left(&mut state, offset);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
                Either::Left(std::iter::once(state))
            }
            Instruction::MovePosition(a, b) => {
                let value = state[b];
                state.remove(b);
                state.insert(a, value);
                trace!(
                    ?instruction,
                    text = unsafe { String::from_utf8_unchecked(state.clone()) }
                );
                Either::Left(std::iter::once(state))
            }
            Instruction::RotateBasedOnPositionOfLetter(letter) => {
                let j = state.iter().position(|x| *x == letter).unwrap();
                let is = (0..state.len())
                    .filter(|i| {
                        let i = *i;
                        if i >= 4 {
                            ((i + i + 2) % state.len()) == j
                        } else {
                            ((i + i + 1) % state.len()) == j
                        }
                    })
                    .collect_vec();

                assert!(!is.is_empty());

                Either::Right(is.into_iter().map(move |i| {
                    let mut new_state = state.clone();

                    let lefts = if i >= 4 { i + 2 } else { i + 1 };

                    rotate_left(&mut new_state, lefts);
                    trace!(
                        ?instruction,
                        text = unsafe { String::from_utf8_unchecked(new_state.clone()) }
                    );
                    new_state
                }))
            }
        }))
    }
    reversed
}

fn rotate_left(state: &mut Vec<u8>, offset: usize) {
    let n = state.len();
    let offset = offset % n;
    if offset == 0 {
        return;
    }
    state.extend_from_within(0..offset);
    state.copy_within(offset.., 0);
    state.truncate(n);
}
fn rotate_right(state: &mut Vec<u8>, offset: usize) {
    let offset = offset % state.len();
    rotate_left(state, state.len() - offset);
}

#[derive(Debug, Copy, Clone)]
enum Instruction {
    // swap letter f with letter a
    SwapLetter(u8, u8),
    // swap position 2 with position 7
    SwapPosition(usize, usize),
    // rotate left 1 step
    // rotate left 4 steps
    RotateLeft(usize),
    // rotate right 6 steps
    RotateRight(usize),
    // rotate based on position of letter g
    RotateBasedOnPositionOfLetter(u8),
    // reverse positions 4 through 7
    ReversePositions(usize, usize),
    // move position 0 to position 6
    MovePosition(usize, usize),
}

fn parse_instructions(input: &str) -> nom::IResult<&str, Vec<Instruction>> {
    all_consuming(separated_list1(line_ending, parse_instruction)).parse(input.trim())
}
fn parse_instruction(input: &str) -> nom::IResult<&str, Instruction> {
    alt((
        // swap letter f with letter a
        (
            tag("swap letter "),
            nom::character::complete::anychar.map(|x| x as u8),
            tag(" with letter "),
            nom::character::complete::anychar.map(|x| x as u8),
        )
            .map(|(_, from, _, to)| Instruction::SwapLetter(from, to)),
        // swap position 2 with position 7
        (
            tag("swap position "),
            parse_usize,
            tag(" with position "),
            parse_usize,
        )
            .map(|(_, from, _, to)| Instruction::SwapPosition(from, to)),
        // reverse positions 4 through 7
        (
            tag("reverse positions "),
            parse_usize,
            tag(" through "),
            parse_usize,
        )
            .map(|(_, from, _, to)| Instruction::ReversePositions(from, to)),
        // move position 0 to position 6
        (
            tag("move position "),
            parse_usize,
            tag(" to position "),
            parse_usize,
        )
            .map(|(_, from, _, to)| Instruction::MovePosition(from, to)),
        // rotate left 1 step
        // rotate left 4 steps
        preceded(tag("rotate left "), parse_steps).map(Instruction::RotateLeft),
        preceded(tag("rotate right "), parse_steps).map(Instruction::RotateRight),
        // rotate based on position of letter g
        preceded(
            tag("rotate based on position of letter "),
            nom::character::complete::anychar.map(|x| x as u8),
        )
        .map(Instruction::RotateBasedOnPositionOfLetter),
    ))
    .parse(input)
}

fn parse_steps(input: &str) -> nom::IResult<&str, usize> {
    alt((
        value(1, tag("1 step")),
        terminated(parse_usize, tag(" steps")),
    ))
    .parse(input)
}
#[cfg(test)]
mod tests {
    use super::{part1, part2, Instruction};
    use advent_utils::rand::{self, Rng};
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[rstest]
    #[case::example(EXAMPLE, "abcde", "decab")]
    #[case::actual(ACTUAL, "abcdefgh", "gbhcefad")]
    fn test_part1(#[case] input: &str, #[case] seq: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(input, seq)), expected);
    }
    #[rstest]
    #[case::example(EXAMPLE, "decab", "abcde")]
    #[case::actual(ACTUAL, "gbhcefad", "abcdefgh")]
    #[case::actual(ACTUAL, "fbgdceah", "gahedfcb")]
    fn test_part2(#[case] input: &str, #[case] seq: &str, #[case] expected: &str) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part2(input, seq)), expected);
    }

    fn random_instruction(r: &mut rand::rngs::ThreadRng) -> Instruction {
        let t = r.gen_range(0..7);
        match t {
            0 => {
                let a = r.gen_range(b'a'..=b'h');
                let b = r.gen_range(b'a'..=b'h');
                Instruction::SwapLetter(a, b)
            }
            1 => {
                let a = r.gen_range(0..8);
                let b = r.gen_range(0..8);
                Instruction::SwapPosition(a, b)
            }
            2 => {
                let a = r.gen_range(0..16);
                Instruction::RotateLeft(a)
            }
            3 => {
                let a = r.gen_range(0..16);
                Instruction::RotateRight(a)
            }
            4 => {
                let a = r.gen_range(b'a'..=b'h');
                Instruction::RotateBasedOnPositionOfLetter(a)
            }
            5 => {
                let a = r.gen_range(0..8);
                let b = r.gen_range(0..8);
                Instruction::ReversePositions(a, b)
            }
            6 => {
                let a = r.gen_range(0..8);
                let b = r.gen_range(0..8);
                Instruction::MovePosition(a, b)
            }
            x => {
                panic!("Unknown type: {x:?}");
            }
        }
    }

    #[test]
    fn test_unscramble() {
        use itertools::Itertools;
        let mut r = rand::thread_rng();
        for _ in 0..100 {
            let instructions = (0usize..r.gen_range(1..101))
                .map(|_| random_instruction(&mut r))
                .collect_vec();
            let seq = "fbgdceah";
            let scrambled = super::scramble(&instructions, seq);
            let unscrambled = super::unscramble(scrambled.as_bytes().to_vec(), &instructions);

            assert!(
                unscrambled
                    .iter()
                    .map(|x| String::from_utf8_lossy(x).to_string())
                    .collect_vec()
                    .contains(&seq.to_owned()),
                "{instructions:?} {seq}"
            );
        }
    }
}
