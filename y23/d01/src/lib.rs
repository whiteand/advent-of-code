use advent_utils::nom::AsChar;

static WORDY_DIGITS: [&str; 10] = [
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn process_line_1(line: &str) -> u32 {
    let mut digits_iter = line.as_bytes().iter().filter_map(|f| match f {
        b'0'..=b'9' => Some(f - b'0'),
        _ => None,
    });

    let a = digits_iter.next().unwrap();
    let b = digits_iter.next_back().unwrap_or(a);

    (a * 10 + b).into()
}

fn process_line_2(line: &str) -> u32 {
    let mut a = 0;
    let mut found_first = false;
    let mut b = 0;
    let mut taken = [0; 10];
    for ch in line.chars() {
        if ch.is_dec_digit() {
            let d = ch.to_digit(10).unwrap();
            if !found_first {
                found_first = true;
                a = d;
            }
            b = d;
            taken.fill(0);
            continue;
        }
        for i in 0..10 {
            let word = WORDY_DIGITS[i];
            let expected = word.as_bytes()[taken[i]];
            if ch as u8 != expected {
                if ch == word.as_bytes()[0].as_char() {
                    taken[i] = 1;
                } else {
                    taken[i] = 0;
                }
                continue;
            }
            taken[i] += 1;

            if taken[i] != word.len() {
                continue;
            }

            taken[i] = 0;
            if !found_first {
                found_first = true;
                a = i as u32;
            }
            b = i as u32;
        }
    }

    a * 10 + b
}

fn solve(file_content: &str, handle_line: impl Fn(&str) -> u32) -> u32 {
    file_content.lines().map(handle_line).sum()
}

pub fn solve_part_1(file_content: &str) -> u32 {
    solve(file_content, process_line_1)
}
pub fn solve_part_2(file_content: &str) -> u32 {
    solve(file_content, process_line_2)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const INPUT2: &str = include_str!("../example2.txt");
    const INPUT3: &str = include_str!("../example3.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "142");
    }
    #[test]
    fn test_task3() {
        assert_eq!(format!("{}", solve_part_1(INPUT3)), "53080");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "54953");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_part_2(INPUT2)), "281");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "53868");
    }
    #[test]
    fn test_task2_actual2() {
        assert_eq!(format!("{}", solve_part_2(INPUT3)), "53268");
    }
}
