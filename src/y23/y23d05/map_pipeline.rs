use std::ops::Range;

use super::int_map::IntMap;

#[derive(Debug)]
pub(super) struct MapPipeline {
    maps: Vec<IntMap>,
}

impl MapPipeline {
    pub(super) fn new(maps: Vec<IntMap>) -> Self {
        Self { maps }
    }

    pub(super) fn get(&self, seed: usize) -> usize {
        let mut value = seed;
        for m in &self.maps {
            value = m.get(value);
        }
        value
    }
    pub(super) fn map_range(
        &self,
        seed_range: Range<usize>,
    ) -> impl Iterator<Item = Range<usize>> + '_ {
        let mut src = Vec::new();
        let mut dst = vec![seed_range];
        for m in &self.maps {
            (src, dst) = (dst, src);
            dst.clear();
            for r in &src {
                dst.extend(m.map_range(r))
            }
        }
        dst.into_iter()
    }
}
