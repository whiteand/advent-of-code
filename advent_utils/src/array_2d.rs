pub struct Array2d<T> {
    arr: Vec<T>,
    cols: usize,
}

pub struct Array2dIndex(usize);

impl<T> Array2d<T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            cols: 0,
            arr: Vec::with_capacity(capacity),
        }
    }
    pub fn get(&self, r: usize, c: usize) -> &T {
        &self.arr[r * self.cols + c]
    }
    pub fn get_copy(&self, r: usize, c: usize) -> T
    where
        T: Copy,
    {
        self.arr[r * self.cols + c]
    }
    pub fn clear_and_resize(&mut self, r: usize, c: usize, value: T)
    where
        T: Copy,
    {
        self.arr.clear();
        self.arr.resize(r * c, value);
        self.cols = c;
    }
    pub fn set(&mut self, r: usize, c: usize, value: T) {
        self.arr[r * self.cols + c] = value;
    }

    pub fn get_col_indexes(&mut self, c: usize) -> impl Iterator<Item = Array2dIndex> {
        let rows = self.arr.len() / self.cols;
        let cols = self.cols;
        (0..rows).map(move |r| Array2dIndex(r * cols + c))
    }

    pub fn get_mut_at(&mut self, Array2dIndex(i): Array2dIndex) -> &mut T {
        &mut self.arr[i]
    }

    pub fn get_row_mut(&mut self, r: usize) -> &mut [T] {
        let offset = r * self.cols;
        &mut self.arr[offset..(offset + self.cols)]
    }

    pub fn set_col(&mut self, c: usize, value: T)
    where
        T: Copy,
    {
        let n = self.arr.len();
        for i in (0..).map(|r| r * self.cols + c) {
            if i >= n {
                break;
            }
            self.arr[i] = value;
        }
    }

    pub fn raw(&mut self) -> &mut [T] {
        self.arr.as_mut_slice()
    }

    pub fn set_at(&mut self, i: Array2dIndex, v: T) {
        self.arr[i.0] = v;
    }
}
