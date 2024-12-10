// Empty -> whitespace -> Empty
// Empty -> " -> (Text, +code, 0)
// Text -> \ -> (OneSlash, +code, 0)
// OneSlash -> \ -> (Text, +code, +char)
// OneSlash -> " -> (Text, +code, +char)
// OneSlash -> x -> (SlashHex, +code, 0)
// OneSlash -> _ -> (Text, +code, +2char)
// SlashHex -> 0-9a-fA-F -> (SlashHexDigit, +code, 0)
// SlashHex -> _ -> (Text, +code, +2char)
// SlashHexDigit -> 0-9a-fA-F -> (Text, +code, +char)
// SlashHexDigit -> _ -> (Text, +code, +2char)

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum State {
    Empty,
    Text,
    OneSlash,
    SlashHex,
    SlashHexDigit,
}
pub fn solve_part_1(file_content: &str) -> usize {
    let bytes = file_content.as_bytes();
    let mut chars = 0;
    let mut code = 0;
    let mut state = State::Empty;
    let mut bytes_it = bytes.iter().copied().peekable();
    while let Some(byte) = bytes_it.peek() {
        match (state, *byte) {
            (State::Empty, b' ' | b'\n') => {
                bytes_it.next();
            }
            (State::Empty, b'"') => {
                bytes_it.next();
                state = State::Text;
                code += 1;
            }
            (State::Text, b'"') => {
                bytes_it.next();
                state = State::Empty;
                code += 1;
            }
            (State::Text, ch) if ch.is_ascii_lowercase() => {
                bytes_it.next();
                code += 1;
                chars += 1;
            }
            (State::Text, b'\\') => {
                bytes_it.next();
                code += 1;
                state = State::OneSlash;
            }
            (State::OneSlash, b'"') => {
                bytes_it.next();
                code += 1;
                chars += 1;
                state = State::Text;
            }
            (State::OneSlash, b'\\') => {
                bytes_it.next();
                code += 1;
                chars += 1;
                state = State::Text;
            }
            (State::OneSlash, b'x') => {
                bytes_it.next();
                code += 1;
                state = State::SlashHex;
            }
            (State::SlashHex, ch) if ch.is_ascii_digit() || (b'a'..=b'f').contains(&ch) => {
                bytes_it.next();
                code += 1;
                state = State::SlashHexDigit;
            }
            (State::SlashHexDigit, ch) if ch.is_ascii_digit() || (b'a'..=b'f').contains(&ch) => {
                bytes_it.next();
                code += 1;
                chars += 1;
                state = State::Text;
            }
            (state, n) => panic!("Unexpected state: {state:?} char: '{}'", n as char),
        }
    }
    code - chars
}
pub fn solve_part_2(file_content: &str) -> usize {
    file_content
        .lines()
        .map(|line| {
            line.as_bytes()
                .iter()
                .copied()
                .map(|b| match b {
                    b'\\' | b'"' => 1,
                    x if x.is_ascii_digit() => 0,
                    x if x.is_ascii_lowercase() => 0,
                    x if x > b'0' => panic!("Unknown character: '{}'", x as char),
                    _ => 0usize,
                })
                .sum::<usize>()
                + 2usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "12");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "1371");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "19");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "2117");
    }
}
