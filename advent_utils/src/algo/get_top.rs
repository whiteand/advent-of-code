use std::{cmp::Ordering, mem::MaybeUninit};

use crate::algo::heapify::{heap_pop_by, heap_raise_by};

/// # SAFETY
/// - array is fully initialized
pub unsafe fn array_assume_init<const N: usize, T>(array: [MaybeUninit<T>; N]) -> [T; N] {
    array.map(|x| x.assume_init())
}

pub trait IntoTopArrayIteratorExt: Iterator {
    fn into_top_array<const N: usize>(self) -> Option<[Self::Item; N]>;
    fn into_top_array_by<const N: usize, F>(self, cmp: F) -> Option<[<Self as Iterator>::Item; N]>
    where
        F: Fn(&<Self as Iterator>::Item, &<Self as Iterator>::Item) -> Ordering;
}

impl<It> IntoTopArrayIteratorExt for It
where
    It: Iterator,
    It::Item: Copy + Ord,
{
    /// Consumes iterator and returns the
    /// array of top N elements (in descending order)
    fn into_top_array<const N: usize>(self) -> Option<[<Self as Iterator>::Item; N]> {
        self.into_top_array_by(Ord::cmp)
    }
    /// Consumes iterator and returns the
    /// array of top N elements (in descending order)
    fn into_top_array_by<const N: usize, F>(
        self,
        mut cmp: F,
    ) -> Option<[<Self as Iterator>::Item; N]>
    where
        F: FnMut(&<Self as Iterator>::Item, &<Self as Iterator>::Item) -> Ordering,
    {
        let (lo, _) = self.size_hint();
        let mut heap = Vec::with_capacity(lo);
        for x in self {
            let n = heap.len();
            heap.push(x);
            heap_raise_by(&mut heap, &mut cmp, n);
        }

        if heap.len() < N {
            return None;
        }

        let mut res = const { [MaybeUninit::uninit(); N] };
        for x in res.iter_mut() {
            x.write(heap_pop_by(&mut heap, &mut cmp).expect("should be present"));
        }
        let res = unsafe { array_assume_init(res) };
        Some(res)
    }
}
