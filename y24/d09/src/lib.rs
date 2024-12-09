use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, checksum)
}
#[tracing::instrument(skip(file_content, get_checksum))]
pub fn solve(file_content: &str, get_checksum: impl FnOnce(&[usize]) -> usize) -> usize {
    let map = file_content
        .trim()
        .bytes()
        .map(|x| (x - b'0') as usize)
        .collect_vec();
    return get_checksum(&map);
}

#[tracing::instrument(skip(map))]
fn checksum(map: &[usize]) -> usize {
    let mut left = 0;
    let mut empty = 1;
    let mut right = map.len() - 1;
    let mut ind = 0;

    let mut total = 0;

    let mut left_n = map[left];
    let mut empty_n = map[empty];
    let mut right_n = map[right];
    let res_len: usize = map.into_iter().step_by(2).sum();

    'check: loop {
        while left_n > 0 {
            if ind >= res_len {
                break 'check;
            }
            total += (left / 2) * ind;
            ind += 1;
            left_n -= 1;
        }
        left += 2;
        if left >= map.len() {
            break;
        }
        left_n = map[left];
        while empty_n > 0 {
            if ind >= res_len {
                break 'check;
            }
            if right_n > 0 {
                right_n -= 1;
                total += (right / 2) * ind;
                ind += 1;
                empty_n -= 1;
            } else {
                right -= 2;
                right_n = map[right];
            }
        }
        empty += 2;
        if empty >= map.len() {
            break;
        }
        empty_n = map[empty];
    }
    total
}

fn checksum2(map: &[usize]) -> usize {
    let mut disc = Vec::from_iter(std::iter::repeat_n(None, map.iter().copied().sum()));
    let mut i = 0;
    let mut id = 0;
    let mut files = Vec::with_capacity(map.len());
    let mut empties = Vec::with_capacity(map.len());
    for (place, x) in map.iter().copied().enumerate() {
        if place % 2 == 0 {
            disc[i..(i + x)].fill(Some(id));
            files.push((i, x));
            i += x;
            id += 1;
        } else {
            empties.push((i, x));
            i += x;
        }
    }
    'file: while let Some((src, file_len)) = files.pop() {
        for (dst, void_len) in &mut empties {
            let dst_ptr = *dst;

            if dst_ptr >= src {
                break;
            }

            if *void_len >= file_len {
                disc.copy_within(src..(src + file_len), dst_ptr);
                disc[src..(src + file_len)].fill(None);
                *dst += file_len;
                *void_len -= file_len;
                continue 'file;
            }
        }
    }

    disc.into_iter()
        .enumerate()
        .filter_map(|(i, x)| x.map(move |x| i * x))
        .sum()
}

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, checksum2)
}

#[cfg(test)]
mod tests {
    use super::{solve_part_1, solve_part_2};
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");
    #[test]
    fn test_part1() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(solve_part_1(EXAMPLE), 1928);
    }

    #[test]
    fn test_part1_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_1(ACTUAL)), "6301895872542");
    }

    #[test]
    fn test_part2() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(EXAMPLE)), "2858");
    }

    #[test]
    fn test_part2_actual() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::FmtSubscriber::builder()
                .without_time()
                .finish(),
        );
        assert_eq!(format!("{}", solve_part_2(ACTUAL)), "6323761685944");
    }

    #[test]
    fn test_sum() {
        const POS: usize = 2;
        const LEN: usize = 3;
        const ID: usize = 5;
        let expected: usize = (POS..(POS + LEN)).map(|x| x * ID).sum();
        let actual = POS * LEN * ID + LEN * (LEN - 1) / 2 * ID;
        assert_eq!(expected, actual);
    }
}
