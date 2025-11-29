use crate::math::Rat;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum SolveError {
    #[error("lefts should have the same len as rights")]
    IncompatibleLeftAndRight,
    #[error("failed to determine the variable at index {0}")]
    CannotSolveFor(usize),
}

pub struct Equations<'t> {
    pub lefts: &'t mut [&'t mut [Rat]],
    pub rights: &'t mut [Rat],
}

impl Equations<'_> {
    fn multiply_row(&mut self, row_index: usize, k: &Rat) {
        let vars = self.lefts[0].len();
        for j in 0..vars {
            self.lefts[row_index][j] *= *k;
        }
        self.rights[row_index] *= *k;
    }

    fn sub_row(&mut self, row_index: usize, other_row_index: usize) {
        let vars = self.lefts[0].len();
        for j in 0..vars {
            let new_value = self.lefts[row_index][j] - self.lefts[other_row_index][j];
            self.lefts[row_index][j] = new_value;
        }

        self.rights[row_index] -= self.rights[other_row_index];
    }

    pub fn solve(mut self) -> Result<(), SolveError> {
        let vars = self.lefts[0].len();
        let eqs = self.rights.len();

        if vars > eqs {
            return Err(SolveError::IncompatibleLeftAndRight);
        }
        for var_index in 0..vars {
            let Some(non_zero_row) = self
                .lefts
                .iter()
                .enumerate()
                .skip(var_index)
                .find(|(_, row)| !row[var_index].is_zero())
                .map(|(i, _)| i)
            else {
                return Err(SolveError::CannotSolveFor(var_index));
            };
            self.multiply_row(non_zero_row, &self.lefts[non_zero_row][var_index].reverse());
            self.swap_rows(var_index, non_zero_row);
            for eq_index in (var_index + 1)..eqs {
                let own_coef = self.lefts[eq_index][var_index];
                if own_coef.is_zero() {
                    continue;
                }
                let k = own_coef.reverse();

                self.multiply_row(eq_index, &k);
                self.sub_row(eq_index, var_index);
            }
        }

        for var_index in (0..vars).rev() {
            for eq in (0..var_index).rev() {
                let own_coef = self.lefts[eq][var_index];
                if own_coef.is_zero() {
                    continue;
                }
                let k = own_coef.reverse();
                self.multiply_row(var_index, &own_coef);
                self.sub_row(eq, var_index);
                self.multiply_row(var_index, &k);
            }
        }
        Ok(())
    }

    fn swap_rows(&mut self, var_index: usize, non_zero_row: usize) {
        if var_index == non_zero_row {
            return;
        }
        let vars = self.lefts[0].len();
        for j in 0..vars {
            let tmp = self.lefts[var_index][j];
            self.lefts[var_index][j] = self.lefts[non_zero_row][j];
            self.lefts[non_zero_row][j] = tmp;
        }
        self.rights.swap(var_index, non_zero_row);
    }
}

pub fn solve_system<const VARS: usize, const EQS: usize>(
    mut lefts: [[Rat; VARS]; EQS],
    mut rights: [Rat; EQS],
) -> Option<[Rat; EQS]> {
    let mut lefts: [&mut [Rat]; EQS] =
        array_init::from_iter(lefts.iter_mut().map(|x| x.as_mut())).unwrap();
    let rights_mut = rights.as_mut();
    Equations {
        lefts: &mut lefts,
        rights: rights_mut,
    }
    .solve()
    .ok()?;

    Some(rights)
}
