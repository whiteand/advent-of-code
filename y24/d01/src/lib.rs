pub fn solve_part_1(file_content: &str) -> usize {
    let (mut xs, mut ys) = parse(file_content);

    xs.sort_unstable();
    ys.sort_unstable();

    xs.into_iter().zip(ys).map(|(a, b)| a.abs_diff(b)).sum()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let (xs, ys) = parse(file_content);
    let max_y = ys.iter().copied().max().unwrap_or_default();
    let mut cnt = vec![0; max_y + 1];
    for y in ys {
        cnt[y] += 1;
    }

    xs.into_iter()
        .filter(|x| *x <= max_y)
        .map(|x| cnt[x] * x)
        .sum()
}

fn parse(file_content: &str) -> (Vec<usize>, Vec<usize>) {
    let mut xs = Vec::with_capacity(1000);
    let mut ys = Vec::with_capacity(1000);
    let mut bytes = file_content.bytes().into_iter();
    let mut state = 0usize;
    let mut nums: [usize; 2] = [0, 0];
    while let Some(b) = bytes.next() {
        match b {
            b'\n' => {
                state = 0;
                xs.push(nums[0]);
                ys.push(nums[1]);
                nums[0] = 0;
                nums[1] = 0;
            }
            b'0'..=b'9' => {
                nums[state] *= 10;
                nums[state] += (b - b'0') as usize;
            }
            b' ' => {
                state = 1;
            }
            _ => unreachable!(),
        }
    }

    (xs, ys)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "11");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "765748");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "31");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "27732508");
    }
}
