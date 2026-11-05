use std::ops::{Deref, DerefMut};

use glam::IVec2;

#[derive(Debug, Copy, Clone)]
pub struct IMat2 {
    elements: [i32; 4],
}

impl Deref for IMat2 {
    type Target = [i32; 4];

    fn deref(&self) -> &Self::Target {
        &self.elements
    }
}

impl DerefMut for IMat2 {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.elements
    }
}

impl IMat2 {
    pub const fn zero() -> Self {
        Self { elements: [0; 4] }
    }
    /**
     * Creates a matrix
     * (m1 m2)
     * (m3 m4)
     */
    pub const fn new(elements: [i32; 4]) -> Self {
        Self { elements: elements }
    }

    pub const fn identity() -> Self {
        Self::new([1, 0, 0, 1])
    }
    pub const fn from_columns(ox: IVec2, oy: IVec2) -> Self {
        Self::new([ox.x, oy.x, ox.y, oy.y])
    }
    pub fn get_by_coords(&self, i: usize, j: usize) -> i32 {
        assert!((0..2).contains(&i));
        assert!((0..2).contains(&j));
        let ind = i * 2 + j;
        self.elements[ind]
    }

    pub fn get_mut_by_coords(&mut self, i: usize, j: usize) -> i32 {
        assert!((0..2).contains(&i));
        assert!((0..2).contains(&j));
        let ind = i * 2 + j;
        self.elements[ind]
    }

    pub const fn determinant(&self) -> i32 {
        self.elements[0] * self.elements[3] - self.elements[1] * self.elements[2]
    }
}
