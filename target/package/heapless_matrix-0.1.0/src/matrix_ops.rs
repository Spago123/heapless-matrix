use heapless::Vec;

use core::ops::{Add, Index, IndexMut, Mul, AddAssign};

use core::cmp::PartialEq;

use core::usize;

use crate::{matrix_trait::MatrixTrait as _, Matrix};

impl<const ROWS: usize, const COLS: usize> Index<usize> for Matrix<ROWS, COLS> {
    type Output = Vec<f64, COLS>;

    fn index(&self, index: usize) -> &Self::Output {
        if index >= ROWS {
            panic!("Row index is out of bounds");
        }
        &self.data[index]
    }
}

impl<const ROWS: usize, const COLS: usize> IndexMut<usize> for Matrix<ROWS, COLS> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= ROWS {
            panic!("Row index is out of bounds");
        }
        &mut self.data[index]
    }
}

impl<const ROWS: usize, const COLS: usize> Add<Matrix<ROWS, COLS>> for Matrix<ROWS, COLS> {
    type Output = Matrix<ROWS, COLS>;

    fn add(self, rhs: Matrix<ROWS, COLS>) -> Self::Output {
        let mut result: Matrix<ROWS, COLS> = Matrix::new().unwrap();

        for i in 0..ROWS {
            for j in 0..COLS {
                result[i][j] = self[i][j] + rhs[i][j];
            }
        }

        result
    }
}

impl<const ROWS: usize, const COLS: usize> AddAssign<Matrix<ROWS, COLS>> for Matrix<ROWS, COLS> {
    fn add_assign(&mut self, rhs: Matrix<ROWS, COLS>) {
        for i in 0..ROWS {
            for j in 0..COLS {
                self[i][j] += rhs[i][j];
            }
        }
    }
}

impl<const LHS_ROWS: usize, const LHS_COLS: usize, const RHS_COLS: usize>
    Mul<Matrix<LHS_COLS, RHS_COLS>> for Matrix<LHS_ROWS, LHS_COLS>
{
    type Output = Matrix<LHS_ROWS, RHS_COLS>;

    fn mul(self, rhs: Matrix<LHS_COLS, RHS_COLS>) -> Self::Output {
        let mut result: Matrix<LHS_ROWS, RHS_COLS> = Matrix::new().unwrap();

        for i in 0..LHS_ROWS {
            for j in 0..RHS_COLS {
                let mut res = 0.;
                for k in 0..LHS_COLS {
                    res += self[i][k] * rhs[k][j];
                }
                result[i][j] = res;
            }
        }
        result
    }
}

fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
    (a - b).abs() < epsilon
}

impl<const ROWS: usize, const COLS: usize> PartialEq<Matrix<ROWS, COLS>> for Matrix<ROWS, COLS> {
    fn eq(&self, other: &Matrix<ROWS, COLS>) -> bool {
        for i in 0..ROWS {
            for j in 0..COLS {
                if !approx_equal(self[i][j], other[i][j], 1e-5) {
                    return false;
                }
            }
        }
        true
    }
}

