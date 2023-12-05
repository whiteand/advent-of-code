#[derive(Debug, Clone)]
pub(super) struct RangeMap {
    pub(super) src: usize,
    length: usize,
    pub(super) dst: usize,
}

impl RangeMap {
    pub fn new(dst: usize, src: usize, length: usize) -> Self {
        Self { src, length, dst }
    }
    pub fn trivial(src: usize, length: usize) -> Self {
        Self {
            src,
            length,
            dst: src,
        }
    }

    #[inline]
    pub(super) fn contains_src(&self, number: usize) -> bool {
        self.src <= number && number < self.src + self.length
    }
    #[inline]
    pub(super) fn map(&self, src: usize) -> usize {
        self.dst + (src - self.src)
    }
    /// Exclusive end of the src range
    #[inline]
    pub(super) fn src_end(&self) -> usize {
        self.src + self.length
    }
}
