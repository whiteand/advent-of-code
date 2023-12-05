use super::range_map::RangeMap;
use std::ops::Range;

#[derive(Debug)]
pub(super) struct IntMap {
    // sorted in src ascending order
    ranges: Vec<RangeMap>,
}

fn populate_default_ranges(mut ranges: Vec<RangeMap>) -> Vec<RangeMap> {
    ranges.sort_by_key(|r| r.src);
    let mut res = Vec::new();
    let mut number_ptr = 0;
    let mut range_ptr = 0;
    loop {
        if range_ptr >= ranges.len() {
            res.push(RangeMap::trivial(number_ptr, usize::MAX - number_ptr));
            break;
        }
        let present_range = &ranges[range_ptr];
        if number_ptr < present_range.src {
            res.push(RangeMap::trivial(
                number_ptr,
                present_range.src - number_ptr,
            ));
            number_ptr = present_range.src;
            continue;
        }

        res.push(present_range.clone());
        number_ptr = present_range.src_end();
        range_ptr += 1;
    }
    res
}

impl IntMap {
    pub(super) fn new(ranges: Vec<RangeMap>) -> Self {
        Self {
            ranges: populate_default_ranges(ranges),
        }
    }

    pub(super) fn get(&self, src: usize) -> usize {
        self.ranges
            .iter()
            .find(|r| r.contains_src(src))
            .expect("no range found")
            .map(src)
    }

    pub(super) fn map_range(
        &self,
        range: &Range<usize>,
    ) -> impl Iterator<Item = Range<usize>> + '_ {
        let mut current_map_range_index = self
            .ranges
            .iter()
            .position(|r| r.contains_src(range.start))
            .unwrap_or_default();

        let mut ptr = range.start;
        let end = range.end;

        std::iter::from_fn(move || {
            while ptr < end {
                let present_range = &self.ranges[current_map_range_index];
                current_map_range_index += 1;
                if present_range.contains_src(ptr) {
                    let next_ptr = present_range.src_end().min(end);
                    let dst_start = present_range.map(ptr);
                    let dst_end = present_range.map(next_ptr - 1) + 1;
                    ptr = next_ptr;
                    return Some(dst_start..dst_end);
                }
            }
            None
        })
    }
}
