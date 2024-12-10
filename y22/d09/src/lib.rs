use std::collections::HashSet;

mod moves;
mod parse;

fn solve<const N: usize>(file_content: &str) -> usize {
    let mut rope_x = [0; N];
    let mut rope_y = [0; N];
    let mut s = HashSet::new();

    for mut m in parse::parse_moves(file_content) {
        for _ in 0..m.distance {
            m.apply(&mut rope_x[0], &mut rope_y[0]);
            for i in 1..N {
                let dx = rope_x[i - 1] - rope_x[i];
                let dy = rope_y[i - 1] - rope_y[i];
                if dy.abs() <= 1 && dx.abs() <= 1 {
                    continue;
                }
                rope_x[i] += dx.signum();
                rope_y[i] += dy.signum();
            }
            let tail_pos_x = rope_x[N - 1];
            let tail_pos_y = rope_y[N - 1];
            s.insert((tail_pos_x, tail_pos_y));
        }
    }
    s.len()
}

pub fn solve_part_1(file_content: &str) -> usize {
    solve::<2>(file_content)
}
pub fn solve_part_2(file_content: &str) -> usize {
    solve::<10>(file_content)
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";
    #[test]
    fn test_y22_d9_p1() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "13");
    }
    #[test]
    fn test_y22_d9_p1_actual() {
        let str = include_str!("../input.txt");
        let res = solve_part_1(&str);
        assert_eq!(res, 6067);
    }
    #[test]
    fn test_y22_d9_p2_actual() {
        let str = include_str!("../input.txt");
        let res = solve_part_2(&str);
        assert_eq!(res, 2471);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(
            format!(
                "{}",
                solve_part_2(
                    "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
                )
            ),
            "36"
        );
    }
    #[test]
    fn test_part_2_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "1");
    }
}
