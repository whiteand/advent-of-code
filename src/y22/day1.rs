use crate::y22::reduces::Reduces;

fn parse_elfes_calories(file_content: &str) -> impl Iterator<Item = u32> + '_ {
    file_content
        .lines()
        .map(|line| line.trim())
        .reduces(0, |elf_group, line| {
            if line.is_empty() {
                return false;
            }
            *elf_group += line.parse::<u32>().unwrap();

            true
        })
}

pub fn solve_part1(file_content: &str) -> u32 {
    parse_elfes_calories(file_content).max().unwrap_or_default()
}

pub fn solve_part2(file_content: &str) -> u32 {
    let mut top1 = 0;
    let mut top2 = 0;
    let mut top3 = 0;

    for elf in parse_elfes_calories(file_content) {
        if elf <= top3 {
            continue;
        }
        if elf <= top2 {
            top3 = elf;
        } else if elf <= top1 {
            top3 = top2;
            top2 = elf;
        } else {
            top3 = top2;
            top2 = top1;
            top1 = elf;
        }
    }

    top1 + top2 + top3
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1000
    2000
    3000
    
    4000
    
    5000
    6000
    
    7000
    8000
    9000
    
    10000";

    #[ignore]
    #[test]
    fn test_part1() {
        assert_eq!(solve_part1(INPUT), 24000);
    }
    #[ignore]
    #[test]
    fn test_part2() {
        assert_eq!(solve_part2(INPUT), 45000);
    }
}
