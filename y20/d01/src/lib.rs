use std::collections::HashSet;

fn parse_nums_list(s: &str) -> impl Iterator<Item = usize> + '_ {
    s.lines().map(|s| s.parse().unwrap())
}

pub fn solve_part_1(file_content: &str) -> impl std::fmt::Display {
    let nums = parse_nums_list(file_content);
    let mut occurred_set = HashSet::new();
    for num in nums {
        if occurred_set.contains(&num) {
            continue;
        }
        occurred_set.insert(num);
        let other_num = 2020 - num;
        if occurred_set.contains(&other_num) {
            return num * other_num;
        }
    }
    0
}
// Right answer: 51810360
// Iterations: 977104
pub fn solve_part_2(file_content: &str) -> impl std::fmt::Display {
    let nums = parse_nums_list(file_content).collect::<Vec<_>>();
    let n = nums.len();
    let mut iter = 0;
    for i in 0..n - 2 {
        for j in (i + 1)..(n - 1) {
            for k in (j + 1)..n {
                iter += 1;
                let a = nums[i];
                let b = nums[j];
                let c = nums[k];
                if a + b + c == 2020 {
                    dbg!(iter);
                    return a * b * c;
                }
            }
        }
    }
    0
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1721
979
366
299
675
1456";
    #[test]
    fn test_part_1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "514579");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "241861950");
    }
}
