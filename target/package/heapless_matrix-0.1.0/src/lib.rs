use heapless::Vec;
use matrix_trait::{IsSquareMatrix, MatrixConcat, MatrixTrait, SquareMatrix};
#[allow(unused_imports)]
use core::clone;

pub mod matrix_trait;

pub mod matrix_ops;



#[derive(Debug, Clone)]
pub struct Matrix<const ROWS: usize, const COLS: usize> {
    data: Vec<Vec<f64, COLS>, ROWS>,
}

impl<const ROWS: usize, const COLS: usize> Matrix<ROWS, COLS> {
    #[allow(dead_code)]
    fn iter(&self) -> core::slice::Iter<'_, Vec<f64, COLS>> {
        self.data.iter()
    }

    #[allow(dead_code)]
    fn iter_mut(&mut self) -> core::slice::IterMut<'_, Vec<f64, COLS>> {
        self.data.iter_mut()
    }
}

impl<const ROWS: usize, const COLS: usize> MatrixTrait<ROWS, COLS> for Matrix<ROWS, COLS> {
    type TransposeType = Matrix<COLS, ROWS>;
    // type MultiplyType<const RHS_COLS: usize> = Matrix<ROWS, RHS_COLS>;

    fn new() -> Option<Self>
    where
        Self: Sized,
    {
        if ROWS < 1 || COLS < 1 {
            return None;
        }
        let mut vec: Vec<Vec<f64, COLS>, ROWS> = Vec::new();
        for _ in 0..ROWS {
            let mut helper: Vec<f64, COLS> = Vec::new();
            for _ in 0..COLS {
                helper.push(0.).unwrap();
            }
            vec.push(helper).unwrap();
        }
        Some(Matrix { data: vec })
    }

    fn eye() -> Option<Self>
    where
        Self: Sized,
    {
        if ROWS < 1 || COLS < 1 {
            return None;
        }

        let mut mat: Vec<Vec<f64, COLS>, ROWS> = Vec::new();

        for i in 0..ROWS {
            let mut row: Vec<f64, COLS> = Vec::new();
            for j in 0..COLS {
                if i == j {
                    row.push(1.).unwrap();
                } else {
                    row.push(0.).unwrap();
                }
            }

            mat.push(row).unwrap();
        }

        Some(Matrix { data: mat })
    }
    /// Function used to create a heapless matrix from an array
    /// Example:
    /// ```
    /// use heapless_matrix::{matrix_trait::MatrixTrait as _, Matrix};
    /// let data = [[1., 2.],
    ///             [3., 4.]];
    /// let mat: Matrix<2, 2> = Matrix::from_vector(data).unwrap();
    ///
    /// for i in 0..2 {
    ///     for j in 0..2 {
    ///         assert_eq!(data[i][j], mat[i][j]);
    ///     }
    /// }
    /// ```
    fn from_vector(data: [[f64; COLS]; ROWS]) -> Option<Self>
    where
        Self: Sized,
    {
        if data.len() != ROWS {
            return None;
        }

        let mut array_data: Vec<Vec<f64, COLS>, ROWS> = Vec::new();
        for row in data.iter() {
            if row.len() != COLS {
                return None;
            }
            let mut row_data: Vec<f64, COLS> = Vec::new();
            for &value in row.iter() {
                row_data.push(value).unwrap();
            }

            array_data.push(row_data).unwrap();
        }

        Some(Matrix { data: array_data })
    }

    fn to_double(&self) -> Option<f64> {
        if ROWS != 1 && COLS != 1 {
            return None;
        }
        Some(self[0][0])
    }

    fn transpose(&self) -> Self::TransposeType {
        let mut transpose: Matrix<COLS, ROWS> = Matrix::new().unwrap();

        for i in 0..ROWS {
            for j in 0..COLS {
                transpose[j][i] = self[i][j]
            }
        }
        transpose
    }

    fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), &'static str>{
        if row1 >= ROWS || row2 >= ROWS {
            return Err("Row index out of bounds");
        }
        self.data.swap(row1, row2);
        Ok(())
    }

    fn sub_matrix<const NEW_ROWS: usize, const NEW_COLS: usize>(&self, row_start: usize, col_start: usize) -> Result<Matrix<NEW_ROWS, NEW_COLS>, &'static str>{
        if row_start + NEW_ROWS > ROWS || col_start + NEW_COLS > COLS {
            return Err("Submatrix dimensions are out of bounds");
        }
        
        let mut sub_data: Matrix<NEW_ROWS, NEW_COLS> = Matrix::new().unwrap();
        for i in 0..NEW_ROWS {
            for j in 0..NEW_COLS {
                sub_data[i][j] = self[row_start + i][col_start + j];
            }
        }

        Ok(sub_data)
    }

}


impl<const ROWS: usize, const COLS: usize> MatrixConcat<ROWS, COLS> for Matrix<ROWS, COLS> {
    ///
    /// Example:
    /// ```
    /// use heapless_matrix::{matrix_trait::{MatrixTrait, MatrixConcat}, Matrix};
    /// let mat1: Matrix<2, 2> = Matrix::eye().unwrap();
    /// let mat2: Matrix<2, 2> = Matrix::new().unwrap();
    /// let mat3 = mat1.clone().x_concat::<2, 4>(mat2.clone()).unwrap();
    /// for i in 0..2 {
    ///     for j in 0..2 {
    ///         assert_eq!(mat3[i][j], mat1[i][j]);
    ///     }
    ///     for j in 0..2 {
    ///         assert_eq!(mat3[i][j + 2], mat2[i][j]);
    ///     }
    /// }
    /// ```
    fn x_concat<const RHS_COLS: usize, const NEW_COLS: usize>(
        self,
        rhs: Matrix<ROWS, RHS_COLS>,
    ) -> Option<Matrix<ROWS, NEW_COLS>> {
        if RHS_COLS + COLS != NEW_COLS {
            return None;
        }

        let mut new_data = [[0.; NEW_COLS]; ROWS];

        for i in 0..ROWS {
            for j in 0..COLS {
                new_data[i][j] = self.data[i][j];
            }
            for j in 0..RHS_COLS {
                new_data[i][COLS + j] = rhs.data[i][j];
            }
        }

        Matrix::from_vector(new_data)
    }
    
    fn y_concat<const RHS_ROWS: usize, const NEW_ROWS: usize>(
        self,
        rhs: Matrix<RHS_ROWS, COLS>
    ) -> Option<Matrix<NEW_ROWS, COLS>> {
        if ROWS + RHS_ROWS != NEW_ROWS {
            return None;
        }

        let mut new_data = [[0.; COLS]; NEW_ROWS];
        for i in 0..COLS {
            for j in 0..ROWS {
                new_data[j][i] = self[j][i];
            }
            for j in 0..RHS_ROWS{
                new_data[ROWS + j][i] = rhs[j][i];
            }
        }

        Matrix::from_vector(new_data)        
    }
}

impl<const N: usize> IsSquareMatrix for Matrix<N, N> {
    
}

impl<const N: usize> SquareMatrix<N> for Matrix<N, N>{
    fn det(&self) -> f64 {
        let mut copy = self.clone();
        for j in 0..(N - 1) {
            for i in ((j + 1)..N).rev(){
                // println!("i: {}, j: {}", i, j);
                if copy[j][j] == 0. && copy[i][j] == 0. {
                    return 0.;
                } else if copy[j][j] == 0. {
                    copy.swap_rows(j, i).unwrap();
                }
                let div = copy[i][j]/copy[j][j];
                for k in 0..N {
                    // println!("{}, {}, {}", copy[i][j],copy[0][j] , copy[0][k]);
                    copy[i][k] -= div * copy[j][k];
                }
                // println!("{:#?}", copy);
            }
        }
        let mut det = 1.;
        for i in 0..N {
            det *= copy[i][i];
        }
        det
    }
    
    fn inv<const DOUBLE_COLS: usize>(&self) -> Result<Matrix<N, N>, &'static str>{
        
        let mat: Matrix<N, N> = Matrix::eye().unwrap();
        let mut mat = self.clone().x_concat::<N, DOUBLE_COLS>(mat).unwrap();

        for j in 0..(N - 1) {
            for i in ((j + 1)..N).rev(){
                // println!("i: {}, j: {}", i, j);
                if mat[j][j] == 0. && mat[i][j] == 0. {
                    return Err("Matrix cannot be inverted");
                } else if mat[j][j] == 0. {
                    mat.swap_rows(j, i).unwrap();
                }
                let div = mat[i][j]/mat[j][j];
                for k in 0..DOUBLE_COLS {
                    // println!("{}, {}, {}", mat[i][j],mat[0][j] , mat[0][k]);
                    mat[i][k] -= div * mat[j][k];
                }
                // println!("{:#?}", mat);
            }
        }

        for j in (1..N).rev() {
            for i in 0..j{
                // println!("i: {}, j: {}", i, j);
                if mat[j][j] == 0. && mat[i][j] == 0. {
                    return Err("Matrix cannot be inverted");
                } else if mat[j][j] == 0. {
                    mat.swap_rows(j, i).unwrap();
                }
                let div = mat[i][j]/mat[j][j];
                for k in 0..DOUBLE_COLS {
                    // println!("{}, {}, {}", mat[i][j],mat[0][j] , mat[0][k]);
                    mat[i][k] -= div * mat[j][k];
                }
                // println!("{:#?}", mat);
            }
        }

        for i in 0..N {
            let div = mat[i][i];
            for j in N..DOUBLE_COLS {
                mat[i][j] /= div;
            }
        }

        mat.sub_matrix::<N, N>(0, N)
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn succes_creation() {
        type Mat3x3 = Matrix<3, 3>;
        match Mat3x3::new() {
            Some(_) => assert!(true),
            None => assert!(false),
        }
    }

    #[test]
    fn fail_creation_1() {
        type Mat3x3 = Matrix<0, 3>;
        match Mat3x3::new() {
            Some(_) => assert!(false),
            None => assert!(true),
        };
    }

    #[test]
    fn indexing_elems() {
        let mut mat2x2: Matrix<2, 2> = Matrix::new().unwrap();

        for i in 0..2 {
            for j in 0..2 {
                mat2x2[i][j] = i as f64 + j as f64;
            }
        }

        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(i as f64 + j as f64, mat2x2[i][j]);
            }
        }
    }

    #[test]
    fn iterating_elems() {
        let mut mat3x3: Matrix<1, 3> = Matrix::new().unwrap();

        for vec in mat3x3.iter_mut() {
            for elem in vec.iter_mut() {
                *elem = 3.;
            }
        }
        for vec in mat3x3.iter() {
            for elem in vec.iter() {
                assert_eq!(3., *elem);
            }
        }
    }

    #[test]
    fn testing_transpose1() {
        let mat4x1: Matrix<4, 1> = Matrix::new().unwrap();
        let mat1x4 = mat4x1.transpose();
        for i in 0..4 {
            for j in 0..1 {
                assert_eq!(mat4x1[i][j], mat1x4[j][i])
            }
        }
    }
    #[test]
    fn testing_eye() {
        let mat: Matrix<3, 3> = Matrix::eye().unwrap();

        for i in 0..3 {
            assert_eq!(1., mat[i][i])
        }
    }

    #[test]
    fn from_slices() {
        let data = [[2., 2.], [3., 3.], [4., 5.]];
        let mat: Matrix<3, 2> = Matrix::from_vector(data).unwrap();
        for i in 0..3 {
            for j in 0..2 {
                assert_eq!(data[i][j], mat[i][j]);
            }
        }
    }

    #[test]
    fn some_addition() {
        let mat1: Matrix<2, 2> = Matrix::from_vector([[1., 2.], [3., 4.]]).unwrap();

        let mat2: Matrix<2, 2> = Matrix::from_vector([[1., 2.], [3., 4.]]).unwrap();

        assert_eq!(
            Matrix::<2, 2>::from_vector([[2., 4.], [6., 8.],]).unwrap(),
            mat1 + mat2
        );
    }

    #[test]
    fn some_mul_1() {
        let mat1: Matrix<1, 4> = Matrix::from_vector([[1., 2., 3., 4.]]).unwrap();
        let mat2 = mat1.transpose();
        let res = mat1 * mat2;

        assert_eq!(30., res.to_double().unwrap());
    }

    #[test]
    fn basic_cloning(){
        let mat: Matrix<2, 2> = Matrix::from_vector([
            [2., 3.], 
            [4., -2.],
        ]).unwrap();

        let clone = mat.clone();
        assert_eq!(clone, mat);
    }

    #[test]
    fn basic_concat(){
        let mat1: Matrix<2, 2> = Matrix::from_vector([
            [1., 2.], 
            [3., 4.],
        ]).unwrap();

        let mat2: Matrix<2, 1> = Matrix::from_vector([
            [1.], 
            [2.], 
        ]).unwrap();

        let mat3 = mat1.clone().x_concat::<1, 3>(mat2.clone()).unwrap();
        for i in 0..2{
            for j in 0..2 {
                assert_eq!(mat3[i][j], mat1[i][j]);
            }
            for j in 0..1 {
                assert_eq!(mat3[i][2 + j], mat2[i][j]);
            }
        }
    }

    #[test]
    fn some_y_concat(){
        let mat1: Matrix<2, 2> = Matrix::from_vector([
            [2., 3.], 
            [1., 4.],
        ]).unwrap();

        let mat2: Matrix<1, 2> = Matrix::from_vector([
            [1., 2.]
        ]).unwrap();

        let mat3 = mat1.clone().y_concat::<1, 3>(mat2.clone()).unwrap();
        for i in 0..2{
            for j in 0..2 {
                assert_eq!(mat3[j][i], mat1[j][i]);
            }
            for j in 0..1{
                assert_eq!(mat3[j + 2][i], mat2[j][i]);
            }
        }
    }

    fn approx_equal(a: f64, b: f64, epsilon: f64) -> bool {
        (a - b).abs() < epsilon
    }
    #[test]
    fn basic_det(){
        let mat: Matrix<2, 2> = Matrix::from_vector([
            [1., 2.], 
            [3., 4.],
        ]).unwrap();

        assert_eq!(-2., mat.det());

        let mat: Matrix<1, 1> = Matrix::eye().unwrap();
        assert_eq!(1., mat.det());

        let mat: Matrix<3, 3> = Matrix::from_vector([
            [3., 5., 6.],
            [-2., 3., 5.], 
            [-1., 2., 7.],
        ]).unwrap();
        assert!(approx_equal(72., mat.det(), 1e-4));

        let mat: Matrix<4, 4> = Matrix::from_vector([
            [1., 2., 3., 4.], 
            [6., 7., 8., 9.], 
            [11., 12., 13., 14.], 
            [16., 17., 18., 19.], 
        ]).unwrap();
        assert!(approx_equal(0., mat.det(), 1e-10));

        let mat: Matrix<5, 5> = Matrix::from_vector([
            [1., 2., 3., 4., 5.], 
            [6., 7., 8., 9., 10.], 
            [11., 12., 13., 14., 11.], 
            [16., 17., 18., 19., 20.],
            [21., 22., 23., 24., 25.], 
        ]).unwrap();
        assert!(approx_equal(-2.7150e-44, mat.det(), 1e-10));

        let mat: Matrix<3, 3> = Matrix::eye().unwrap();
        assert!(approx_equal(1., mat.det(), 1e-10));

        let mat: Matrix<2, 2> = Matrix::from_vector([
            [1., 2.], 
            [1., 2.],
        ]).unwrap();
        assert!(approx_equal(0., mat.det(), 1e-10));

        let mut mat: Matrix<3, 3> = Matrix::eye().unwrap();
        mat[0][0] = 0.;
        assert!(approx_equal(0., mat.det(), 1e-10));
    }

    #[test]
    fn testing_inversion(){
        let mat: Matrix<3, 3> = Matrix::from_vector([
            [2., 3., 2.], 
            [1., 5., 3.], 
            [1., 3., 6.],
        ]).unwrap();

        assert_eq!(Matrix::<3, 3>::from_vector([[ 0.72413793, -0.4137931,  -0.03448276],
            [-0.10344828,  0.34482759, -0.13793103],
            [-0.06896552, -0.10344828,  0.24137931],]).unwrap(), mat.inv::<6>().unwrap());

        let mat: Matrix<2, 2> = Matrix::from_vector([
            [2., 3.], 
            [1., 2.],
        ]).unwrap();

        assert_eq!(Matrix::<2, 2>::from_vector(
            [[ 2., -3.],
            [-1.,  2.]]).unwrap(), mat.inv::<4>().unwrap());

        let mat: Matrix<1, 1> = Matrix::from_vector([[2.]]).unwrap();
        assert_eq!(0.5, mat.inv::<2>().unwrap().to_double().unwrap());        
        
        let mat: Matrix<2, 2> = Matrix::eye().unwrap();

        assert_eq!(Matrix::<2, 2>::eye().unwrap(), mat.inv::<4>().unwrap());

    }
}
