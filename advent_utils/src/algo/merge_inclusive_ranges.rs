use std::ops::RangeInclusive;

pub struct MergeInclusiveRangesIter<'t, T> {
    src: &'t mut [RangeInclusive<T>],
    len: usize,
}

impl<'t, T> MergeInclusiveRangesIter<'t, T> {
    fn new(src: &'t mut [RangeInclusive<T>]) -> Self
    where
        T: TempStep + Ord,
    {
        // After sorting src has this order (smaller-start-last)
        // src[0]:                       ##### (23-27)
        // src[1]:                       ### (23-25)
        // src[2]:                  ### (18-20)
        // src[3]:                ### (16-18)
        // src[4]:           ### (11-13)
        // src[5]:     ##### (5-9)
        // src[6]:   ### (3-5)
        // src[7]: ### (1-3)

        src.sort_by(|a, b| b.start().cmp(a.start()));

        Self {
            len: src.len(),
            src,
        }
    }

    fn take_last(&mut self) -> Option<&mut RangeInclusive<T>> {
        if self.len == 0 {
            return None;
        }
        let x = &mut self.src[self.len - 1];

        self.len -= 1;

        Some(x)
    }

    fn truncate(&mut self, new_len: usize) {
        self.len = self.len.min(new_len)
    }

    fn iter_remaining_ranges(
        &self,
    ) -> impl ExactSizeIterator<Item = &RangeInclusive<T>> + DoubleEndedIterator {
        self.src[..self.len].iter()
    }
}

impl<'t, T: TempStep + Ord> Iterator for MergeInclusiveRangesIter<'t, T> {
    type Item = RangeInclusive<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let first_range = self.take_last()?;

        // Finding the largest max which would merge first ranges
        // If remaining ranges are:
        // ranges[0]                         ##### <-\
        // ranges[1]                         ###.    |
        // ranges[2]                    ###.         +---- these ranges will remain
        // ranges[3]                  ###            |
        // ranges[4]             ### <---------------/
        // ranges[5]     ## <----------------------------- last merged range
        // ranges[6]   #######
        // ranges[7] ### <-------------------------------- first_range

        let min = first_range.start().clone();
        let mut max = first_range.end().clone();
        let mut last_merged_index = None;

        for (i, r) in self.iter_remaining_ranges().enumerate().rev() {
            let r_start = r.start();
            if r_start.gt(&max) {
                let (between, _) = T::temp_steps_between(r_start, &max);
                if between > 1 {
                    break;
                }
            }
            max = if r.end() > &max { r.end().clone() } else { max };
            last_merged_index = Some(i);
        }

        let res = Some(min.clone()..=max.clone());
        let new_len = last_merged_index.unwrap_or(self.src.len());
        self.truncate(new_len);
        res
    }
}

pub trait TempStep: Clone + PartialOrd + Sized {
    fn temp_steps_between(start: &Self, end: &Self) -> (usize, Option<usize>);
}

macro_rules! impl_via_abs_diff {
    ($($t:ty),*) => {
        $(
            impl TempStep for $t {
                fn temp_steps_between(a: &Self, other: &Self) -> (usize, Option<usize>) {
                    let diff = (*a).abs_diff(*other) as usize;
                    (diff, Some(diff ))
                }
            }
        )*
    }
}

impl_via_abs_diff! {
    u8, i8, u16, i16, u32, i32, u64, i64, usize, isize
}

/**
 * Rearranges the ranges in the src.
 * Then iterates over all "merged" ranges
 */
pub fn merge_inclusive_ranges<'t, T: TempStep + Ord>(
    src: &'t mut [RangeInclusive<T>],
) -> MergeInclusiveRangesIter<'t, T> {
    MergeInclusiveRangesIter::new(src)
}
