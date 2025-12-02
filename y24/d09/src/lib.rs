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
    get_checksum(&map)
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
    let res_len: usize = map.iter().step_by(2).sum();

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
    let mut disc = Vec::from_iter(std::iter::repeat_n(0, map.iter().copied().sum()));
    let mut i = 0;
    let mut id = 0;
    let mut files = Vec::with_capacity(map.len());
    const SIZES: usize = 10;
    let default_cap = (disc.len() / SIZES / 9).max(5);
    tracing::info!(?default_cap, len = disc.len());
    let mut voides_per_size: [Vec<usize>; SIZES] = [
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
        Vec::with_capacity(map.len() / 2),
    ];
    for (place, x) in map.iter().copied().enumerate() {
        if place % 2 == 0 {
            disc[i..(i + x)].fill(id);
            files.push((i, x));
            i += x;
            id += 1;
        } else {
            voides_per_size[x].push(i);
            i += x;
        }
    }
    for void_ptr in &mut voides_per_size {
        void_ptr.reverse();
    }
    tracing::info!(emptiness=?voides_per_size.iter().map(|x| x.len()).collect_vec());

    'file: while let Some((src, file_len)) = files.pop() {
        let Some(void_len) = (file_len..voides_per_size.len())
            .filter(|n| !voides_per_size[*n].is_empty())
            .map(|void_len| (void_len, voides_per_size[void_len].last().copied().unwrap()))
            .filter(|(_, ptr)| *ptr < src)
            .min_by_key(|(_, n)| *n)
            .map(|(x, _)| x)
        else {
            continue;
        };
        let dst_ptr = voides_per_size[void_len].pop().unwrap();

        debug_assert!(dst_ptr + file_len <= src, "{dst_ptr} + {file_len} <= {src}");
        disc.copy_within(src..(src + file_len), dst_ptr);
        disc[src..(src + file_len)].fill(0);
        let new_len = void_len - file_len;
        if new_len == 0 {
            continue 'file;
        }
        let target = &mut voides_per_size[new_len];
        let new_ptr = dst_ptr + file_len;
        target.push(new_ptr);
        let mut i = target.len() - 1;
        while i > 0 && target[i - 1] < new_ptr {
            target.swap(i, i - 1);
            i -= 1
        }
        debug_assert!(target.is_sorted_by(|a, b| a > b));
    }

    disc.into_iter().enumerate().map(|(i, x)| i * x).sum()
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
