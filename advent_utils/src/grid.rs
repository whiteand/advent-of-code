use std::ops::Range;

use glam::IVec2;
use itertools::Itertools;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Grid<T> {
    arr: Vec<T>,
    row_start_indexes: Vec<usize>,
}

impl<T: std::fmt::Debug> std::fmt::Debug for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "[")?;
        for row in self.rows() {
            writeln!(f, " {:?}", row)?;
        }
        writeln!(f, "]")
    }
}

#[derive(Clone)]
struct RowsRangesIter<'t> {
    row_start_indexes: &'t [usize],
    arr_len: usize,
}

impl<'t> Iterator for RowsRangesIter<'t> {
    type Item = Range<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let (a, b) = self.row_start_indexes.split_first()?;
        self.row_start_indexes = b;
        Some(*a..b.first().copied().unwrap_or(self.arr_len))
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        let (_, suffix) = self.row_start_indexes.split_at_checked(n)?;
        self.row_start_indexes = suffix;
        self.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<'t> DoubleEndedIterator for RowsRangesIter<'t> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (last, init) = self.row_start_indexes.split_last()?;
        self.row_start_indexes = init;
        let len = self.arr_len;
        self.arr_len = *last;
        Some(*last..len)
    }
}

impl<'t> ExactSizeIterator for RowsRangesIter<'t> {
    fn len(&self) -> usize {
        self.row_start_indexes.len()
    }
}

struct RowsMutIter<'t, T> {
    remaining: &'t mut [T],
    rows_ranges: RowsRangesIter<'t>,
}

impl<'t, T> RowsMutIter<'t, T> {
    pub fn new(row_start_indexes: &'t [usize], remaining: &'t mut [T]) -> Self {
        Self {
            rows_ranges: RowsRangesIter {
                row_start_indexes,
                arr_len: remaining.len(),
            },
            remaining,
        }
    }

    fn take_prefix(&mut self, prefix_len: usize) -> &'t mut [T] {
        let (prefix, suffix) = self.remaining.split_at_mut(prefix_len);

        self.remaining = unsafe { std::mem::transmute::<&mut [T], &mut [T]>(suffix) };

        unsafe { std::mem::transmute::<&mut [T], &mut [T]>(prefix) }
    }
    fn take_suffix(&mut self, suffix_len: usize) -> &'t mut [T] {
        let (prefix, suffix) = self
            .remaining
            .split_at_mut(self.remaining.len() - suffix_len);

        self.remaining = unsafe { std::mem::transmute::<&mut [T], &mut [T]>(prefix) };

        unsafe { std::mem::transmute::<&mut [T], &mut [T]>(suffix) }
    }
}

impl<'t, T> Iterator for RowsMutIter<'t, T> {
    type Item = &'t mut [T];

    fn next(&mut self) -> Option<Self::Item> {
        let row_range = self.rows_ranges.next()?;

        let prefix = self.take_prefix(row_range.len());

        Some(prefix)
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.rows_ranges.size_hint()
    }
    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        if n == 0 {
            self.next()
        } else if n == 1 {
            let first = self.rows_ranges.next()?;
            self.take_prefix(first.len());
            self.next()
        } else {
            let first = self.rows_ranges.next()?;
            let last_skipped = self.rows_ranges.nth(n - 2)?;
            let skipped_len = last_skipped.end - first.start;
            self.take_prefix(skipped_len);
            self.next()
        }
    }
}

impl<'t, T> ExactSizeIterator for RowsMutIter<'t, T> {
    fn len(&self) -> usize {
        self.rows_ranges.len()
    }
}

impl<'t, T> DoubleEndedIterator for RowsMutIter<'t, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let last_range = self.rows_ranges.next_back()?;
        let suffix = self.take_suffix(last_range.len());
        Some(suffix)
    }
}

#[derive(Clone)]
struct RowsLengthsIter<'t> {
    rows_range_it: RowsRangesIter<'t>,
}
impl Iterator for RowsLengthsIter<'_> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.rows_range_it.next().map(|x| x.len())
    }
    fn nth(&mut self, index: usize) -> Option<Self::Item> {
        self.rows_range_it.nth(index).map(|x| x.len())
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl DoubleEndedIterator for RowsLengthsIter<'_> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.rows_range_it.next_back().map(|x| x.len())
    }
}

impl ExactSizeIterator for RowsLengthsIter<'_> {
    fn len(&self) -> usize {
        self.rows_range_it.len()
    }
}

#[derive(Clone)]
struct CoordsIter<'t> {
    row: i32,
    current_row_columns: Range<i32>,
    len_iter: RowsLengthsIter<'t>,
}

impl<'t> CoordsIter<'t> {
    fn new(row_start_indexes: &'t [usize], arr_len: usize) -> Self {
        Self {
            row: -1,
            current_row_columns: 0..0,
            len_iter: RowsLengthsIter {
                rows_range_it: RowsRangesIter {
                    row_start_indexes,
                    arr_len,
                },
            },
        }
    }
}

impl Iterator for CoordsIter<'_> {
    type Item = IVec2;

    fn next(&mut self) -> Option<Self::Item> {
        match self.current_row_columns.next() {
            Some(c) => Some(IVec2::new(c, self.row)),
            None => {
                let new_len = self.len_iter.next()?;
                self.current_row_columns = 0..new_len as i32;
                self.row += 1;
                self.next()
            }
        }
    }
}

impl<'t> ExactSizeIterator for CoordsIter<'t> {
    fn len(&self) -> usize {
        self.current_row_columns.len() + self.len_iter.clone().sum::<usize>()
    }
}

impl<T> Grid<T> {
    pub fn new(size: IVec2, value: T) -> Self
    where
        T: Copy,
    {
        let rows = size.y as usize;
        let cols = size.x as usize;
        Self {
            arr: vec![value; rows * cols],
            row_start_indexes: (0..rows).map(|i| i * cols).collect_vec(),
        }
    }
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arr: Vec::with_capacity(capacity),
            row_start_indexes: Vec::with_capacity(capacity.isqrt()),
        }
    }

    pub fn coords(&self) -> impl ExactSizeIterator<Item = IVec2> + '_ {
        CoordsIter::new(&self.row_start_indexes, self.arr.len())
    }
    pub fn elements_len(&self) -> usize {
        self.arr.len()
    }
    pub fn map<U>(&self, mut f: impl FnMut(&T, IVec2) -> U) -> Grid<U> {
        let mut arr: Vec<U> = Vec::with_capacity(self.arr.len());
        let row_start_indexes = self.row_start_indexes.clone();
        arr.extend((0..self.arr.len()).scan(IVec2::new(0, -1), |pos, i| {
            if row_start_indexes.contains(&i) {
                pos.y += 1;
                pos.x = 0;
            } else {
                pos.x += 1;
            }
            let x = &self.arr[i];
            let u = f(x, *pos);
            Some(u)
        }));

        Grid {
            arr,
            row_start_indexes,
        }
    }

    /// # Safety
    /// - all row_start_indexes are sorted in non-decreasing order (smaller first)
    /// - all row_start_indexes are valid indexes of the arr
    pub unsafe fn from_raw_parts(arr: Vec<T>, row_start_indexes: Vec<usize>) -> Self {
        Self {
            arr,
            row_start_indexes,
        }
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        self.arr.fill(value);
    }

    pub fn max_column(&self) -> usize {
        self.rows_ranges()
            .map(|r| r.len())
            .max()
            .unwrap_or_default()
    }
    pub fn min_column(&self) -> usize {
        self.rows_ranges()
            .map(|r| r.len())
            .min()
            .unwrap_or_default()
    }

    #[inline(always)]
    pub fn cols(&self, row: usize) -> usize {
        let Some(start) = self.row_start_indexes.get(row) else {
            return 0;
        };
        let end = self
            .row_start_indexes
            .get(row + 1)
            .copied()
            .unwrap_or(self.arr.len());
        end - start
    }
    #[inline(always)]
    pub fn rows_len(&self) -> usize {
        self.row_start_indexes.len()
    }
    #[inline(always)]
    fn rows_ranges(
        &self,
    ) -> impl ExactSizeIterator<Item = Range<usize>> + DoubleEndedIterator + '_ {
        RowsRangesIter {
            row_start_indexes: &self.row_start_indexes,
            arr_len: self.arr.len(),
        }
    }
    fn row_range(&self, row: usize) -> Option<Range<usize>> {
        let start = self.row_start_indexes.get(row).copied()?;
        let end = self
            .row_start_indexes
            .get(row + 1)
            .copied()
            .unwrap_or(self.arr.len());
        Some(start..end)
    }
    #[inline(always)]
    pub fn rows(&self) -> impl ExactSizeIterator<Item = &[T]> + DoubleEndedIterator + '_ {
        self.rows_ranges().map(|range| &self.arr[range])
    }

    /// Iterates over all "places" in the column.
    /// Place can be empty (if corresponding row does not have this column)
    pub fn safe_iter_col(
        &self,
        col: usize,
    ) -> impl ExactSizeIterator<Item = Option<&T>> + DoubleEndedIterator + '_ {
        self.rows().map(move |r| r.get(col))
    }

    /// Iterates over all "places" in the column.
    /// Place can be empty
    pub fn checked_iter_col_copied(
        &self,
        col: usize,
    ) -> impl ExactSizeIterator<Item = Option<T>> + DoubleEndedIterator + '_
    where
        T: Copy,
    {
        self.safe_iter_col(col).map(|x| x.copied())
    }

    pub fn iter_col(
        &self,
        col: usize,
    ) -> impl ExactSizeIterator<Item = &T> + DoubleEndedIterator + '_ {
        self.rows().map(move |r| r.get(col).unwrap())
    }

    pub fn iter_col_copied(
        &self,
        col: usize,
    ) -> impl ExactSizeIterator<Item = T> + DoubleEndedIterator + '_
    where
        T: Copy,
    {
        self.checked_iter_col_copied(col).map(|x| x.unwrap())
    }

    pub fn neighbours<'t, D>(
        &'t self,
        pos: IVec2,
        dirs: D,
    ) -> impl Iterator<Item = (IVec2, &'t T)> + 't
    where
        D: IntoIterator<Item = IVec2> + 't,
    {
        dirs.into_iter()
            .map(move |d| d + pos)
            .filter_map(|p| self.get(p).map(|x| (p, x)))
    }
    pub fn neighbours_copy<'t, D>(
        &'t self,
        pos: IVec2,
        dirs: D,
    ) -> impl Iterator<Item = (IVec2, T)> + 't
    where
        D: IntoIterator<Item = IVec2> + 't,
        T: Copy,
    {
        dirs.into_iter()
            .map(move |d| d + pos)
            .filter_map(|p| self.get_copy(p).map(|x| (p, x)))
    }

    #[inline(always)]
    pub fn rows_mut(
        &mut self,
    ) -> impl ExactSizeIterator<Item = &mut [T]> + DoubleEndedIterator + '_ {
        RowsMutIter::new(&self.row_start_indexes, self.arr.as_mut_slice())
    }
    pub fn iter_line(&self, pos: IVec2, v: IVec2) -> impl Iterator<Item = &T> {
        let mut p = pos;
        std::iter::from_fn(move || {
            let value = self.get(p)?;
            p += v;
            Some(value)
        })
    }
    pub fn iter_line_copy(&self, pos: IVec2, v: IVec2) -> impl Iterator<Item = T> + '_
    where
        T: Copy,
    {
        self.iter_line(pos, v).copied()
    }
    pub fn row(&self, row: usize) -> Option<&[T]> {
        let range = self.row_range(row)?;
        Some(&self.arr[range])
    }
    pub fn row_mut(&mut self, row: usize) -> Option<&mut [T]> {
        let range = self.row_range(row)?;
        Some(&mut self.arr[range])
    }
    pub fn set(&mut self, pos: IVec2, value: T) -> Option<T> {
        match self.get_mut(pos) {
            Some(prev) => Some(std::mem::replace(prev, value)),
            None => unreachable!("You cannot set value at {pos}. Grid size={}", self.size()),
        }
    }
    #[inline(always)]
    pub fn get(&self, pos: IVec2) -> Option<&T> {
        (pos.y >= 0 && pos.x >= 0)
            .then(|| self.get_at(pos.y as usize, pos.x as usize))
            .flatten()
    }
    pub fn get_copy(&self, pos: IVec2) -> Option<T>
    where
        T: Copy,
    {
        self.get(pos).copied()
    }

    pub fn get_at(&self, row: usize, col: usize) -> Option<&T> {
        self.row(row).and_then(|r| r.get(col))
    }

    pub fn get_copy_at(&self, row: usize, col: usize) -> Option<T>
    where
        T: Copy,
    {
        self.get_at(row, col).copied()
    }

    #[inline(always)]
    pub fn get_mut(&mut self, pos: IVec2) -> Option<&mut T> {
        let row = pos.y;
        if row < 0 {
            return None;
        }
        self.row_mut(row as usize).and_then(|r| {
            let col = pos.x;
            if col < 0 {
                return None;
            }
            r.get_mut(col as usize)
        })
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.arr.iter()
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.arr.iter_mut()
    }

    /// Iterates all cells in order of (0..rows).flatMap(|r| (0..cols))
    #[inline(always)]
    pub fn entries(&self) -> impl Iterator<Item = (IVec2, &T)> + '_ {
        let arr = &self.arr;
        self.row_start_indexes
            .iter()
            .copied()
            .enumerate()
            .flat_map(move |(i, start)| {
                let end = self
                    .row_start_indexes
                    .get(i + 1)
                    .copied()
                    .unwrap_or(arr.len());
                (start..end).map(move |ind| (IVec2::new((ind - start) as i32, i as i32), &arr[ind]))
            })
    }
    /// Iterates all cells in order of (0..rows).flatMap(|r| (0..cols))
    #[inline(always)]
    pub fn entries_copy(&self) -> impl Iterator<Item = (IVec2, T)> + '_
    where
        T: Copy,
    {
        let arr = &self.arr;
        self.row_start_indexes
            .iter()
            .copied()
            .enumerate()
            .flat_map(move |(i, start)| {
                let end = self
                    .row_start_indexes
                    .get(i + 1)
                    .copied()
                    .unwrap_or(arr.len());
                (start..end).map(move |ind| (IVec2::new((ind - start) as i32, i as i32), arr[ind]))
            })
    }

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.cols(0) as i32, self.rows_len() as i32)
    }

    pub fn resize(&mut self, new_size: IVec2, value: T)
    where
        T: Copy,
    {
        let new_cols = new_size.x as usize;
        let new_rows = new_size.y as usize;
        let old_rows = self.rows_len();

        let additional_space = self
            .rows()
            .map(|x| new_cols.saturating_sub(x.len()))
            .sum::<usize>()
            + (new_rows.saturating_sub(old_rows) * new_cols);

        if additional_space > 0 {
            let prev_n = self.arr.len();
            self.arr.resize(prev_n + additional_space, value);
            self.arr.copy_within(0..prev_n, additional_space);
            for x in self.row_start_indexes.iter_mut() {
                *x += additional_space;
            }
        }

        let mut dst = 0;
        for i in (0..self.rows_len()).take(new_rows) {
            let start = self.row_start_indexes[i];
            let end = self
                .row_start_indexes
                .get(i + 1)
                .copied()
                .unwrap_or(self.arr.len());

            let old_cols = end - start;
            if new_cols == old_cols && dst == start {
                dst += old_cols;
                continue;
            }
            self.row_start_indexes[i] = dst;
            if new_cols <= old_cols {
                let new_end = start + new_cols;
                self.arr.copy_within(start..new_end, dst);
                dst += new_end - start;
            } else {
                // new_cols > old_cols
                self.arr.copy_within(start..end, dst);
                dst += end - start;
                self.arr[dst..(dst + new_cols - old_cols)].fill(value);
                dst += new_cols - old_cols;
            }
        }
        self.row_start_indexes
            .reserve(new_rows.saturating_sub(old_rows));
        for _ in old_rows..new_rows {
            self.row_start_indexes.push(dst);
            self.arr[dst..(dst + new_cols)].fill(value);
            dst += new_cols;
        }
        if new_rows < old_rows {
            self.row_start_indexes.truncate(new_rows);
        }

        self.arr.truncate(dst);
    }

    ///
    /// Returns `None` if the grid itself cannot be tranposed.
    /// Example of impossible transpose:
    /// ```ignore
    /// abcdef
    /// ghi
    /// asdk
    /// ```
    ///
    pub fn transpose(&self) -> Option<Grid<T>>
    where
        T: Clone,
    {
        if self.size() == IVec2::ZERO {
            return Some(self.clone());
        }
        let cols_len = self.cols(0);
        if (1..self.rows_len()).any(|r| self.cols(r) != cols_len) {
            return None;
        }
        Some(
            (0..cols_len)
                .map(|c| self.rows().map(move |row| row.get(c).unwrap().clone()))
                .collect::<Grid<T>>(),
        )
    }
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

pub trait ToAsciiChar {
    fn to_ascii_char(&self) -> u8;
}

impl ToAsciiChar for bool {
    fn to_ascii_char(&self) -> u8 {
        if *self {
            b'#'
        } else {
            b'.'
        }
    }
}

impl ToAsciiChar for u8 {
    fn to_ascii_char(&self) -> u8 {
        *self
    }
}

impl<T: ToAsciiChar> ToAsciiChar for Option<T> {
    fn to_ascii_char(&self) -> u8 {
        match self {
            Some(t) => t.to_ascii_char(),
            None => b' ',
        }
    }
}

impl<T: ToAsciiChar> Grid<T> {
    pub fn render_ascii(&self) -> String {
        let mut res = String::with_capacity(self.arr.len() + self.row_start_indexes.len() + 1);
        for row in self.rows() {
            res.extend(row.iter().map(ToAsciiChar::to_ascii_char).map(char::from));
            res.push('\n');
        }
        res
    }
}

impl<X, Inner, InIter> FromIterator<Inner> for Grid<X>
where
    Inner: IntoIterator<Item = X, IntoIter = InIter>,
    InIter: Iterator<Item = X>,
{
    fn from_iter<T: IntoIterator<Item = Inner>>(iter: T) -> Self {
        let mut arr = Vec::new();
        let mut rows = Vec::new();
        let iter = iter.into_iter();
        let (lo, _) = iter.size_hint();
        rows.reserve(lo);
        for it in iter {
            let row = arr.len();
            arr.extend(it);
            rows.push(row)
        }
        Self {
            arr,
            row_start_indexes: rows,
        }
    }
}

/// Represents neighbours from top, right,bottom and left.
pub struct NonDiagonal;

impl NonDiagonal {
    pub const fn directions() -> [IVec2; 4] {
        [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X]
    }
}

impl NonDiagonal {
    pub fn to_str(dir: IVec2) -> &'static str {
        match (dir.x, dir.y) {
            (0, 1) => "v",
            (0, -1) => "^",
            (1, 0) => ">",
            (-1, 0) => "<",
            _ => "?",
        }
    }
    pub fn to_ascii_char(dir: IVec2) -> u8 {
        match (dir.x, dir.y) {
            (0, 1) => b'v',
            (0, -1) => b'^',
            (1, 0) => b'>',
            (-1, 0) => b'<',
            _ => b'?',
        }
    }
}

impl IntoIterator for NonDiagonal {
    type Item = IVec2;

    type IntoIter = std::array::IntoIter<IVec2, 4>;

    fn into_iter(self) -> Self::IntoIter {
        Self::directions().into_iter()
    }
}

/// Represents neighbours from
/// ```ignore
/// top-left    top       top-right
/// left                      right
/// bottom-left bottom bottom-right
/// ```
pub struct N8;

impl N8 {
    pub const fn directions() -> [IVec2; 8] {
        [
            IVec2::new(-1, -1),
            IVec2::new(0, -1),
            IVec2::new(1, -1),
            IVec2::new(1, 0),
            IVec2::new(1, 1),
            IVec2::new(0, 1),
            IVec2::new(-1, 1),
            IVec2::new(-1, 0),
        ]
    }
}

impl IntoIterator for N8 {
    type Item = IVec2;

    type IntoIter = std::array::IntoIter<IVec2, 8>;

    fn into_iter(self) -> Self::IntoIter {
        Self::directions().into_iter()
    }
}

#[cfg(test)]
mod tests {
    use glam::IVec2;
    use itertools::Itertools;

    #[test]
    fn test_rows_mut() {
        let mut grid = crate::parse::ascii_grid("123\n456\n789");
        grid.rows_mut().enumerate().for_each(|(i, r)| {
            r.iter_mut().for_each(|x| *x = b'0' + i as u8);
        });

        assert_eq!(grid.render_ascii(), "000\n111\n222\n");
    }
    #[test]
    fn test_rows_mut_len() {
        let mut grid = crate::parse::ascii_grid("123\n456\n789");
        let x = grid.rows_mut().len();
        assert_eq!(x, 3);
    }
    #[test]
    fn test_rows_mut_nth_1() {
        let mut grid = crate::parse::ascii_grid("123\n456\n789\nabc");
        let mut it = grid.rows_mut();
        let xs = it.nth(1).unwrap();
        for x in xs {
            *x = b'x';
        }
        let ys = it.next().unwrap();
        for y in ys {
            *y = b'y'
        }
        drop(it);
        assert_eq!(grid.render_ascii(), "123\nxxx\nyyy\nabc\n");
    }
    #[test]
    #[allow(clippy::iter_nth_zero)]
    fn test_rows_mut_nth_0() {
        let mut grid = crate::parse::ascii_grid("123\n456\n789\nabc");
        let mut it = grid.rows_mut();

        let xs = it.nth(0).unwrap();
        for x in xs {
            *x = b'x';
        }
        drop(it);
        assert_eq!(grid.render_ascii(), "xxx\n456\n789\nabc\n");
    }
    #[test]
    fn test_coords_iter_len() {
        let grid = crate::parse::ascii_grid("123\n456\n789\nabc");
        let it = grid.coords();
        assert_eq!(it.len(), 12);
        let grid = crate::parse::ascii_grid("123\n45\n789\nabc");
        let it = grid.coords();
        assert_eq!(it.len(), 11);
    }
    #[test]
    fn test_coords_iter() {
        let grid = crate::parse::ascii_grid("123\n456\n789\nabc");
        let it = grid.coords().collect_vec();
        assert_eq!(
            it,
            vec![
                IVec2::new(0, 0),
                IVec2::new(1, 0),
                IVec2::new(2, 0),
                IVec2::new(0, 1),
                IVec2::new(1, 1),
                IVec2::new(2, 1),
                IVec2::new(0, 2),
                IVec2::new(1, 2),
                IVec2::new(2, 2),
                IVec2::new(0, 3),
                IVec2::new(1, 3),
                IVec2::new(2, 3),
            ]
        )
    }
    #[test]
    fn test_rows_mut_nth_2() {
        let mut grid = crate::parse::ascii_grid("123\n456\n789\nabc");
        let mut it = grid.rows_mut();
        let xs = it.nth(2).unwrap();
        for x in xs {
            *x = b'x';
        }
        let xs = it.next().unwrap();
        for x in xs {
            *x = b'y';
        }
        drop(it);
        assert_eq!(grid.render_ascii(), "123\n456\nxxx\nyyy\n");
    }
    #[test]
    fn test_rows_mut_next_back() {
        let mut grid = crate::parse::ascii_grid("123\n456\n789\nabc");
        let mut it = grid.rows_mut();
        let xs = it.next_back().unwrap();
        for x in xs {
            *x = b'x';
        }
        let xs = it.next_back().unwrap();
        for x in xs {
            *x = b'y';
        }
        drop(it);
        assert_eq!(grid.render_ascii(), "123\n456\nyyy\nxxx\n");
    }

    #[test]
    fn test_get() {
        let grid = crate::parse::ascii_grid("123\n456\n789");
        assert_eq!(grid.get(IVec2::new(0, 0)).copied(), Some(b'1'));
        assert_eq!(grid.get(IVec2::new(1, 0)).copied(), Some(b'2'));
        assert_eq!(grid.get(IVec2::new(2, 0)).copied(), Some(b'3'));
        assert_eq!(grid.get(IVec2::new(0, 1)).copied(), Some(b'4'));
        assert_eq!(grid.get(IVec2::new(1, 1)).copied(), Some(b'5'));
        assert_eq!(grid.get(IVec2::new(2, 1)).copied(), Some(b'6'));
        assert_eq!(grid.get(IVec2::new(0, 2)).copied(), Some(b'7'));
        assert_eq!(grid.get(IVec2::new(1, 2)).copied(), Some(b'8'));
        assert_eq!(grid.get(IVec2::new(2, 2)).copied(), Some(b'9'));
        assert_eq!(
            grid.neighbours(
                IVec2::new(1, 1),
                [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
            )
            .collect::<Vec<_>>(),
            vec![
                (IVec2::new(2, 1), &b'6'),
                (IVec2::new(0, 1), &b'4'),
                (IVec2::new(1, 2), &b'8'),
                (IVec2::new(1, 0), &b'2')
            ]
        );
        assert_eq!(
            grid.neighbours(
                IVec2::new(0, 0),
                [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y]
            )
            .collect::<Vec<_>>(),
            vec![(IVec2::new(1, 0), &b'2'), (IVec2::new(0, 1), &b'4'),]
        );
        assert_eq!(grid.cols(0), 3);
        assert_eq!(grid.cols(1), 3);
        assert_eq!(grid.cols(2), 3);
        assert_eq!(grid.rows_len(), 3);
        assert_eq!(grid.rows_ranges().collect_vec(), vec![0..3, 3..6, 6..9]);
        let grid = crate::parse::ascii_grid("123\n46\n789");
        assert_eq!(grid.cols(0), 3);
        assert_eq!(grid.cols(1), 2);
        assert_eq!(grid.cols(2), 3);
        assert_eq!(grid.rows_len(), 3);
        assert_eq!(grid.get(IVec2::new(0, 0)).copied(), Some(b'1'));
        assert_eq!(grid.get(IVec2::new(1, 0)).copied(), Some(b'2'));
        assert_eq!(grid.get(IVec2::new(2, 0)).copied(), Some(b'3'));
        assert_eq!(grid.get(IVec2::new(0, 1)).copied(), Some(b'4'));
        assert_eq!(grid.get(IVec2::new(1, 1)).copied(), Some(b'6'));
        assert_eq!(grid.get(IVec2::new(2, 1)).copied(), None);
        assert_eq!(grid.get(IVec2::new(0, 2)).copied(), Some(b'7'));
        assert_eq!(grid.get(IVec2::new(1, 2)).copied(), Some(b'8'));
        assert_eq!(grid.get(IVec2::new(2, 2)).copied(), Some(b'9'));
        assert_eq!(grid.rows_ranges().collect_vec(), vec![0..3, 3..5, 5..8]);
    }
    #[test]
    fn test_new() {
        let grid = super::Grid::new(IVec2::new(3, 3), b'a');
        assert_eq!(grid.render_ascii(), "aaa\naaa\naaa\n");
        let grid = super::Grid::new(IVec2::new(2, 3), b'a');
        assert_eq!(grid.size(), IVec2::new(2, 3));
        assert_eq!(grid.render_ascii(), "aa\naa\naa\n");
    }
    #[test]
    fn test_map() {
        let grid = super::Grid::new(IVec2::new(3, 3), b'a');
        let mut inc = 0;
        assert_eq!(grid.render_ascii(), "aaa\naaa\naaa\n");
        let grid = grid.map(|x, p| {
            let res = x + p.x as u8 + p.y as u8;
            inc += 1;
            res
        });
        assert_eq!(grid.size(), IVec2::new(3, 3));
        assert_eq!(grid.render_ascii(), "abc\nbcd\ncde\n");
        assert_eq!(inc, 9);
    }
    #[test]
    fn test_resize() {
        const GRID_STR: &str = "123\n456\n789";
        let mut grid = crate::parse::ascii_grid(GRID_STR);
        assert_eq!(grid.render_ascii(), "123\n456\n789\n");
        grid.resize(IVec2::new(3, 2), b'0');
        assert_eq!(grid.render_ascii(), "123\n456\n");

        let mut grid = crate::parse::ascii_grid(GRID_STR);
        grid.resize(IVec2::new(2, 3), b'0');
        assert_eq!(grid.render_ascii(), "12\n45\n78\n");

        let mut grid = crate::parse::ascii_grid(GRID_STR);
        grid.resize(IVec2::new(2, 2), b'0');
        assert_eq!(grid.render_ascii(), "12\n45\n");

        let mut grid = crate::parse::ascii_grid("123\n456a\n789");
        grid.resize(IVec2::new(3, 3), b'0');
        assert_eq!(grid.render_ascii(), "123\n456\n789\n");

        let mut grid = crate::parse::ascii_grid("123\n45\n789");
        grid.resize(IVec2::new(3, 3), b'0');
        assert_eq!(grid.render_ascii(), "123\n450\n789\n");

        let mut grid = crate::parse::ascii_grid("123\n456\n789");
        grid.resize(IVec2::new(3, 4), b'0');
        assert_eq!(grid.render_ascii(), "123\n456\n789\n000\n");

        let mut grid = crate::parse::ascii_grid("123\n456\n789");
        grid.resize(IVec2::new(4, 3), b'0');
        assert_eq!(grid.render_ascii(), "1230\n4560\n7890\n");

        let mut grid = crate::parse::ascii_grid("123\n4\n789");
        grid.resize(IVec2::new(4, 3), b'0');
        assert_eq!(grid.render_ascii(), "1230\n4000\n7890\n");

        let mut grid = crate::parse::ascii_grid("123456\n4\n789");
        grid.resize(IVec2::new(4, 3), b'0');
        assert_eq!(grid.render_ascii(), "1234\n4000\n7890\n");

        let mut grid = crate::parse::ascii_grid("123456\n4\n789");
        grid.resize(IVec2::new(0, 0), b'0');
        assert_eq!(grid.render_ascii(), "");
        grid.resize(IVec2::new(3, 3), b'a');
        assert_eq!(grid.render_ascii(), "aaa\naaa\naaa\n");

        let mut grid = crate::parse::ascii_grid("123456\n4\n789");
        grid.resize(IVec2::new(1, 1), b'0');
        assert_eq!(grid.render_ascii(), "1\n");
    }
}

#[cfg(test)]
mod alloc_tests {
    use glam::IVec2;
    use mockalloc::Mockalloc;
    use rstest::rstest;
    use std::alloc::System;

    use super::Grid;

    #[global_allocator]
    static ALLOCATOR: Mockalloc<System> = Mockalloc(System);

    #[rstest]
    #[case(IVec2::new(3, 3), 2)]
    #[case(IVec2::new(128, 128), 2)]
    fn test_new(#[case] size: IVec2, #[case] allocs: u64) {
        let alloc_info = mockalloc::record_allocs(|| {
            Grid::new(size, b'0');
        });
        assert!(
            alloc_info.num_allocs() == allocs,
            "We expected at most {} allocations, but got {}",
            allocs,
            alloc_info.num_allocs(),
        );
    }

    #[rstest]
    #[case(crate::parse::ascii_grid("123\n456\n789"), IVec2::new(3, 2), 0)]
    #[case(crate::parse::ascii_grid("123\n456\n789"), IVec2::new(2, 3), 0)]
    #[case(crate::parse::ascii_grid("123\n456a\n789"), IVec2::new(3, 3), 0)]
    #[case(crate::parse::ascii_grid("123\n45\n789"), IVec2::new(3, 3), 1)]
    #[case(crate::parse::ascii_grid("123\n456\n789"), IVec2::new(3, 4), 2)]
    #[case(crate::parse::ascii_grid("123\n456\n789"), IVec2::new(4, 3), 1)]
    #[case(crate::parse::ascii_grid("1234\n45\n789"), IVec2::new(3, 3), 0)]
    #[case(crate::parse::ascii_grid("1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890\n1234567890"), IVec2::new(20, 20), 2)]
    fn test_resize(#[case] mut grid: Grid<u8>, #[case] new_size: IVec2, #[case] allocs: u64) {
        let alloc_info = mockalloc::record_allocs(|| {
            grid.resize(new_size, b'0');
        });
        assert!(
            alloc_info.num_allocs() <= allocs,
            "We expected at most {} allocations, but got {}",
            allocs,
            alloc_info.num_allocs(),
        );
    }
}
