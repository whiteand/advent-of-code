fn solve<const N: usize>(file_content: &str) -> usize {
    // since the input is always ASCII characters - we use assumption that each character is written as single byte
    /* Invariant 1: cnt contains the count of each character inside the sequence of N chars we look at the moment  */
    /* Invariant 2: dublicates contains the number of dublicates in the current sequence of N chars */
    /* Invariant 3: current sequence has N last characters of the input */
    let chars = file_content.as_bytes();
    let mut cnt = [0usize; 256];
    let mut dublicates = 0;
    for &c in &chars[..N] {
        cnt[c as usize] += 1;
        if cnt[c as usize] == 2 {
            dublicates += 1;
        }
    }
    if dublicates <= 0 {
        return N;
    }

    for (i, &x) in chars[N..].iter().enumerate() {
        // moving to next window

        let goes_outside_c = chars[i] as usize;
        cnt[goes_outside_c] -= 1;
        if cnt[goes_outside_c] == 1 {
            dublicates -= 1;
        }

        let c = x as usize;
        cnt[c] += 1;
        if cnt[c] == 2 {
            dublicates += 1;
        }
        // at this point all invariants are preserved

        if dublicates == 0 {
            return i + N + 1;
        }
    }
    0
}
pub fn solve_task1(file_content: &str) -> usize {
    solve::<4>(file_content)
}
pub fn solve_task2(file_content: &str) -> impl std::fmt::Display {
    solve::<14>(file_content)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1("abcd")), "4");
        assert_eq!(
            format!("{}", solve_task1("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            "5"
        );
        assert_eq!(
            format!("{}", solve_task1("nppdvjthqldpwncqszvftbrmjlhg")),
            "6"
        );
        assert_eq!(
            format!("{}", solve_task1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            "10"
        );
        assert_eq!(
            format!("{}", solve_task1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            "11"
        );
    }

    #[test]
    #[ignore]
    fn test_task2() {
        assert_eq!(
            format!("{}", solve_task2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")),
            "19"
        );
        assert_eq!(
            format!("{}", solve_task2("bvwbjplbgvbhsrlpgdmjqwftvncz")),
            "23"
        );
        assert_eq!(
            format!("{}", solve_task2("nppdvjthqldpwncqszvftbrmjlhg")),
            "23"
        );
        assert_eq!(
            format!("{}", solve_task2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")),
            "29"
        );
        assert_eq!(
            format!("{}", solve_task2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")),
            "26"
        );
    }
}
