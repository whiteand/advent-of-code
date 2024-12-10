pub fn solve_part_1(file_content: &str) -> usize {
    file_content
        .lines()
        .flat_map(str_to_card_matches)
        .filter(|matches| *matches > 0)
        .map(|matches| 1 << (matches - 1))
        .sum()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let matches = file_content
        .lines()
        .flat_map(str_to_card_matches)
        .collect::<Vec<_>>();
    let mut cards_instances = vec![1; matches.len()];
    let mut sum = 0;
    for (i, matches) in matches.into_iter().enumerate() {
        let j0 = i + 1;
        let j1 = usize::min(cards_instances.len(), i + matches + 1);
        for j in j0..j1 {
            cards_instances[j] += cards_instances[i];
        }
        sum += cards_instances[i];
    }
    sum
}

fn parse_ints(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.split_ascii_whitespace().map(|s| match s.len() {
        1 => s.as_bytes()[0] - b'0',
        2 => (s.as_bytes()[0] - b'0') * 10 + (s.as_bytes()[1] - b'0'),
        _ => unreachable!("Not expected number: {}", s),
    } as u32)
}

// WARNING: This algorithm assumes that numbers are in 0..128 range
fn str_to_card_matches(line: &str) -> Option<usize> {
    let (_card_title, numbers) = line.split_once(':')?;

    let (win_numbers_str, given_numbers_str) = numbers.split_once('|')?;

    let winning_set = parse_ints(win_numbers_str).fold(0u128, |bitmask, n| bitmask | (1 << n));

    Some(
        parse_ints(given_numbers_str)
            .filter(|n| winning_set & (1 << n) != 0)
            .count(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "13");
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "22193");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "30");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "5625994");
    }
}
