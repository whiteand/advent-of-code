use itertools::Itertools;

use super::vector::Vector;

#[derive(Default)]
pub struct Figure {
    pub points: Vec<Vector>,
}

impl Figure {
    /// Returns the height of this [`Figure`].
    ///
    /// # Panics
    ///
    /// Panics if figure is empty
    pub fn height(&self) -> usize {
        self.points.last().unwrap().y as usize + 1
    }

    /// Returns the width of this [`Figure`].
    ///
    /// # Panics
    ///
    /// Panics if figure is empty
    pub fn width(&self) -> usize {
        self.points.iter().map(|v| v.x).max().unwrap() as usize + 1
    }

    pub fn contains(&self, target_x: isize, target_y: isize) -> bool {
        self.points
            .iter()
            .take_while(|v| v.y <= target_y)
            .contains(&Vector::new(target_x, target_y))
    }
}

impl std::fmt::Debug for Figure {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.contains(x as isize, y as isize) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<Vec<(isize, isize)>> for Figure {
    fn from(points: Vec<(isize, isize)>) -> Self {
        Figure {
            points: points.into_iter().map(Vector::from).collect(),
        }
    }
}
