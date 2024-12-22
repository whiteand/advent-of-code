#[tracing::instrument(skip(file_content))]
pub fn part1(file_content: &str) -> String {
    solve::<272>(file_content)
}

fn solve<const N: usize>(file_content: &str) -> String {
    let mut file = Vec::with_capacity(N * 2);
    file.extend_from_slice(file_content.trim().as_bytes());
    while file.len() < N {
        step(&mut file);
    }

    checksum::<N>(file)
}

fn checksum<const N: usize>(mut file: Vec<u8>) -> String {
    file.truncate(N);

    while file.len() % 2 == 0 {
        checksum_step(&mut file);
    }

    unsafe { String::from_utf8_unchecked(file) }
}

fn checksum_step(file: &mut Vec<u8>) {
    let mut src = file.as_mut_ptr();
    let mut dst = src;
    let end = unsafe { src.add(file.len()) };
    while src != end {
        unsafe {
            let a = *src;
            src = src.add(1);
            let b = *src;
            src = src.add(1);
            *dst = if a == b { b'1' } else { b'0' };
            dst = dst.add(1)
        }
    }
    file.truncate(file.len() >> 1);
}

/// Call the data you have at this point "a".  
/// Make a copy of "a"; call this copy "b".  
/// Reverse the order of the characters in "b".  
/// In "b", replace all instances of 0 with 1 and all 1s with 0.  
/// The resulting data is "a", then a single 0, then "b".  
fn step(target: &mut Vec<u8>) {
    let len = target.len();
    target.push(b'0');
    target.extend_from_within(0..len);
    let b_range = (len + 1)..((len << 1) + 1);
    for x in &mut target[b_range.clone()] {
        *x = b'1' - *x + b'0';
    }
    target[b_range].reverse();
}
#[tracing::instrument(skip(file_content))]
pub fn part2(file_content: &str) -> String {
    solve::<35651584>(file_content)
}

#[cfg(test)]
mod tests {
    use super::part1;
    use rstest::rstest;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

    #[rstest]
    #[case(b"1", b"100")]
    #[case(b"0", b"001")]
    #[case(b"11111", b"11111000000")]
    #[case(b"111100001010", b"1111000010100101011110000")]
    fn test_step(#[case] input: &[u8], #[case] expected: &[u8]) {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );

        let mut res = Vec::new();
        res.extend_from_slice(input);
        super::step(&mut res);
        assert_eq!(res.as_slice(), expected);
    }

    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", super::solve::<20>(EXAMPLE)), "01100");
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", part1(ACTUAL)), "10010010110011010");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(
            format!("{}", super::solve::<35651584>(ACTUAL)),
            "01010100101011100"
        );
    }
}
