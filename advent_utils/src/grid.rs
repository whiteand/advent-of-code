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

/// Represents neighbours from top, right,bottom and left.
pub struct NonDiagonal;

impl IntoIterator for NonDiagonal {
    type Item = IVec2;

    type IntoIter = std::array::IntoIter<IVec2, 4>;

    fn into_iter(self) -> Self::IntoIter {
        [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X].into_iter()
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

    pub fn coords(&self) -> impl Iterator<Item = IVec2> + '_ {
        (0..self.rows_len()).flat_map(|r| {
            (0..self.row(r).map_or(0, |r| r.len())).map(move |c| IVec2::new(c as i32, r as i32))
        })
    }
    pub fn elements_len(&self) -> usize {
        self.arr.len()
    }
    pub fn map<U>(&self, f: impl Fn(&T, IVec2) -> U) -> Grid<U> {
        let f = &f;
        self.rows()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(move |(j, x)| f(x, IVec2::new(i as i32, j as i32)))
            })
            .collect()
    }

    pub fn fill(&mut self, value: T)
    where
        T: Copy,
    {
        self.arr.fill(value);
    }
    #[inline(always)]
    pub fn cols(&self, row: usize) -> usize {
        let start = self.row_start_indexes[row];
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
    fn rows_ranges(&self) -> impl Iterator<Item = Range<usize>> + '_ {
        self.row_start_indexes
            .iter()
            .copied()
            .chain(std::iter::once(self.arr.len()))
            .tuple_windows()
            .map(|(start, end)| start..end)
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
    pub fn rows(&self) -> impl Iterator<Item = &[T]> + '_ {
        self.rows_ranges().map(|range| &self.arr[range])
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

    #[inline(always)]
    pub fn rows_mut(&mut self) -> impl Iterator<Item = &mut [T]> + '_ {
        let mut row_lengths = self
            .row_start_indexes
            .iter()
            .copied()
            .chain(std::iter::once(self.arr.len()))
            .tuple_windows()
            .map(|(start, end)| end - start);

        let mut remaining = self.arr.as_mut_slice();
        std::iter::from_fn(move || {
            let row_len = row_lengths.next()?;

            let temp = std::mem::take(&mut remaining);
            let (row, rest) = temp.split_at_mut(row_len);

            remaining = rest;

            Some(row)
        })
    }
    pub fn iter_line(&self, pos: IVec2, v: IVec2) -> impl Iterator<Item = &T> {
        let mut p = pos;
        std::iter::from_fn(move || {
            let value = self.get(p)?;
            p += v;
            Some(value)
        })
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
            None => unreachable!("You cannot set value at {pos}"),
        }
    }
    #[inline(always)]
    pub fn get(&self, pos: IVec2) -> Option<&T> {
        let row = pos.y;
        if row < 0 {
            return None;
        }
        self.row(row as usize).and_then(|r| {
            let col = pos.x;
            if col < 0 {
                return None;
            }
            r.get(col as usize)
        })
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

    pub fn size(&self) -> IVec2 {
        IVec2::new(self.rows_len() as i32, self.cols(0) as i32)
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
}

impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.arr.into_iter()
    }
}

impl Grid<u8> {
    pub fn render_ascii(&self) -> String {
        let mut res = String::with_capacity(self.arr.len() + self.row_start_indexes.len() + 1);
        for row in self.rows() {
            res.extend(row.iter().copied().map(char::from));
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
        for it in iter {
            let row = arr.len();
            for x in it {
                arr.push(x);
            }
            rows.push(row)
        }
        Self {
            arr,
            row_start_indexes: rows,
        }
    }
}

#[cfg(test)]
mod tests {
    use glam::IVec2;
    use itertools::Itertools;

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
