use std::fmt::Write;

use itertools::Itertools;

#[tracing::instrument(skip(file_content))]
pub fn solve_part_1(file_content: &str) -> usize {
    solve(file_content, checksum)
}
pub fn solve(file_content: &str, get_checksum: impl FnOnce(&[usize]) -> usize) -> usize {
    let map = file_content
        .trim()
        .bytes()
        .map(|x| (x - b'0') as usize)
        .collect_vec();
    return get_checksum(&map);
}

fn checksum(map: &[usize]) -> usize {
    let mut left = 0;
    let mut empty = 1;
    let mut right = map.len() - 1;
    let mut ind = 0;

    let mut total = 0;

    let mut left_n = map[left];
    let mut empty_n = map[empty];
    let mut right_n = map[right];
    let mut res = Vec::new();
    let res_len: usize = map.into_iter().step_by(2).sum();

    'check: loop {
        while left_n > 0 {
            if ind >= res_len {
                break 'check;
            }
            total += (left / 2) * ind;
            ind += 1;
            left_n -= 1;
            res.push(left / 2);
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
                res.push(right / 2);
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
#[derive(Debug, Clone, Copy)]
enum Segment {
    Void(usize),
    File { len: usize, id: usize },
}
impl Segment {
    fn is_void(&self) -> bool {
        return matches!(self, Segment::Void(_));
    }
    fn resize(&mut self, new_len: usize) {
        match self {
            Segment::Void(x) => *x = new_len,
            Segment::File { len, .. } => *len = new_len,
        }
    }
    fn len(&self) -> usize {
        match self {
            Segment::Void(len) => *len,
            Segment::File { len, .. } => *len,
        }
    }
    fn id(&self) -> Option<usize> {
        match self {
            Segment::Void(_) => None,
            Segment::File { id, .. } => Some(*id),
        }
    }
    fn iter(&self) -> impl Iterator<Item = Option<usize>> {
        match self {
            Segment::Void(len) => std::iter::repeat_n(None, *len),
            Segment::File { len, id } => std::iter::repeat_n(Some(*id), *len),
        }
    }

    fn has_id(&self, id: usize) -> bool {
        self.id().map_or(false, |x| x == id)
    }
}

impl std::fmt::Display for Segment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Segment::Void(len) => {
                for _ in 0..*len {
                    f.write_char('.')?;
                }
            }
            Segment::File { len, id } => {
                write!(f, "[{id};{len}]")?;
            }
        }
        Ok(())
    }
}

fn checksum2(map: &[usize]) -> usize {
    let mut segments = get_segments(map);

    let mut to_move = segments.iter().flat_map(|x| x.id()).collect_vec();
    while let Some(id) = to_move.pop() {
        let Some(src) = segments.iter().position(|x| x.has_id(id)) else {
            unreachable!("failed to find a file with id: {id}");
        };
        try_move_file(&mut segments, src);
    }

    segments
        .into_iter()
        .flat_map(|x| x.iter())
        .enumerate()
        .filter_map(|(i, id)| id.map(move |id| (i, id)))
        .map(|(a, b)| a * b)
        .sum()
}
fn try_move_file(segments: &mut Vec<Segment>, src: usize) {
    let file_len = segments[src].len();
    let Some(dst) = (0..src).find(|x| segments[*x].is_void() && segments[*x].len() >= file_len)
    else {
        return;
    };

    move_file(segments, src, dst);
}

fn move_file(segments: &mut Vec<Segment>, src: usize, dst: usize) {
    debug_assert!(src > dst);
    debug_assert!(segments[dst].is_void());
    let right_void_len = if src + 1 < segments.len() && segments[src + 1].is_void() {
        debug_assert!(segments[src + 1].is_void());
        segments[src + 1].len()
    } else {
        0
    };
    let left_void_len = if src > 0 && segments[src - 1].is_void() {
        segments[src - 1].len()
    } else {
        0
    };
    let new_void_len = segments[src].len() + right_void_len + left_void_len;
    let voids_range = match (left_void_len > 0, right_void_len > 0) {
        (true, true) => (src - 1)..(src + 2),
        (true, false) => (src - 1)..(src + 1),
        (false, true) => (src)..(src + 2),
        (false, false) => (src)..(src + 1),
    };
    let file = segments[src];
    if voids_range.end == segments.len() {
        segments.drain(voids_range.clone());
    } else {
        segments.drain((voids_range.start + 1)..voids_range.end);
        let _ = std::mem::replace(
            &mut segments[voids_range.start],
            Segment::Void(new_void_len),
        );
    }
    let new_dst_len = segments[dst].len() - file.len();
    segments[dst].resize(new_dst_len);
    segments.insert(dst, file);
}

fn get_segments(map: &[usize]) -> Vec<Segment> {
    let mut entities = Vec::new();
    let mut i = 0;
    let mut is_empty = false;
    let mut id = 0;
    while i < map.len() {
        if is_empty {
            if map[i] == 0 {
                i += 1;
                is_empty = false;
                continue;
            }
            entities.push(Segment::Void(map[i]));
            i += 1;
            is_empty = false;
        } else {
            debug_assert_ne!(map[i], 0, "file should not be zero len");
            entities.push(Segment::File { len: map[i], id });
            i += 1;
            id += 1;
            is_empty = true;
        }
    }
    entities
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
}
