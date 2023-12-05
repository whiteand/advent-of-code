#[derive(Debug, Clone)]
pub(super) struct RangeMap {
    pub(super) src: usize,
    pub(super) length: usize,
    pub(super) dst: usize,
}

impl RangeMap {
    pub fn new(dst: usize, src: usize, length: usize) -> Self {
        Self { src, length, dst }
    }

    #[inline]
    pub(super) fn contains_src(&self, src: usize) -> bool {
        self.src <= src && src < self.src + self.length
    }
    #[inline]
    pub(super) fn contains_dst(&self, dst: usize) -> bool {
        self.dst <= dst && dst < self.dst + self.length
    }

    #[inline]
    pub(super) fn map(&self, src: usize) -> usize {
        self.dst + src - self.src
    }

    #[inline]
    pub(super) fn reverse_map(&self, dst: usize) -> usize {
        self.src + dst - self.dst
    }
}
