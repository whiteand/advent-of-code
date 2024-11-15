use itertools::Itertools;

pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, 40)
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, 50)
}
pub fn solve(file_content: &str, n: usize) -> usize {
    let mut seq = file_content
        .lines()
        .take(1)
        .flat_map(|line| line.as_bytes().iter().copied())
        .map(|b| (b - b'0') as u64)
        .collect_vec();
    let mut buf = Vec::with_capacity(seq.len());
    expand_many(&mut seq, &mut buf, n);
    seq.len()
}
fn expand_many(mut seq: &mut Vec<u64>, mut buf: &mut Vec<u64>, n: usize) {
    for _ in 0..(n / 2) {
        expand(&seq, &mut buf);
        expand(&buf, &mut seq);
    }
    if n % 2 == 1 {
        expand(&seq, &mut buf);
        seq.truncate(buf.len());
        seq.copy_from_slice(&buf);
    }
}
fn expand(src: &[u64], dest: &mut Vec<u64>) {
    if src.is_empty() {
        dest.truncate(0);
        return;
    }

    dest.clear();
    let mut ch = src[0];
    let mut cnt = 1;
    for &x in src.iter().skip(1) {
        if x == ch {
            cnt += 1;
        } else {
            dest.push(cnt);
            dest.push(ch);
            cnt = 1;
            ch = x;
        }
    }
    dest.push(cnt);
    dest.push(ch);
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        assert_eq!(format!("{}", solve_part_1(EXAMPLE)), "82350");
    }

    #[test]
    fn test_part1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "329356");
    }

    #[test]
    fn test_part2() {
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "1166642");
    }

    #[test]
    fn test_part2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "4666278");
    }
}
