use std::{
    cmp::Ordering,
    ops::{Add, Div, Sub},
};

/// Searches for a solution between left and right
/// Returns Ok(P) when Equal ordering is returned
/// Returns Err(P) when search is finished and
/// P is the minimal value which returns Greater
pub fn binary_search<P>(mut left: P, mut right: P, mut f: impl FnMut(P) -> Ordering) -> Result<P, P>
where
    P: Add<P, Output = P> + Sub<P, Output = P> + Div<P, Output = P> + Copy + From<u8> + PartialOrd,
{
    while left + P::from(1u8) < right {
        let mid = left + (right - left) / P::from(2u8);
        match f(mid) {
            Ordering::Less => left = mid,
            Ordering::Equal => return Ok(mid),
            Ordering::Greater => right = mid,
        }
    }
    Err(right)
}
