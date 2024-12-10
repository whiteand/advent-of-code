use super::range_map::RangeMap;

#[derive(Debug)]
pub(super) struct IntMap {
    // sorted in src ascending order
    ranges: Vec<RangeMap>,
}

impl IntMap {
    pub(super) fn new(mut ranges: Vec<RangeMap>) -> Self {
        ranges.sort_by_key(|r| r.src);
        Self { ranges }
    }

    pub(super) fn get(&self, src: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_src(src)) {
            Some(r) => r.map(src),
            None => src,
        }
    }

    pub(super) fn key_points(&self) -> impl Iterator<Item = usize> + '_ {
        std::iter::once(0)
            .chain(self.ranges.iter().flat_map(|r| [r.src, r.src + r.length]))
            .chain(std::iter::once(usize::MAX))
    }

    pub(super) fn reverse_get(&self, dst: usize) -> usize {
        match self.ranges.iter().find(|r| r.contains_dst(dst)) {
            Some(r) => r.reverse_map(dst),
            None => dst,
        }
    }
}
