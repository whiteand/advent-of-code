use std::str::FromStr;

trait IsValid {
    fn is_valid(&self, password: &str) -> bool;
}

struct OldWorkRule {
    char: char,
    min: usize,
    max: usize,
}

impl IsValid for OldWorkRule {
    fn is_valid(&self, password: &str) -> bool {
        let count = password.chars().filter(|c| *c == self.char).count();
        count >= self.min && count <= self.max
    }
}

impl FromStr for OldWorkRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let mut min_max = parts.next().unwrap().split('-');
        let min = min_max.next().unwrap().parse().unwrap();
        let max = min_max.next().unwrap().parse().unwrap();
        let char = parts.next().unwrap().chars().next().unwrap();
        Ok(OldWorkRule { char, min, max })
    }
}

struct CurrentRule {
    char: char,
    positions: [usize; 2],
}

impl FromStr for CurrentRule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let mut positions = parts.next().unwrap().split('-');
        let first: usize = positions.next().unwrap().parse().unwrap();
        let second: usize = positions.next().unwrap().parse().unwrap();
        let char = parts.next().unwrap().chars().next().unwrap();
        Ok(Self {
            char,
            positions: [usize::min(first, second) - 1, usize::max(first, second) - 1],
        })
    }
}

impl IsValid for CurrentRule {
    fn is_valid(&self, password: &str) -> bool {
        let mut cnt = 0;
        for (i, ch) in password.chars().enumerate() {
            if i > self.positions[1] {
                break;
            }
            if !self.positions.contains(&i) {
                continue;
            }
            if ch == self.char {
                cnt += 1;
            }
        }
        cnt == 1
    }
}

fn count_password<Rule>(file_content: &str) -> usize
where
    Rule: IsValid,
    Rule: FromStr,
    Rule::Err: std::fmt::Debug,
{
    let mut res = 0;
    for line in file_content.lines() {
        let mut parts = line.split(": ");
        let rule = parts.next().unwrap().parse::<Rule>().unwrap();
        let password = parts.next().unwrap();
        if rule.is_valid(password) {
            res += 1;
        }
    }
    res
}

pub fn solve_part_1(file_content: &str) -> impl std::fmt::Display {
    count_password::<OldWorkRule>(file_content)
}
pub fn solve_part_2(file_content: &str) -> impl std::fmt::Display {
    count_password::<CurrentRule>(file_content)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1-3 a: abcde
1-3 b: cdefg
2-9 c: ccccccccc";
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "2");
    }
    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "1");
    }
}
