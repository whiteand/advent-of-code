use super::int_map::IntMap;

#[derive(Debug)]
pub(super) struct MapPipeline {
    pub(super) maps: Vec<IntMap>,
}

impl MapPipeline {
    pub(super) fn new(maps: Vec<IntMap>) -> Self {
        Self { maps }
    }

    pub(super) fn levels(&self) -> usize {
        self.maps.len()
    }

    pub(super) fn get_src(&self, dst: usize, applied_maps_number: usize) -> usize {
        let mut value = dst;
        for m in self.maps.iter().take(applied_maps_number).rev() {
            value = m.reverse_get(value);
        }
        value
    }

    pub(super) fn get(&self, seed: usize) -> usize {
        self.maps.iter().fold(seed, |v, m| m.get(v))
    }

    pub(super) fn key_points(&self, map_index: usize) -> impl Iterator<Item = usize> + '_ {
        self.maps[map_index].key_points()
    }

    pub(super) fn get_from_level(&self, src: usize, already_applied: usize) -> usize {
        self.maps[already_applied..]
            .iter()
            .fold(src, |v, m| m.get(v))
    }
}
