use crate::Matrix;

#[allow(dead_code)]
pub trait MatrixTrait<const ROWS: usize, const COLS: usize> {
    type TransposeType: MatrixTrait<COLS, ROWS>;

    ///
    /// Function that creates an matrix where all elements are equal to zero
    fn new() -> Option<Self>
    where
        Self: Sized;

    ///
    /// Function that creates an identity matrix
    /// 
    fn eye() -> Option<Self>
    where
        Self: Sized;
    ///
    /// Function that creates an heapless matrix from an array
    ///
    fn from_vector(data: [[f64; COLS]; ROWS]) -> Option<Self>
    where
        Self: Sized;

    ///
    /// Function that transposes a matrix
    /// 
    fn transpose(&self) -> Self::TransposeType;

    ///
    /// Function that converts a 1x1 matrix into a f64
    /// 
    fn to_double(&self) -> Option<f64>;

    fn swap_rows(&mut self, row1: usize, row2: usize) -> Result<(), &'static str>;

    fn sub_matrix<const NEW_ROWS: usize, const NEW_COLS: usize>(&self, row_start: usize, col_start: usize) -> Result<Matrix<NEW_ROWS, NEW_COLS>, &'static str>;
}

pub trait MatrixConcat<const ROWS: usize, const COLS: usize> {
    ///
    /// Function that concatenates matrices horizontally
    /// 
    fn x_concat<const RHS_COLS: usize, const NEW_COLS: usize>(
        self,
        rhs: Matrix<ROWS, RHS_COLS>,
    ) -> Option<Matrix<ROWS, NEW_COLS>>;

    ///
    /// Function that concatenates matrices vertically
    /// 
    fn y_concat<const RHS_ROWS: usize, const NEW_ROWS: usize>(
        self,
        rhs: Matrix<RHS_ROWS, COLS>
    ) -> Option<Matrix<NEW_ROWS, COLS>>;
}

pub trait SquareMatrix<const N: usize> {
    fn det(&self) -> f64;
    fn inv<const DOUBLE_COLS: usize>(&self) -> Result<Matrix<N, N>, &'static str>;
}

pub trait IsSquareMatrix {}


