#[derive(PartialEq, Eq, Clone)]
struct Pattern {
    rocks: Vec<Vec<bool>>,
    transposed: bool,
}

impl std::fmt::Display for Pattern {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for r in 0..self.rows() {
            for c in 0..self.cols() {
                write!(f, "{}", if self.get(r, c) { '#' } else { '.' })?;
            }
        }
        Ok(())
    }
}

#[derive(PartialEq, Eq, Debug)]
enum Symmetry {
    Row(usize),
    Col(usize),
}

fn is_prime(&n: &usize) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut p = 3;
    while p * p > n {
        if n % p == 0 {
            return false;
        }
        p += 2;
    }
    true
}

impl Pattern {
    fn transpose(&mut self) {
        self.transposed = !self.transposed;
    }

    fn rows(&self) -> usize {
        if self.transposed {
            self.rocks[0].len()
        } else {
            self.rocks.len()
        }
    }

    fn cols(&self) -> usize {
        if self.transposed {
            self.rocks.len()
        } else {
            self.rocks[0].len()
        }
    }

    fn get(&self, r: usize, c: usize) -> bool {
        if self.transposed {
            self.rocks[c][r]
        } else {
            self.rocks[r][c]
        }
    }

    fn is_symmetry(&self, mr: usize) -> bool {
        let start = if mr >= self.rows() / 2 {
            2 * mr + 2 - self.rows()
        } else {
            0
        };
        for row in start..=mr {
            let mirrored_row = 2 * mr - row + 1;

            for col in 0..self.cols() {
                let obj = self.get(row, col);
                let image = self.get(mirrored_row, col);
                if obj != image {
                    return false;
                }
            }
        }
        true
    }

    fn get_symmetry(&mut self) -> Option<Symmetry> {
        if let Some(row) = self.get_mirrored_row().next() {
            return Some(Symmetry::Row(row));
        }
        self.transpose();
        let col_sym = self.get_mirrored_row().next();
        if let Some(col) = col_sym {
            self.transpose();
            return Some(Symmetry::Col(col));
        }
        self.transpose();
        None
    }

    fn get_mirrored_row(&self) -> impl Iterator<Item = usize> + '_ {
        let primes = std::iter::successors(Some(2), |n| Some(n + 1))
            .filter(is_prime)
            .take(self.cols())
            .collect::<Vec<_>>();
        let mut prefix_row_col_product_sum = vec![0; self.rows()];
        let mut prefix_rocks_sum = vec![0; self.rows()];
        let mut prefix_row_coef_sum = vec![0; self.rows()];
        let mut suffix_row_col_product_sum = vec![0; self.rows()];
        let mut suffix_row_coef_sum = vec![0; self.rows()];
        let mut suffix_rocks_sum = vec![0; self.rows()];
        prefix_rocks_sum[0] = (0..self.cols()).filter(|c| self.get(0, *c)).count();
        prefix_row_coef_sum[0] = (0..self.cols())
            .filter(|c| self.get(0, *c))
            .map(|c| primes[c])
            .sum();
        prefix_row_col_product_sum[0] = prefix_row_coef_sum[0];
        for row in 1..self.rows() {
            let rocks_n = (0..self.cols()).filter(|c| self.get(row, *c)).count();
            prefix_rocks_sum[row] = prefix_rocks_sum[row - 1] + rocks_n;

            let row_coef: usize = (0..self.cols())
                .filter(|c| self.get(row, *c))
                .map(|c| primes[c])
                .sum();
            prefix_row_coef_sum[row] = prefix_row_coef_sum[row - 1] + row_coef;

            let row_col_product = row_coef * (row + 1);
            prefix_row_col_product_sum[row] = prefix_row_col_product_sum[row - 1] + row_col_product;
        }
        suffix_rocks_sum[self.rows() - 1] = (0..self.cols())
            .filter(|c| self.get(self.rows() - 1, *c))
            .count();
        suffix_row_coef_sum[self.rows() - 1] = (0..self.cols())
            .filter(|c| self.get(self.rows() - 1, *c))
            .map(|c| primes[c])
            .sum();
        suffix_row_col_product_sum[self.rows() - 1] =
            suffix_row_coef_sum[self.rows() - 1] * self.rows();

        for row in (0..self.rows() - 1).rev() {
            let rocks_n = (0..self.cols()).filter(|c| self.get(row, *c)).count();
            suffix_rocks_sum[row] = suffix_rocks_sum[row + 1] + rocks_n;

            let row_coef: usize = (0..self.cols())
                .filter(|c| self.get(row, *c))
                .map(|c| primes[c])
                .sum();
            suffix_row_coef_sum[row] = suffix_row_coef_sum[row + 1] + row_coef;

            let row_col_product = row_coef * (row + 1);
            suffix_row_col_product_sum[row] = suffix_row_col_product_sum[row + 1] + row_col_product;
        }

        return (0..(self.rows() - 1)).filter_map(move |mr| {
            if mr * 2 + 2 == self.rows() {
                let n = prefix_rocks_sum[self.rows() - 1];
                if n % 2 != 0 {
                    return None;
                }
                let top_part_sum = prefix_row_col_product_sum[mr];
                let bottom_part_sum = suffix_row_col_product_sum[mr + 1];
                let s = top_part_sum + bottom_part_sum;
                let coef_sum = prefix_row_coef_sum[self.rows() - 1];
                let target = (mr * 2 + 3) * coef_sum / 2;
                if s == target && self.is_symmetry(mr) {
                    return Some(mr);
                }
            }

            // 2mr - r + 1 = r
            // 2mr + 2
            if mr < self.rows() / 2 {
                let n = prefix_rocks_sum[2 * mr + 1];
                if n % 2 != 0 {
                    return None;
                }
                let coef_sum = prefix_row_coef_sum[2 * mr + 1];

                // only top part is reflecting
                let top_part_sum = prefix_row_col_product_sum[mr];
                let bottom_part_sum =
                    suffix_row_col_product_sum[mr + 1] - suffix_row_col_product_sum[2 * mr + 2];
                let s = top_part_sum + bottom_part_sum;
                let target = (mr * 2 + 3) * coef_sum / 2;
                if s == target && self.is_symmetry(mr) {
                    return Some(mr);
                }
            } else {
                // only bottom part is reflecting
                let n = suffix_rocks_sum[2 + 2 * mr - self.rows()];
                if n % 2 != 0 {
                    return None;
                }

                let top_part_sum = prefix_row_col_product_sum[mr]
                    - prefix_row_col_product_sum[2 * mr + 1 - self.rows()];
                let bottom_part_sum = suffix_row_col_product_sum[mr + 1];
                let coef_sum = suffix_row_coef_sum[2 + 2 * mr - self.rows()];
                let s = top_part_sum + bottom_part_sum;
                let target = (mr * 2 + 3) * coef_sum / 2;

                if s == target && self.is_symmetry(mr) {
                    return Some(mr);
                }
            }
            return None;
        });
    }

    fn flip(&mut self, r: usize, c: usize) {
        if self.transposed {
            self.rocks[c][r] = !self.rocks[c][r];
        } else {
            self.rocks[r][c] = !self.rocks[r][c];
        }
    }

    fn fix_smudge(&mut self) {
        let symmetry = self
            .get_symmetry()
            .expect("some symmetry should be present");

        for i in 0..self.rows() {
            for j in 0..self.cols() {
                self.flip(i, j);
                for r in self.get_mirrored_row() {
                    match symmetry {
                        Symmetry::Row(row) if r != row => {
                            return;
                        }
                        Symmetry::Col(_) => return,
                        _ => {}
                    };
                }

                let mut found = false;
                self.transpose();
                for c in self.get_mirrored_row() {
                    match symmetry {
                        Symmetry::Row(_) => {
                            found = true;
                            break;
                        }
                        Symmetry::Col(col) if col != c => {
                            found = true;
                            break;
                        }
                        _ => {}
                    };
                }
                self.transpose();
                if found {
                    return;
                }
                self.flip(i, j);
            }
        }
        unreachable!("Smudge does not exists: {}", self)
    }
}

pub fn solve_task1(file_content: &str) -> usize {
    parse_patterns(file_content)
        .map(|mut pattern| {
            match pattern
                .get_symmetry()
                .expect("There should be at least one mirrored row or col")
            {
                Symmetry::Row(row) => (row + 1) * 100,
                Symmetry::Col(col) => col + 1,
            }
        })
        .sum()
}
pub fn solve_task2(file_content: &str) -> usize {
    parse_patterns(file_content)
        .map(|mut pattern| {
            pattern.fix_smudge();
            match pattern
                .get_symmetry()
                .expect("There should be at least one mirrored row or col")
            {
                Symmetry::Row(row) => (row + 1) * 100,
                Symmetry::Col(col) => col + 1,
            }
        })
        .sum()
}
fn parse_patterns(rocks: &str) -> impl Iterator<Item = Pattern> + '_ {
    rocks.split("\n\n").map(|pattern| {
        let rocks = pattern
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        '.' => false,
                        '#' => true,
                        _ => panic!("Invalid char"),
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Pattern {
            rocks,
            transposed: false,
        }
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = include_str!("./y23d13/example.txt");
    const ACTUAL: &str = include_str!("../../benches/y23/y23d13.txt");
    #[test]
    fn test_task1() {
        assert_eq!(format!("{}", solve_task1(INPUT)), "405");
    }

    #[test]
    fn test_task1_actual() {
        assert_eq!(format!("{}", solve_task1(ACTUAL)), "33122");
    }

    #[test]
    fn test_task2() {
        assert_eq!(solve_task2(INPUT), 400);
    }

    #[test]
    fn test_task2_actual() {
        assert_ne!(format!("{}", solve_task2(ACTUAL)), "40692");
        assert_eq!(format!("{}", solve_task2(ACTUAL)), "0");
    }
}
