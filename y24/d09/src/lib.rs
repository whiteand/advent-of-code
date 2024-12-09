use std::fmt::Write;

use itertools::Itertools;
pub mod arr_part2;
mod list;

use list::{List, NodeIndex};

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
struct File {
    len: usize,
}
impl Segment {
    pub fn is_void(&self) -> bool {
        return matches!(self, Segment::Void(_));
    }
    pub fn resize(&mut self, new_len: usize) {
        match self {
            Segment::Void(x) => *x = new_len,
            Segment::File { len, .. } => *len = new_len,
        }
    }
    pub fn id(&self) -> Option<usize> {
        match self {
            Segment::Void(_) => None,
            Segment::File { id, .. } => Some(*id),
        }
    }
    pub fn len(&self) -> usize {
        match self {
            Segment::Void(len) => *len,
            Segment::File { len, .. } => *len,
        }
    }

    pub fn checksum(&self, pos: usize) -> usize {
        match self {
            Segment::Void(_) => 0,
            Segment::File { len, id } => {
                return pos * len * id + len * (len - 1) / 2 * id;
            }
        }
    }
    pub fn as_file(&self) -> Option<File> {
        match self {
            Segment::Void(_) => None,
            Segment::File { len, .. } => Some(File { len: *len }),
        }
    }
    pub fn as_void(&self) -> Option<usize> {
        match self {
            Segment::Void(len) => Some(*len),
            Segment::File { .. } => None,
        }
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
    let segments_vec = get_segments(map);

    let mut to_move = Vec::new();

    let mut segments = list::List::new();
    for segment in segments_vec {
        match segment {
            Segment::Void(x) => {
                segments.push_back(Segment::Void(x));
            }
            Segment::File { len, id } => {
                to_move.push(segments.push_back(Segment::File { len, id }));
            }
        }
    }

    'file: while let Some(file_node_index) = to_move.pop() {
        let file_node = segments.get(file_node_index).unwrap();
        let file_len = file_node.value.len();
        let mut head = segments.first().unwrap();
        while head != file_node_index {
            let Some(head_node) = segments.get(head) else {
                break;
            };
            let head_next = head_node.next;

            match head_node.value.as_void() {
                Some(void_len) if void_len >= file_len => {
                    move_file(&mut segments, file_node_index, head);
                    continue 'file;
                }
                _ => {
                    head = match head_next {
                        Some(x) => x,
                        _ => {
                            break;
                        }
                    };
                }
            }
        }
    }

    checksum_from_segments(segments.iter())
}

pub fn checksum_from_segments<'a>(mut segments: impl Iterator<Item = &'a Segment>) -> usize {
    let mut pos = 0;
    let mut total = 0;
    while let Some(seg) = segments.next() {
        total += seg.checksum(pos);
        pos += seg.len();
    }
    total
}

#[tracing::instrument(skip(segments))]
fn remove_file(segments: &mut List<Segment>, src: NodeIndex) -> File {
    let file = segments.get(src).and_then(|x| x.value.as_file()).unwrap();
    let prev_void_idx = segments
        .get(src)
        .and_then(|x| x.prev)
        .filter(|index| segments.get(*index).map_or(false, |x| x.value.is_void()));
    let next_void_idx = segments
        .get(src)
        .and_then(|x| x.next)
        .filter(|index| segments.get(*index).map_or(false, |x| x.value.is_void()));
    match (prev_void_idx, next_void_idx) {
        (Some(prev_void_idx), Some(next_void_idx)) => {
            // we need to remove file node and next void node
            segments.unlink(next_void_idx);
            segments.unlink(src);
            let next_void_len = segments.get(next_void_idx).map(|x| x.value.len()).unwrap();
            if let Some(x) = segments.get_mut(prev_void_idx).map(|x| &mut x.value) {
                x.resize(x.len() + file.len + next_void_len);
            }
        }
        (Some(prev_void_idx), None) => {
            // we need to remove file node and next void node
            segments.unlink(src);

            if let Some(x) = segments.get_mut(prev_void_idx).map(|x| &mut x.value) {
                x.resize(x.len() + file.len);
            }
        }
        (None, None) => {
            // we need just to remove file
            let next = segments.get(src).and_then(|x| x.next);

            match next {
                Some(next) => {
                    let idx = segments.push_back(Segment::Void(file.len));
                    segments.unlink(idx);
                    segments.unlink(src);
                    segments.insert_node_before(next, idx);
                }
                None => {
                    segments.unlink(src);
                }
            }
        }
        (None, Some(next_void_idx)) => {
            segments.unlink(src);
            let next_void = segments
                .get_mut(next_void_idx)
                .map(|x| &mut x.value)
                .unwrap();
            next_void.resize(next_void.len() + file.len);
        }
    }
    file
}
#[tracing::instrument(skip(segments))]
fn move_file(segments: &mut List<Segment>, src: NodeIndex, dst: NodeIndex) {
    let file = remove_file(segments, src);
    let dst_void = &mut segments.get_mut(dst).unwrap().value;
    let dst_void_len = dst_void.len();
    dst_void.resize(dst_void_len - file.len);
    segments.insert_node_before(dst, src);
    if dst_void_len == file.len {
        segments.unlink(dst);
    }
}

#[tracing::instrument(skip(map))]
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
