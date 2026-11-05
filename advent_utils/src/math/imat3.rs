use glam::IVec3;

use crate::math::imat2::IMat2;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct IMat3 {
    elements: [i32; 9],
}

impl IMat3 {
    pub const fn zero() -> Self {
        Self { elements: [0; 9] }
    }
    pub const fn identity() -> Self {
        Self::from_axis(IVec3::X, IVec3::Y, IVec3::Z)
    }
    pub const fn new(elements: [i32; 9]) -> Self {
        Self { elements: elements }
    }
    pub const fn from_axis(ox: IVec3, oy: IVec3, oz: IVec3) -> Self {
        Self {
            elements: [ox.x, oy.x, oz.x, ox.y, oy.y, oz.y, ox.z, oy.z, oz.z],
        }
    }

    pub const fn get_row(&self, ind: usize) -> IVec3 {
        IVec3::new(
            self.elements[ind * 3 + 0],
            self.elements[ind * 3 + 1],
            self.elements[ind * 3 + 2],
        )
    }

    /// Returns A * column vector vec
    pub const fn mul_vec3(&self, vec: IVec3) -> IVec3 {
        IVec3::new(
            self.elements[0] * vec.x + self.elements[1] * vec.y + self.elements[2] * vec.z,
            self.elements[3] * vec.x + self.elements[4] * vec.y + self.elements[5] * vec.z,
            self.elements[6] * vec.x + self.elements[7] * vec.y + self.elements[8] * vec.z,
        )
    }

    pub const fn get_by_coords(&self, i: usize, j: usize) -> i32 {
        let ind = i * 3 + j;
        self.elements[ind]
    }
    pub const fn get_mut_by_coords(&mut self, i: usize, j: usize) -> &mut i32 {
        let ind = i * 3 + j;
        &mut self.elements[ind]
    }
    pub const fn cofactor(&self, i: usize, j: usize) -> IMat2 {
        let mut mat2 = [0; 4];
        let mut dst = 0;

        let mut ii = 0;

        if ii != i {
            let mut jj = 0;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
            jj += 1;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
            jj += 1;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
        }
        ii += 1;
        if ii != i {
            let mut jj = 0;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
            jj += 1;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
            jj += 1;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
        }
        ii += 1;
        if ii != i {
            let mut jj = 0;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
            jj += 1;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
                dst += 1;
            }
            jj += 1;
            if jj != j {
                mat2[dst] = self.elements[ii * 3 + jj];
            }
        }
        IMat2::new(mat2)
    }

    pub const fn cofactor_mat3(&self) -> Self {
        Self::new([
            self.cofactor(0, 0).determinant(),
            -self.cofactor(0, 1).determinant(),
            self.cofactor(0, 2).determinant(),
            -self.cofactor(1, 0).determinant(),
            self.cofactor(1, 1).determinant(),
            -self.cofactor(1, 2).determinant(),
            self.cofactor(2, 0).determinant(),
            -self.cofactor(2, 1).determinant(),
            self.cofactor(2, 2).determinant(),
        ])
    }
    pub const fn adjoint(&self) -> Self {
        self.cofactor_mat3().transpose()
    }

    pub const fn transpose(&self) -> Self {
        Self::new([
            self.elements[0],
            self.elements[3],
            self.elements[6],
            self.elements[1],
            self.elements[4],
            self.elements[7],
            self.elements[2],
            self.elements[5],
            self.elements[8],
        ])
    }

    pub const fn inverse(&self) -> Option<Self> {
        let determinant = self.determinant();

        if determinant == 0
            || self.elements[0] % determinant != 0
            || self.elements[1] % determinant != 0
            || self.elements[2] % determinant != 0
            || self.elements[3] % determinant != 0
            || self.elements[4] % determinant != 0
            || self.elements[5] % determinant != 0
            || self.elements[6] % determinant != 0
            || self.elements[7] % determinant != 0
            || self.elements[8] % determinant != 0
        {
            return None;
        }

        let adjoint_elements = self.adjoint().elements;

        Some(Self::new([
            adjoint_elements[0] / determinant,
            adjoint_elements[1] / determinant,
            adjoint_elements[2] / determinant,
            adjoint_elements[3] / determinant,
            adjoint_elements[4] / determinant,
            adjoint_elements[5] / determinant,
            adjoint_elements[6] / determinant,
            adjoint_elements[7] / determinant,
            adjoint_elements[8] / determinant,
        ]))
    }
    pub const fn determinant(&self) -> i32 {
        let [a, b, c, d, e, f, g, h, i] = self.elements;

        a * IMat2::new([e, f, h, i]).determinant() - b * IMat2::new([d, f, g, i]).determinant()
            + c * IMat2::new([d, e, g, h]).determinant()
    }
}

#[cfg(test)]
mod tests {
    use glam::IVec3;

    use crate::math::imat3::IMat3;

    #[test]
    fn test_adjoint() {
        let mat = IMat3::new([2, 1, 3, 0, 2, 4, 1, 1, 2]);
        let adjoint = mat.adjoint();
        assert_eq!(adjoint, IMat3::new([0, 1, -2, 4, 1, -8, -2, -1, 4]))
    }

    #[test]
    fn test_determinant() {
        let mat = IMat3::new([2, 1, 3, 0, 2, 4, 1, 1, 2]);
        assert_eq!(mat.determinant(), -2);
    }

    #[test]
    fn test_inverse() {
        // 0 1 0
        // 1,0,0
        // 0,0,1

        let a = IMat3::new([0, 1, 0, 1, 0, 0, 0, 0, 1]);
        let v = IVec3::new(1, 2, 3);
        let inv_a = a.inverse().unwrap();
        assert_eq!(v, inv_a.mul_vec3(a.mul_vec3(v)));
    }
}
