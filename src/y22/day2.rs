#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Choice {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<char> for Choice {
    fn from(value: char) -> Self {
        match value {
            'A' => Choice::Rock,
            'B' => Choice::Paper,
            'C' => Choice::Scissors,
            'X' => Choice::Rock,
            'Y' => Choice::Paper,
            'Z' => Choice::Scissors,
            _ => unreachable!(),
        }
    }
}

fn play(opponent: Choice, my_choice: Choice) -> Outcome {
    match (opponent, my_choice) {
        (Choice::Rock, Choice::Paper) => Outcome::Win,
        (Choice::Rock, Choice::Scissors) => Outcome::Loss,
        (Choice::Paper, Choice::Rock) => Outcome::Loss,
        (Choice::Paper, Choice::Scissors) => Outcome::Win,
        (Choice::Scissors, Choice::Rock) => Outcome::Win,
        (Choice::Scissors, Choice::Paper) => Outcome::Loss,
        _ => Outcome::Draw,
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl From<char> for Outcome {
    fn from(c: char) -> Self {
        match c {
            'X' => Outcome::Loss,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => unreachable!(),
        }
    }
}

fn get_score((a, b): (Choice, Choice)) -> u32 {
    play(a, b) as u32 + b as u32
}

fn parse_char_pairs(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
    input.lines().map(|line| {
        let mut chars = line.chars();
        let first_char = chars.next().unwrap();
        chars.next();
        let second_char = chars.next().unwrap();

        (first_char, second_char)
    })
}

pub fn solve_part1(file_content: &str) -> u32 {
    parse_char_pairs(file_content)
        .map(|(a, b)| (a.into(), b.into()))
        .map(get_score)
        .sum::<u32>()
}

fn restore_your_move(opponent: Choice, outcome: Outcome) -> Choice {
    for my_choice in [Choice::Rock, Choice::Paper, Choice::Scissors] {
        if play(opponent, my_choice) == outcome {
            return my_choice;
        }
    }
    unreachable!("No move found")
}

pub fn solve_part2(file_content: &str) -> u32 {
    parse_char_pairs(file_content)
        .map(|(a, b)| (a.into(), b.into()))
        .map(|(opponent, outcome)| (opponent, restore_your_move(opponent, outcome)))
        .map(get_score)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "A Y\nB X\nC Z";

    #[ignore]
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 15);
    }
    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 12);
    }
}
