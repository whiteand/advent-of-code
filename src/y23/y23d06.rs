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

fn binary_search(from: usize, to: usize, from_predicate: impl Fn(usize) -> bool) -> usize {
    let mut l = from;
    let mut r = to;
    if from_predicate(r) {
        return r;
    }
    if !from_predicate(l) {
        return l;
    }
    while l.abs_diff(r) > 1 {
        let mid = (l + r) / 2;
        if from_predicate(mid) {
            l = mid;
        } else {
            r = mid;
        }
    }
    l
}

fn get_possible_ways_to_win(time: usize, distance: usize) -> usize {
    let is_record = |x| get_d(time, x) > distance;
    binary_search(time / 2, time, is_record) - binary_search(time / 2, 0, is_record) + 1
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
        for t in 0..50 {
            for d in t..150 {
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
