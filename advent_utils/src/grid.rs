use std::ops::Range;

use itertools::Itertools;

#[derive(Debug)]
pub struct Grid<T> {
    arr: Vec<T>,
    row_start_indexes: Vec<usize>,
}

impl<T> Grid<T> {
    pub fn coords(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        (0..self.rows_len())
            .flat_map(|r| (0..self.row(r).map_or(0, |r| r.len())).map(move |c| (r, c)))
    }
    pub fn map<U>(&self, f: impl Fn(&T, usize, usize) -> U) -> Grid<U> {
        let f = &f;
        self.rows()
            .enumerate()
            .map(|(i, row)| row.into_iter().enumerate().map(move |(j, x)| f(x, i, j)))
            .collect()
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

            let temp = std::mem::replace(&mut remaining, &mut []);
            let (row, rest) = temp.split_at_mut(row_len);

            remaining = rest;

            Some(row)
        })
    }
    pub fn iter_line(
        &self,
        row: isize,
        col: isize,
        delta_row: isize,
        delta_col: isize,
    ) -> impl Iterator<Item = &T> {
        let mut r = row;
        let mut c = col;
        std::iter::from_fn(move || {
            let r_u: usize = if r < 0 { return None } else { Some(r as usize) }?;
            let c_u: usize = if c < 0 { return None } else { Some(c as usize) }?;
            let value = self.get(r_u, c_u)?;
            r += delta_row;
            c += delta_col;
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
    pub fn set(&mut self, row: usize, col: usize, value: T) -> Option<T> {
        match self.get_mut(row, col) {
            Some(prev) => Some(std::mem::replace(prev, value)),
            None => unreachable!("You cannot set value at {row} and {col}"),
        }
    }
    #[inline(always)]
    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        self.row(row).and_then(|r| r.get(col))
    }
    #[inline(always)]
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        self.row_mut(row).and_then(|r| r.get_mut(col))
    }

    #[inline(always)]
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.arr.iter()
    }
    #[inline(always)]
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.arr.into_iter()
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.arr.iter_mut()
    }

    #[inline(always)]
    pub fn entries(&self) -> impl Iterator<Item = (usize, usize, &T)> + '_ {
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
                (start..end).map(move |ind| (i, ind - start, &arr[ind]))
            })
    }
}

impl Grid<u8> {
    pub fn from_ascii_grid(grid: &str) -> Self {
        grid.lines()
            .map(|line| line.as_bytes().into_iter().copied())
            .collect()
    }

    pub fn render_ascii(&self) -> String {
        let mut res = String::with_capacity(self.arr.len() + self.row_start_indexes.len() + 1);
        for row in self.rows() {
            res.extend(row.into_iter().copied().map(char::from));
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
    use itertools::Itertools;

    use super::Grid;

    #[test]
    fn test_get() {
        let grid = Grid::from_ascii_grid("123\n456\n789");
        assert_eq!(grid.get(0, 0).copied(), Some(b'1'));
        assert_eq!(grid.get(0, 1).copied(), Some(b'2'));
        assert_eq!(grid.get(0, 2).copied(), Some(b'3'));
        assert_eq!(grid.get(1, 0).copied(), Some(b'4'));
        assert_eq!(grid.get(1, 1).copied(), Some(b'5'));
        assert_eq!(grid.get(1, 2).copied(), Some(b'6'));
        assert_eq!(grid.get(2, 0).copied(), Some(b'7'));
        assert_eq!(grid.get(2, 1).copied(), Some(b'8'));
        assert_eq!(grid.get(2, 2).copied(), Some(b'9'));
        assert_eq!(grid.cols(0), 3);
        assert_eq!(grid.cols(1), 3);
        assert_eq!(grid.cols(2), 3);
        assert_eq!(grid.rows_len(), 3);
        assert_eq!(grid.rows_ranges().collect_vec(), vec![0..3, 3..6, 6..9]);
        let grid = Grid::from_ascii_grid("123\n46\n789");
        assert_eq!(grid.cols(0), 3);
        assert_eq!(grid.cols(1), 2);
        assert_eq!(grid.cols(2), 3);
        assert_eq!(grid.rows_len(), 3);
        assert_eq!(grid.get(0, 0).copied(), Some(b'1'));
        assert_eq!(grid.get(0, 1).copied(), Some(b'2'));
        assert_eq!(grid.get(0, 2).copied(), Some(b'3'));
        assert_eq!(grid.get(1, 0).copied(), Some(b'4'));
        assert_eq!(grid.get(1, 1).copied(), Some(b'6'));
        assert_eq!(grid.get(1, 2).copied(), None);
        assert_eq!(grid.get(2, 0).copied(), Some(b'7'));
        assert_eq!(grid.get(2, 1).copied(), Some(b'8'));
        assert_eq!(grid.get(2, 2).copied(), Some(b'9'));
        assert_eq!(grid.rows_ranges().collect_vec(), vec![0..3, 3..5, 5..8]);
    }
}
