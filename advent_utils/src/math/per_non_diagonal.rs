use glam::IVec2;

#[derive(Debug, Clone, Copy)]
pub struct PerNonDiagonalDirection<T> {
    pub up: T,
    pub right: T,
    pub left: T,
    pub down: T,
}

impl<T> PerNonDiagonalDirection<T> {
    pub fn from_fn(f: impl FnMut(IVec2) -> T) -> Self {
        let [up, right, down, left] = Self::dirs().map(f);
        Self {
            up,
            right,
            left,
            down,
        }
    }
    pub fn splat(v: &T) -> Self
    where
        T: Clone,
    {
        Self::from_fn(|_| v.clone())
    }
    pub fn get(&self, dir: IVec2) -> Option<&T> {
        match (dir.x, dir.y) {
            (0, 1) => Some(&self.down),
            (0, -1) => Some(&self.up),
            (1, 0) => Some(&self.right),
            (-1, 0) => Some(&self.left),
            _ => None,
        }
    }
    pub unsafe fn unsafe_get(&self, dir: IVec2) -> &T {
        match (dir.x, dir.y) {
            (0, 1) => &self.down,
            (0, -1) => &self.up,
            (1, 0) => &self.right,
            (-1, 0) => &self.left,
            _ => unreachable!(),
        }
    }
    pub fn get_mut(&mut self, dir: IVec2) -> Option<&mut T> {
        match (dir.x, dir.y) {
            (0, 1) => Some(&mut self.down),
            (0, -1) => Some(&mut self.up),
            (1, 0) => Some(&mut self.right),
            (-1, 0) => Some(&mut self.left),
            _ => None,
        }
    }
    pub unsafe fn unsafe_get_mut(&mut self, dir: IVec2) -> &mut T {
        match (dir.x, dir.y) {
            (0, 1) => &mut self.down,
            (0, -1) => &mut self.up,
            (1, 0) => &mut self.right,
            (-1, 0) => &mut self.left,
            _ => unreachable!(),
        }
    }

    pub fn all(&self) -> [(IVec2, &T); 4] {
        [
            (IVec2::NEG_Y, &self.up),
            (IVec2::X, &self.right),
            (IVec2::Y, &self.down),
            (IVec2::NEG_X, &self.left),
        ]
    }
    pub fn all_mut(&mut self) -> [(IVec2, &mut T); 4] {
        [
            (IVec2::NEG_Y, &mut self.up),
            (IVec2::X, &mut self.right),
            (IVec2::Y, &mut self.down),
            (IVec2::NEG_X, &mut self.left),
        ]
    }

    pub fn dirs() -> [IVec2; 4] {
        [IVec2::NEG_Y, IVec2::X, IVec2::Y, IVec2::NEG_X]
    }
}
