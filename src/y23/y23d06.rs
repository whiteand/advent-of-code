fn parse1(file_content: &str) -> (Vec<usize>, Vec<usize>) {
    let mut line_it = file_content.lines();
    let times = line_it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let distances = line_it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (times, distances)
}

fn parse2(file_content: &str) -> (usize, usize) {
    let mut line_it = file_content.lines();
    let time = line_it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let distance = line_it
        .next()
        .unwrap()
        .split_ascii_whitespace()
        .skip(1)
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    (time, distance)
}

#[inline]
fn get_d(time: usize, holding_time: usize) -> usize {
    (time - holding_time) * holding_time
}

fn int_sqrt(n: usize) -> usize {
    let mut left = 0;
    let mut right = n / 2;
    while left + 1 < right {
        let mid = (left + right) / 2;
        if mid <= n / mid {
            left = mid;
        } else {
            right = mid;
        }
    }
    left
}
fn get_end(time: usize, distance: usize) -> usize {
    let d = time * time - 4 * distance;
    let res = (time + int_sqrt(d)) / 2;
    if get_d(time, res) > distance && get_d(time, res + 1) <= distance {
        return res;
    }
    if get_d(time, res) <= distance && get_d(time, res - 1) > distance {
        return res - 1;
    }
    unreachable!();
}

fn get_start(time: usize, distance: usize) -> usize {
    let d = time * time - 4 * distance;
    let res = (time - int_sqrt(d)) / 2;
    if get_d(time, res) > distance && get_d(time, res - 1) <= distance {
        return res;
    }
    if get_d(time, res + 1) > distance && get_d(time, res) <= distance {
        return res + 1;
    }
    unreachable!();
}

fn get_possible_ways_to_win(time: usize, distance: usize) -> usize {
    get_end(time, distance) - get_start(time, distance) + 1
}

pub fn solve_task1(file_content: &str) -> usize {
    let (times, distances) = parse1(file_content);
    times
        .into_iter()
        .enumerate()
        .map(|(game_ind, time)| get_possible_ways_to_win(time, distances[game_ind]))
        .product()
}
pub fn solve_task2(file_content: &str) -> usize {
    let (time, distance) = parse2(file_content);
    get_possible_ways_to_win(time, distance)
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d06/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d06.txt");

    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "288");
    }

    #[test]
    fn test_fast() {
        for t in 0..500 {
            for d in 0..1500 {
                let expected = (0..t).filter(|x| get_d(t, *x) > d).count();
                if expected == 0 {
                    break;
                }
                let actual = get_possible_ways_to_win(t, d);
                assert_eq!(actual, expected, "t: {}, d: {}", t, d)
            }
        }
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "252000");
    }

    #[test]
    fn test_task2() {
        assert_eq!(format!("{}", solve_task2(INPUT)), "71503");
    }

    #[test]
    fn test_task2_actual() {
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "36992486");
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
