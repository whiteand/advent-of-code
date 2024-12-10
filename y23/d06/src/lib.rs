use std::ops::BitXor;

pub fn solve_part_1(file_content: &str) -> usize {
    let mut line_it = file_content.lines();
    let times = parse_ints(line_it.next().unwrap());
    let distances = parse_ints(line_it.next().unwrap());
    times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| get_possible_ways_to_win(time, distance))
        .product()
}
pub fn solve_part_2(file_content: &str) -> usize {
    let mut nums = file_content.lines().map(|line| {
        line.split_ascii_whitespace()
            .skip(1)
            .collect::<String>()
            .parse()
            .unwrap()
    });
    let time = nums.next().unwrap();
    let distance = nums.next().unwrap();

    get_possible_ways_to_win(time, distance)
}

fn get_possible_ways_to_win(time: usize, distance: usize) -> usize {
    let d = time * time - 4 * distance;
    let sqrt_d = (d as f64).sqrt() as usize;

    if sqrt_d * sqrt_d == d {
        sqrt_d - 1
    } else {
        sqrt_d + 1 - (time & 1).bitxor(sqrt_d & 1)
    }
}

fn parse_ints(line: &str) -> Vec<usize> {
    line.split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse().unwrap())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    fn slow(t: usize, d: usize) -> usize {
        (0..t).filter(|&x| x * (t - x) > d).count()
    }

    #[test]
    fn test_part_1() {
        let inputs = [(7, 9), (15, 40), (30, 200), (8, 12), (7, 10)];
        for (t, d) in inputs {
            let r = slow(t, d);
            assert_eq!(
                get_possible_ways_to_win(t, d),
                r,
                "invalid for {} and {}, expected {}",
                t,
                d,
                r
            );
        }
    }
    #[test]
    fn test_part_1_example() {
        assert_eq!(format!("{}", solve_part_1(INPUT)), "288");
    }

    #[test]
    fn test_fast() {
        for t in 0..50 {
            for d in t..150 {
                let expected = (0..t).filter(|&x| x * (t - x) > d).count();
                if expected == 0 {
                    break;
                }
                let actual = get_possible_ways_to_win(t, d);
                assert_eq!(actual, expected, "t: {}, d: {}", t, d)
            }
        }
    }

    #[test]
    fn test_part_1_actual() {
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "252000");
    }

    #[test]
    fn test_part_2() {
        assert_eq!(format!("{}", solve_part_2(INPUT)), "71503");
    }

    #[test]
    fn test_part_2_actual() {
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "36992486");
    }

    #[test]
    fn test_possible_ways_7_9() {
        let s = 7;
        let e = 9;
        assert_eq!(get_possible_ways_to_win(s, e), 4);
    }
    #[test]
    fn test_possible_ways_15_40() {
        let s = 15;
        let e = 40;
        assert_eq!(get_possible_ways_to_win(s, e), 8);
    }
    #[test]
    fn test_possible_ways_7530_940200() {
        let s = 71530;
        let e = 940200;
        assert_eq!(get_possible_ways_to_win(s, e), 71503);
    }
}
