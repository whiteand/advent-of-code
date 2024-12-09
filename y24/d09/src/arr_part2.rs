use itertools::Itertools;

use crate::{checksum_from_segments, get_segments, solve, Segment};

#[tracing::instrument(skip(map))]
pub fn checksum2(map: &[usize]) -> usize {
    let mut segments = get_segments(map);

    let mut to_move = segments.iter().flat_map(|x| x.id()).collect_vec();
    while let Some(id) = to_move.pop() {
        let Some(src) = segments.iter().position(|x| x.id() == Some(id)) else {
            unreachable!("failed to find a file with id: {id}");
        };
        try_move_file(&mut segments, src);
    }

    checksum_from_segments(segments.iter())
}
#[tracing::instrument(skip(segments))]
fn try_move_file(segments: &mut Vec<Segment>, src: usize) {
    let file_len = segments[src].len();
    let Some(dst) = (0..src).find(|x| segments[*x].is_void() && segments[*x].len() >= file_len)
    else {
        return;
    };

    move_file(segments, src, dst);
}

#[tracing::instrument(skip(segments))]
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

#[tracing::instrument(skip(file_content))]
pub fn solve_part_2(file_content: &str) -> usize {
    solve(file_content, checksum2)
}

#[cfg(test)]
mod tests {
    use super::solve_part_2;
    const EXAMPLE: &str = include_str!("../example.txt");
    const ACTUAL: &str = include_str!("../input.txt");

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
